# Alcubierre

> *Miguel Alcubierre theorized faster-than-light travel by warping spacetime around a vessel — not by moving through it.*
> *This project does the same for blockchain transactions.*

Alcubierre is a production-grade RPC node infrastructure built in Rust — the layer that sits between a raw consensus engine and the outside world. The same layer that powers every wallet, DEX, and MEV bot you've ever used. Built from scratch, hard problems documented honestly.

---

## What this is

Every blockchain network has two distinct layers:

```
[ Wallets / SDKs / DEXs / MEV bots ]
              ↕ JSON-RPC
         [ Alcubierre ]              ← this project
              ↕
      [ Consensus engine ]
```

The consensus engine produces blocks. It has no friendly interface. No way for a wallet to ask "did my transaction go through?" No historical state. No mempool visibility.

Alcubierre is everything between that engine and the outside world:

- **JSON-RPC server** — the front door. Wallets and SDKs connect here.
- **Transaction lifecycle engine** — full state machine from submission to finality.
- **Block indexer** — streams committed blocks into a queryable database.
- **Mempool stream** — real-time pending transaction feed. This is how MEV bots work.
- **Observability layer** — Prometheus metrics, structured logs, latency tracking.

---

## Why I'm building this

Most blockchain infrastructure content shows the happy path. This project documents the hard parts:

- What happens to your indexer when the chain reorgs
- How backpressure propagates from a slow DB query back to the client connection
- What "finality" actually means and when it's safe to tell a user their transaction succeeded
- How mempool visibility works and why it matters for DeFi execution

Each hard problem ships as a commit and a post on X.

---

## Project scope

```
Phase 1 — JSON-RPC server + transaction lifecycle
Phase 2 — Block indexer + historical queries  
Phase 3 — Mempool stream + latency optimization
Phase 4 — Load testing + real production numbers
```

---

## API surface

### Account
```
getAccountInfo(address)
→ { balance, nonce, chainId }

getTokenBalances(address, chainId)
→ [{ tokenId, symbol, balance }]

getSupportedTokens(chainId)
→ [{ tokenId, symbol, contractAddress }]
```

### Transactions
```
estimateGas(from, to, value, data)
→ { gasLimit, gasPrice }

sendTransaction(rawSignedTxBytes)
→ { txHash }

getTransactionStatus(txHash)
→ { status: PENDING | INCLUDED | FINALIZED | FAILED,
    queuePosition,    -- only when PENDING
    blockNumber,      -- only when INCLUDED or FINALIZED
    gasUsed,          -- only when FINALIZED
    failureReason }   -- only when FAILED

replaceTransaction(originalTxHash, rawReplacementTxBytes)
→ { newTxHash, status: PENDING }
```

### Mempool (Phase 3)
```
subscribeToMempool(filterParams)
→ stream of { txHash, from, to, value, gasPrice }
```

---

## Tech stack

| Layer | Technology |
|---|---|
| RPC server | Rust, Axum, Tokio |
| Database | PostgreSQL, SQLx |
| Cache | Redis |
| Analytics | ClickHouse, BigQuery |
| Observability | Prometheus, Sentry, OpenTelemetry |
| Load testing | k6 |

---

## Hard numbers (updated as I build)

| Metric | Target | Measured |
|---|---|---|
| Throughput | 10k TPS | — |
| P99 latency | < 100ms | — |
| Block → client notification | < 50ms | — |
| Reorg recovery time | < 1s | — |

*Numbers get filled in as each phase ships.*

---

## Build log

| Week | What shipped | Post |
|---|---|---|
| 1 | Project setup, API design | — |

*Updated weekly.*

---

## Running locally

```bash
# coming in phase 1
```

---

## Following along

Built in public on X:@Amanjain812527

Every hard problem becomes a thread. Every phase ships as a tagged release.

---

*Named after Miguel Alcubierre — the physicist who proposed warping spacetime around a vessel rather than moving through it. That's what this layer does for transactions.*
