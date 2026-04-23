package blockstoreinfra

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"strings"

	"github.com/anthropics/agentsmesh/backend/internal/domain/blockstore"
	"github.com/google/uuid"
	"gorm.io/gorm"
)

// detectPgvector probes column existence + dim width and memoizes the result
// on the Repository. Safe to call on any dialect — non-Postgres short-circuits
// to false so SQLite tests stay on the JSONB path.
func (r *Repository) detectPgvector() (bool, int) {
	r.pgvectorOnce.Do(func() {
		if r.db.Name() != "postgres" {
			return
		}
		var typeName string
		err := r.db.Raw(`
			SELECT format_type(a.atttypid, a.atttypmod)
			  FROM pg_attribute a
			  JOIN pg_class c ON a.attrelid = c.oid
			 WHERE c.relname = 'block_embeddings'
			   AND a.attname = 'vec'
			   AND NOT a.attisdropped
		`).Row().Scan(&typeName)
		if err != nil || typeName == "" {
			return
		}
		if i := strings.Index(typeName, "("); i > 0 {
			rest := typeName[i+1:]
			if j := strings.Index(rest, ")"); j > 0 {
				_, _ = fmt.Sscanf(rest[:j], "%d", &r.pgvectorDims)
			}
		}
		r.pgvectorReady = r.pgvectorDims > 0
	})
	return r.pgvectorReady, r.pgvectorDims
}

// UpsertEmbedding writes the latest embedding for a block, replacing any
// prior row. When the pgvector `vec` column is present we write to both the
// JSONB `vector` (portable, used by reads on downgraded servers) and the
// native `vec` column (indexed, used by HNSW search). On SQLite / pgvector-
// less Postgres only `vector` is written.
func (r *Repository) UpsertEmbedding(
	ctx context.Context,
	blockID uuid.UUID,
	model string,
	dims int,
	vector []float32,
	sourceHash string,
) error {
	raw, err := json.Marshal(vector)
	if err != nil {
		return err
	}
	// Write to pgvector column only when available AND the embedder's dim
	// matches the column width. Mismatch (e.g. OpenAI 1536 vs column 256)
	// silently degrades to JSONB-only so operators can run a mixed fleet
	// during an embedding-model migration.
	usePgvector, colDims := r.detectPgvector()
	if usePgvector && colDims == dims {
		vecLit := formatVectorLiteral(vector)
		return r.db.WithContext(ctx).Exec(`
			INSERT INTO block_embeddings (block_id, model, dims, vector, vec, source_hash, created_at, updated_at)
			VALUES (?, ?, ?, ?, ?::vector, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
			ON CONFLICT (block_id) DO UPDATE SET
				model = excluded.model,
				dims = excluded.dims,
				vector = excluded.vector,
				vec = excluded.vec,
				source_hash = excluded.source_hash,
				updated_at = CURRENT_TIMESTAMP
		`, blockID, model, dims, string(raw), vecLit, sourceHash).Error
	}
	return r.db.WithContext(ctx).Exec(`
		INSERT INTO block_embeddings (block_id, model, dims, vector, source_hash, created_at, updated_at)
		VALUES (?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
		ON CONFLICT (block_id) DO UPDATE SET
			model = excluded.model,
			dims = excluded.dims,
			vector = excluded.vector,
			source_hash = excluded.source_hash,
			updated_at = CURRENT_TIMESTAMP
	`, blockID, model, dims, string(raw), sourceHash).Error
}

// DeleteEmbedding removes the embedding for a block (e.g. on block delete
// or model switch). Errors other than not-found bubble up.
func (r *Repository) DeleteEmbedding(ctx context.Context, blockID uuid.UUID) error {
	res := r.db.WithContext(ctx).Where("block_id = ?", blockID).Delete(&blockstore.BlockEmbedding{})
	if res.Error != nil && !errors.Is(res.Error, gorm.ErrRecordNotFound) {
		return res.Error
	}
	return nil
}

// formatVectorLiteral serialises a []float32 into the `[1,2,3]` form expected
// by pgvector's text input. Cheaper than creating a driver-level type; also
// makes the SQL readable in query logs.
func formatVectorLiteral(v []float32) string {
	var b strings.Builder
	b.WriteByte('[')
	for i, x := range v {
		if i > 0 {
			b.WriteByte(',')
		}
		_, _ = fmt.Fprintf(&b, "%g", x)
	}
	b.WriteByte(']')
	return b.String()
}
