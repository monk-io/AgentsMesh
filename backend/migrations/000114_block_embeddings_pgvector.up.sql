-- Phase 4+ pgvector upgrade: swap JSONB vector storage for a native vector
-- column with an HNSW index. Makes semantic search scale from linear-scan
-- (thousands) to index-backed ANN (millions).
--
-- IMPORTANT: Requires the pgvector extension. `deploy/*/docker-compose.yml`
-- now uses `pgvector/pgvector:pg16` which bundles it. Self-host operators
-- on vanilla `postgres:16-alpine` will see CREATE EXTENSION fail; Service
-- layer detects this at boot and falls back to JSONB-only mode.

CREATE EXTENSION IF NOT EXISTS vector;

-- Default dim aligns with the service-layer HashEmbedder (256). When a site
-- switches to OpenAI (1536), drop + recreate the column; the service will
-- re-embed all blocks on demand (see embedding_refresh.go).
ALTER TABLE block_embeddings
    ADD COLUMN IF NOT EXISTS vec vector(256);

-- HNSW is the right default for < 10M rows and read-heavy workloads (our
-- case: Agents query more than they write). vector_cosine_ops matches the
-- cosine similarity used by the service layer.
CREATE INDEX IF NOT EXISTS idx_block_embeddings_hnsw
    ON block_embeddings USING hnsw (vec vector_cosine_ops);
