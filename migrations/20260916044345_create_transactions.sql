-- Add migration script here
CREATE TABLE transactions (
    tx_hash      TEXT PRIMARY KEY,
    status       TEXT NOT NULL,
    block_number BIGINT,
    gas_used     BIGINT,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW()
);