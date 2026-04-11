CREATE TABLE resource_grants (
    id              BIGSERIAL PRIMARY KEY,
    organization_id BIGINT NOT NULL REFERENCES organizations(id),
    resource_type   VARCHAR(32) NOT NULL,
    resource_id     VARCHAR(64) NOT NULL,
    user_id         BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    granted_by      BIGINT NOT NULL REFERENCES users(id),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(resource_type, resource_id, user_id)
);

CREATE INDEX idx_resource_grants_resource ON resource_grants(resource_type, resource_id);
CREATE INDEX idx_resource_grants_user ON resource_grants(organization_id, user_id);
