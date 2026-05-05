package blockstoreservice

import (
	"bytes"
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"net/http"
	"time"
)

// OpenAIEmbedder calls the OpenAI embeddings API to produce dense vectors.
// Usable as a drop-in EmbeddingProvider — the Service stores whatever bytes
// the provider returns, so swapping Hash ⇄ OpenAI at boot is transparent.
//
// NOTE: when the embedder's dims differ from the Postgres `vec(D)` column
// dims, the repo transparently falls back to the JSONB path for both write
// and search. Operators who want HNSW under OpenAI must ALTER the column
// to match the chosen model (3-small: 1536, 3-large: 3072).
type OpenAIEmbedder struct {
	apiKey  string
	model   string
	dims    int
	baseURL string
	hc      *http.Client
}

// OpenAIEmbedderConfig carries the tunables for an OpenAI-compatible embedder.
// Compatible endpoints (Azure OpenAI, local proxies, LiteLLM) work by pointing
// BaseURL at the alternate host; the wire shape matches OpenAI's v1 API.
type OpenAIEmbedderConfig struct {
	APIKey  string
	Model   string // e.g. "text-embedding-3-small"
	Dims    int    // dimensionality reported by the model (3-small: 1536)
	BaseURL string // default: https://api.openai.com/v1
	Timeout time.Duration
}

func NewOpenAIEmbedder(cfg OpenAIEmbedderConfig) (*OpenAIEmbedder, error) {
	if cfg.APIKey == "" {
		return nil, errors.New("openai embedder: APIKey is required")
	}
	if cfg.Model == "" {
		cfg.Model = "text-embedding-3-small"
	}
	if cfg.Dims == 0 {
		cfg.Dims = 1536
	}
	if cfg.BaseURL == "" {
		cfg.BaseURL = "https://api.openai.com/v1"
	}
	if cfg.Timeout == 0 {
		cfg.Timeout = 30 * time.Second
	}
	return &OpenAIEmbedder{
		apiKey:  cfg.APIKey,
		model:   cfg.Model,
		dims:    cfg.Dims,
		baseURL: cfg.BaseURL,
		hc:      &http.Client{Timeout: cfg.Timeout},
	}, nil
}

func (e *OpenAIEmbedder) Model() string { return e.model }
func (e *OpenAIEmbedder) Dims() int     { return e.dims }

type openaiEmbedRequest struct {
	Input      string `json:"input"`
	Model      string `json:"model"`
	Dimensions int    `json:"dimensions,omitempty"`
}

type openaiEmbedResponse struct {
	Data []struct {
		Embedding []float32 `json:"embedding"`
	} `json:"data"`
	Error *struct {
		Message string `json:"message"`
		Type    string `json:"type"`
	} `json:"error,omitempty"`
}

// Embed sends one request per call — the service layer already batches ops
// into a worker goroutine, so request-per-text is acceptable and keeps the
// retry story simple. Switch to the batch endpoint when p99 embed latency
// becomes the bottleneck.
func (e *OpenAIEmbedder) Embed(ctx context.Context, text string) ([]float32, error) {
	reqBody := openaiEmbedRequest{Input: text, Model: e.model}
	// 3-small / 3-large support a `dimensions` reduction parameter; pass it
	// when the caller configured a non-default size to keep stored vectors
	// aligned with the DB column width.
	if e.dims != 0 {
		reqBody.Dimensions = e.dims
	}
	body, _ := json.Marshal(reqBody)
	req, err := http.NewRequestWithContext(ctx, "POST", e.baseURL+"/embeddings", bytes.NewReader(body))
	if err != nil {
		return nil, err
	}
	req.Header.Set("Authorization", "Bearer "+e.apiKey)
	req.Header.Set("Content-Type", "application/json")

	resp, err := e.hc.Do(req)
	if err != nil {
		return nil, fmt.Errorf("openai embed: %w", err)
	}
	defer resp.Body.Close()

	raw, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, err
	}
	var out openaiEmbedResponse
	if err := json.Unmarshal(raw, &out); err != nil {
		return nil, fmt.Errorf("openai embed: decode: %w (body=%s)", err, string(raw))
	}
	if out.Error != nil {
		return nil, fmt.Errorf("openai embed: %s (%s)", out.Error.Message, out.Error.Type)
	}
	if resp.StatusCode >= 400 {
		return nil, fmt.Errorf("openai embed: HTTP %d: %s", resp.StatusCode, string(raw))
	}
	if len(out.Data) == 0 {
		return nil, errors.New("openai embed: empty data")
	}
	vec := out.Data[0].Embedding
	if len(vec) != e.dims {
		// The API is authoritative — trust its length, update dims so the
		// Dims() accessor stays consistent with what we've actually stored.
		e.dims = len(vec)
	}
	return vec, nil
}
