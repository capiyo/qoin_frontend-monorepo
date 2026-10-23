# Qoin Monorepo

Real-time crypto trading dashboard. pnpm workspaces + Turborepo.

- `apps/web` — Next.js (React, TypeScript, TanStack Query, Tailwind)
- `apps/mobile` — Expo / React Native
- `packages/core` — shared theme tokens, types, and mock market-data engine used by both apps

## Getting started

```bash
pnpm install
pnpm dev:web      # Next.js on http://localhost:3000
pnpm dev:mobile   # Expo dev server
```

## Notes

- Mock data lives in `packages/core/src/mockData.ts` and simulates a WebSocket price feed
  (random-walk candles + live PnL) so both apps run with no backend. Swap `useMarketFeed`'s
  internals for a real WebSocket client to go live — the hook's return shape is the contract.
- Brand tokens (colors, semantics) are centralized in `packages/core/src/theme.ts` and consumed
  by `apps/web/tailwind.config.ts` and `apps/mobile/src/theme/`. Change them in one place.
- `FeedBanner` (web + mobile) surfaces stale/disconnected states prominently, separate from the
  small `StatusBadge` in the header, so degraded data is never silently displayed as current.
- `OrderTicket` (web + mobile) is a buy/sell form with a market/limit toggle and an explicit
  confirm step before "placing" an order. It only updates local state — wire it to a real order
  endpoint when one exists.
- The original brief was pasted with the "Architecture & Charting" section cut off mid-sentence,
  so anything specific to charting/timeframe interaction beyond what's implemented here wasn't
  in what I received — flag it if there's more to the spec and I'll fill it in.


## Qoin market-flow research services

This branch also contains an early backend foundation under `services/`: Rust/Axum is authoritative for event ingestion, state estimation, and deterministic vector decisions; C++/Drogon is a thin gateway; MongoDB persists events and snapshots. Reference models and tests are under `research/` and `tests/`, with contracts and operational notes under `contracts/` and `docs/`.

**Research/paper only.** No live order route is registered. The isolated Binance adapter is not wired to model signals. Native Rust/Drogon builds, MongoDB integration, replay/backtesting, testnet, and profitability remain unverified. See `docs/PROGRESS.md` and `docs/RISK_POLICY.md` before running services.
