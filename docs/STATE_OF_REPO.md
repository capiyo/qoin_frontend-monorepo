# State of repository

This branch integrates a first research-engine foundation into `capiyo/qoin_frontend-monorepo`. It adds Axum event/state/vector endpoints, a deterministic geometric baseline, an isolated Binance adapter source, a Drogon proxy, MongoDB development configuration, reference models, tests, and risk/architecture notes.

## Current verified status
The Python reference tests are present in this branch but have not yet been run against this remote branch inside a CI runner. In the earlier audited project snapshot, 8/8 Python tests passed, the JSON schema parsed, OpenAPI YAML parsed, and the smoke script passed shell syntax validation.

## Current limitations
- Native Rust compilation, Rust unit tests, and Drogon compilation are unverified because toolchains were unavailable in the audit environment.
- No lockfile or CI workflow is included yet.
- State is currently held in memory and is not fully reconstructed from MongoDB after restart.
- Model scales, weights, thresholds, and version are hardcoded; there is no versioned config contract consumed by runtime.
- No production market-data adapters, event-driven backtester, walk-forward results, shadow mode, testnet validation, or profitability evidence.
- No live-order route is registered. The gated Binance adapter source is deliberately isolated and not called by the signal path.
- Authentication/authorization, rate limiting, risk engine, protective stops, position/order reconciliation, kill switch, and crash recovery are incomplete.

Keep this system research/paper-only. Do not claim production readiness or profitability.
