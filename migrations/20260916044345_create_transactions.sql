-- Add migration script here
CREATE TABLE transactions (
    tx_hash      TEXT PRIMARY KEY,
    status       TEXT NOT NULL,
    block_number BIGINT,
    gas_used     BIGINT,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE TABLE accounts (
    address     TEXT PRIMARY KEY,
    balance     TEXT NOT NULL,
    nonce       BIGINT NOT NULL DEFAULT 0,
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

INSERT INTO accounts (address, balance, nonce) 
VALUES ('0xabc123', '1000000', 0);
