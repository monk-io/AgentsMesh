package blockstoreservice

import (
	"context"
	"log/slog"
	"sync"
	"sync/atomic"
	"time"

	"github.com/anthropics/agentsmesh/backend/internal/domain/blockstore"
	blockstoreinfra "github.com/anthropics/agentsmesh/backend/internal/infra/blockstore"
	"github.com/anthropics/agentsmesh/backend/internal/infra/otel"
)

// Service is the Block Store facade. It accepts semantic ops, validates them
// against block-type specs, applies them atomically inside a workspace-scoped
// advisory-locked transaction, and emits one BlockOp row per primitive change.
type Service struct {
	repo      blockstore.Repository
	publisher *blockstoreinfra.OpPublisher
	embedder  EmbeddingProvider
	logger    *slog.Logger

	// Embedding pipeline: writes land in a buffered channel and one worker
	// drains it. Non-blocking: if the queue is full the op is dropped with a
	// warning — embeddings are an auxiliary index and stale-for-a-few-seconds
	// is acceptable under load.
	//
	// closed is flipped atomically to 1 by Close() so late-arriving
	// enqueueEmbeddings calls short-circuit instead of silently leaking into
	// embedInflight (no worker will ever Done() them post-Close).
	embedQueue    chan embedJob
	embedWG       sync.WaitGroup
	embedInflight sync.WaitGroup
	embedCancel   context.CancelFunc
	embedClosed   atomic.Bool

	// warnDefaultEmbedderOnce guards the startup warning emitted when the
	// service is still on HashEmbedder at first traffic. See WarnIfDefaultEmbedder.
	warnDefaultEmbedderOnce sync.Once
}

type embedJob struct {
	ops []*blockstore.BlockOp
}

func NewService(repo blockstore.Repository, logger *slog.Logger) *Service {
	if logger == nil {
		logger = slog.Default()
	}
	s := &Service{
		repo:       repo,
		embedder:   NewHashEmbedder(256),
		logger:     logger,
		embedQueue: make(chan embedJob, 256),
	}
	s.startEmbedWorker()
	return s
}

// startEmbedWorker launches the background drain loop. Kept unexported — the
// worker lifecycle is bound to the Service; callers shut it down via Close.
func (s *Service) startEmbedWorker() {
	ctx, cancel := context.WithCancel(context.Background())
	s.embedCancel = cancel
	s.embedWG.Add(1)
	go func() {
		defer s.embedWG.Done()
		for {
			select {
			case <-ctx.Done():
				return
			case job, ok := <-s.embedQueue:
				if !ok {
					return
				}
				if s.embedder != nil {
					otel.BlockstoreEmbedQueue.Add(ctx, -1)
					embedStart := time.Now()
					s.refreshEmbeddings(ctx, job.ops)
					otel.BlockstoreEmbedDuration.Record(ctx,
						float64(time.Since(embedStart).Milliseconds()))
				}
				s.embedInflight.Done()
			}
		}
	}()
}

// Close drains any in-flight embedding work and stops the worker. Called by
// main.go on graceful shutdown. Safe to call more than once.
func (s *Service) Close() {
	if !s.embedClosed.CompareAndSwap(false, true) {
		return
	}
	if s.embedCancel != nil {
		s.embedCancel()
	}
	s.embedWG.Wait()
}

// enqueueEmbeddings hands a finished op batch to the background worker. Drops
// the job non-blockingly if the queue is saturated so ApplyOps never stalls.
// After Close() it is a no-op — without this guard, Add(1) on a stopped
// worker would leak an inflight count that FlushEmbeddings can never drain.
func (s *Service) enqueueEmbeddings(ops []*blockstore.BlockOp) {
	if s.embedder == nil || len(ops) == 0 || s.embedClosed.Load() {
		return
	}
	s.embedInflight.Add(1)
	select {
	case s.embedQueue <- embedJob{ops: ops}:
		otel.BlockstoreEmbedQueue.Add(context.Background(), 1)
	default:
		s.embedInflight.Done()
		s.logger.Warn("blockstore.embedding.queue_full",
			"dropped_ops", len(ops))
	}
}

// FlushEmbeddings blocks until every enqueued embed job has finished. Used by
// integration tests that call ApplyOps then immediately SemanticSearch; not
// intended for production code paths.
func (s *Service) FlushEmbeddings() {
	s.embedInflight.Wait()
}

// SetPublisher wires the EventBus publisher. Called by main.go after the bus
// is constructed. Passing nil disables external op broadcasting but keeps all
// writes working.
func (s *Service) SetPublisher(p *blockstoreinfra.OpPublisher) {
	s.publisher = p
}

// SetEmbedder swaps the default HashEmbedder for a production embedding
// provider (e.g. OpenAI, Voyage). Passing nil disables auto-embedding.
func (s *Service) SetEmbedder(e EmbeddingProvider) {
	s.embedder = e
}

// WarnIfDefaultEmbedder emits a one-shot warning when the service is still
// using the bootstrap HashEmbedder (which produces low-quality vectors and
// should never be the final choice in production). Callers invoke this
// after all wiring is complete (end of server startup). Silent by design
// if SetEmbedder has already installed a real provider. The warning surface
// is explicit so operators don't quietly run semantic search on bag-of-words
// vectors and wonder why results look random.
func (s *Service) WarnIfDefaultEmbedder() {
	s.warnDefaultEmbedderOnce.Do(func() {
		if s.embedder == nil {
			return
		}
		if s.embedder.Model() == "hash-bow-v1" {
			s.logger.Warn("blockstore.embedder.default_in_use",
				"model", s.embedder.Model(),
				"note", "HashEmbedder is a dev fallback. Call service.SetEmbedder(openaiEmbedder) before serving traffic in production.")
		}
	})
}
