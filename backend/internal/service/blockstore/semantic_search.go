package blockstoreservice

import (
	"context"
	"sort"
	"strings"
	"time"

	"github.com/anthropics/agentsmesh/backend/internal/domain/blockstore"
	"github.com/anthropics/agentsmesh/backend/internal/infra/otel"
	"github.com/google/uuid"
)

// SearchHit is one ranked result from semantic search. Score is cosine
// similarity in [-1, 1]; higher is more relevant.
type SearchHit struct {
	BlockID uuid.UUID `json:"block_id"`
	Type    string    `json:"type"`
	Snippet string    `json:"snippet"`
	Score   float32   `json:"score"`
}

// SearchInput narrows a semantic search to one workspace. TopK defaults to 10
// when unset; MinScore filters out weak matches (0 keeps everything).
type SearchInput struct {
	WorkspaceID uuid.UUID
	Query       string
	TopK        int
	MinScore    float32
	TypeFilter  string
}

// SemanticSearch embeds the query and delegates ranking to the repository.
// On Postgres with pgvector the repo returns already-ranked rows from HNSW;
// on SQLite / plain Postgres we rank in memory. ACL filtering happens here
// so private blocks never leak into results regardless of the backend.
func (s *Service) SemanticSearch(
	ctx context.Context,
	actor ActorContext,
	in SearchInput,
) ([]SearchHit, error) {
	start := time.Now()
	defer func() {
		otel.BlockstoreSearchDuration.Record(ctx, float64(time.Since(start).Milliseconds()))
	}()
	if s.embedder == nil {
		return nil, blockstore.ErrEmbeddingDisabled
	}
	if strings.TrimSpace(in.Query) == "" {
		return []SearchHit{}, nil
	}
	if err := s.assertSameOrg(ctx, actor, in.WorkspaceID); err != nil {
		return nil, err
	}
	topK := in.TopK
	if topK <= 0 || topK > 100 {
		topK = 10
	}

	qvec, err := s.embedder.Embed(ctx, in.Query)
	if err != nil {
		return nil, err
	}
	// Over-fetch so in-memory ACL + type filters can't underflow the final
	// topK. The pgvector path uses the fetchK as its LIMIT; the JSONB fallback
	// ignores it and returns the full set for service-level ranking.
	fetchK := topK * 3
	if fetchK < 30 {
		fetchK = 30
	}
	rows, err := s.repo.SearchEmbeddings(ctx, in.WorkspaceID, s.embedder.Model(), qvec, fetchK)
	if err != nil {
		return nil, err
	}
	// pgvector path returns rows pre-sorted ASC by cosine distance; we map
	// that to Score DESC. When every row already carries a non-zero Score the
	// ordering is correct — skip the re-sort to save O(k log k) on the hot
	// path. The JSONB fallback leaves Score zero until filterAndScore fills
	// it in and then we do need to sort.
	preRanked := len(rows) > 0
	for _, r := range rows {
		if r.Score == 0 {
			preRanked = false
			break
		}
	}
	hits := filterAndScore(rows, qvec, actor, in)
	if !preRanked {
		sort.Slice(hits, func(i, j int) bool { return hits[i].Score > hits[j].Score })
	}
	if len(hits) > topK {
		hits = hits[:topK]
	}
	return hits, nil
}

// filterAndScore walks each candidate, applies type + ACL filters, and fills
// in a snippet. Rows from a pgvector-backed repo already carry Score; others
// get scored here. Kept pure so tests can exercise ranking independently.
func filterAndScore(
	rows []blockstore.EmbeddingRow,
	qvec []float32,
	actor ActorContext,
	in SearchInput,
) []SearchHit {
	out := make([]SearchHit, 0, len(rows))
	for _, row := range rows {
		if in.TypeFilter != "" && row.Type != in.TypeFilter {
			continue
		}
		score := row.Score
		if score == 0 {
			score = CosineSimilarity(qvec, row.Vector)
		}
		if score < in.MinScore {
			continue
		}
		if !extractACL(row.Meta).allows(actor.UserID, row.CreatedBy) {
			continue
		}
		out = append(out, SearchHit{
			BlockID: row.BlockID,
			Type:    row.Type,
			Snippet: snippet(row.Text, 160),
			Score:   score,
		})
	}
	return out
}

// snippet truncates on a word boundary near the max length and appends an
// ellipsis when the original was longer. Used for result previews; callers
// that need full text can re-fetch the block by id.
func snippet(text *string, max int) string {
	if text == nil {
		return ""
	}
	t := strings.TrimSpace(*text)
	if len(t) <= max {
		return t
	}
	cut := t[:max]
	if i := strings.LastIndexAny(cut, " \n\t"); i > 0 {
		cut = cut[:i]
	}
	return cut + "…"
}
