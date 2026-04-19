package blockstoreservice

import (
	"context"
	"crypto/sha256"
	"encoding/binary"
	"encoding/hex"
	"math"
	"strings"
)

// EmbeddingProvider turns a block's text summary into a fixed-dim float vector.
// Production deployments plug in an API-backed implementation (OpenAI, Voyage,
// a local sentence-transformers server, ...). Tests and air-gapped setups use
// HashEmbedder which is deterministic and requires no network.
type EmbeddingProvider interface {
	Model() string
	Dims() int
	Embed(ctx context.Context, text string) ([]float32, error)
}

// HashEmbedder is a bag-of-words hashing vectorizer. It tokenizes on whitespace
// + punctuation, hashes each token into one of D buckets, L2-normalizes the
// result, and returns the vector. Same input → same output across processes.
//
// Quality is obviously below a real language model, but semantically related
// documents that share vocabulary land in the same direction of vector space,
// which is enough to demonstrate the pipeline end-to-end and to support
// deterministic tests. Swap for a real embedder in prod.
type HashEmbedder struct {
	dims int
}

func NewHashEmbedder(dims int) *HashEmbedder {
	if dims <= 0 {
		dims = 256
	}
	return &HashEmbedder{dims: dims}
}

func (h *HashEmbedder) Model() string { return "hash-bow-v1" }
func (h *HashEmbedder) Dims() int     { return h.dims }

func (h *HashEmbedder) Embed(_ context.Context, text string) ([]float32, error) {
	vec := make([]float32, h.dims)
	for _, tok := range tokenize(text) {
		sum := sha256.Sum256([]byte(tok))
		idx := binary.BigEndian.Uint32(sum[:4]) % uint32(h.dims)
		sign := float32(1)
		if sum[4]&1 == 1 {
			sign = -1
		}
		vec[idx] += sign
	}
	return l2Normalize(vec), nil
}

// tokenize splits text into lowercase alphanumeric tokens, dropping the rest.
// A whitespace + punctuation split is sufficient for a bag-of-words model.
func tokenize(s string) []string {
	s = strings.ToLower(s)
	out := make([]string, 0, 8)
	var buf strings.Builder
	flush := func() {
		if buf.Len() > 0 {
			out = append(out, buf.String())
			buf.Reset()
		}
	}
	for _, r := range s {
		if (r >= 'a' && r <= 'z') || (r >= '0' && r <= '9') {
			buf.WriteRune(r)
			continue
		}
		flush()
	}
	flush()
	return out
}

// l2Normalize scales the vector so its L2 norm is 1; returns zero vector as-is
// to avoid NaN. Cosine similarity on normalized vectors reduces to dot product.
func l2Normalize(v []float32) []float32 {
	var sq float64
	for _, x := range v {
		sq += float64(x) * float64(x)
	}
	if sq == 0 {
		return v
	}
	inv := float32(1.0 / math.Sqrt(sq))
	for i := range v {
		v[i] *= inv
	}
	return v
}

// CosineSimilarity returns dot(a,b) / (||a|| ||b||). Used by SemanticSearch to
// rank candidate embeddings against the query vector. Vectors produced by this
// package's embedders are already normalized, so callers typically get cosine
// via a plain dot product — but this helper keeps the general case correct.
func CosineSimilarity(a, b []float32) float32 {
	if len(a) != len(b) || len(a) == 0 {
		return 0
	}
	var dot, na, nb float64
	for i := range a {
		dot += float64(a[i]) * float64(b[i])
		na += float64(a[i]) * float64(a[i])
		nb += float64(b[i]) * float64(b[i])
	}
	if na == 0 || nb == 0 {
		return 0
	}
	return float32(dot / (math.Sqrt(na) * math.Sqrt(nb)))
}

// HashTextForEmbedding returns a short stable fingerprint of the input so the
// service can skip re-embedding blocks whose text hasn't changed. 16 hex chars
// of sha256 is plenty for collision-free dedupe per-block.
func HashTextForEmbedding(text string) string {
	sum := sha256.Sum256([]byte(text))
	return hex.EncodeToString(sum[:8])
}
