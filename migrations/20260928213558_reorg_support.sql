-- Add migration script here
ALTER TABLE transactions ADD COLUMN block_hash TEXT;
CREATE TABLE transaction_events (
    id           BIGSERIAL PRIMARY KEY,
    tx_hash      TEXT NOT NULL,
    status       TEXT NOT NULL,
    block_number BIGINT,
    block_hash   TEXT,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE blocks (
    block_number    BIGINT PRIMARY KEY, 
    block_hash      TEXT NOT NULL,
    parent_hash     TEXT NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);