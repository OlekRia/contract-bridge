# Research Ledger — Beyond Points (hand-evaluation discovery)

Brief: `contract-bridge/hand-evaluation-discovery.yaml` (v1). Owner: Claude (PI).
Codex: **not connected** this session (see `04-experiment-contracts.md`).

## Reproducibility header (packet 1)

| Item | Value |
|---|---|
| Date | 2026-11-17 |
| Repo / submodule | `5c9cc32` (dirty: docs/*, gov/requirements.yaml R-TOOL-015, new sim modules) / `6e4726a` (untracked: research/, *.yaml prompts) |
| Toolchain | rustc 1.99.1 → **1.100.0 mid-session** (external `rustup update`), dds-bridge 0.18.0 |
| Hardware | 28 threads, 31 GB |
| RNG | `rand::rngs::StdRng::seed_from_u64(seed)`, uniform 52-card shuffle, rejection on auction filter |
| DD | `dds_bridge::Solver::solve_deals`, all 5 strains, batches of 400 |

## Datasets

| Name | Scenario | Seed | N | Role | Path |
|---|---|---|---|---|---|
| nt-dev | EC-1 | 101 | 20 000 | exploration | `data/nt-dev-s101.jsonl` |
| major-dev | EC-2 | 201 | 20 000 | exploration + training | `data/major-dev-s201.jsonl` |
| nt-test | EC-1 | 112 | 40 000 | held-out confirmation (sha `428591275967782f`) | `data/nt-test-s112.jsonl` |
| major-test | EC-2 | 212 | 40 000 | held-out confirmation (sha `4cfbd2249f294abb`) | `data/major-test-s212.jsonl` |
| (reserved) | EC-1/EC-2 | 113 / 213 | — | untouched, packet 2 | — |

Acceptance rates: EC-1 0.0112, EC-2 0.0186 (rejection sampling on uniform deals; no weights).
Compute: ≈ 40-64 deals/s all strains (28 threads); total ≈ 120k generated deals + 72k
Monte-Carlo solves (ceiling) + 4 800 (teaching MC). Wall time ≈ 1.5 h.

## Work items

| ID | Item | Owner | State | Artifact |
|---|---|---|---|---|
| L-01 | Prior-art audit | Claude (subagent) | MEASURED | `01-prior-art.md` |
| L-02 | Anomaly audit (H-8/H-31/H-34, T16, T12 B16) | Claude (subagent) | MEASURED | `02-anomaly-audit.md` |
| L-03 | Hypothesis portfolio | Claude | AVAILABLE | `03-hypothesis-cards.md` |
| L-04 | Experiment contracts + Codex packets | Claude | AVAILABLE | `04-experiment-contracts.md` |
| L-05 | Evaluator/feature library (TDD) | Claude | AVAILABLE | `src/sim/hand_eval.rs`, `src/sim/eval_lab.rs` |
| L-06 | EC-1 dev + confirmation | Claude | MEASURED (P1 confirmed) | `06-results.md` |
| L-07 | EC-2 dev + confirmation | Claude | MEASURED (P2 inconclusive, P3/P4 confirmed) | `06-results.md` |
| L-08 | Kaplan–Rubens (Pavlicek variant) | Claude | AVAILABLE (8 fixtures; variant ambiguity in `01` §3.3) | `hand_eval.rs` |
| L-10 | HE-09 strain transfer | Claude | MEASURED (S7 confirmed) | `06-results.md` |
| L-11 | S2 Monte-Carlo ceiling (EC-2 dev) | Claude | MEASURED (≈ Zar) | `data/ceiling-major-dev-v.*` |
| L-12 | Teaching positions (24, 6 pairs) | Claude | AVAILABLE | `07-teaching-positions.md` |
| L-13 | Human procedure + usability test | Claude / Ori+Urs | PROPOSED (not run) | `08-human-procedure.md` |
| L-14 | Director packet | Claude → Ori | Proposal A RATIFIED (AMD-34/DEC-15); §3 corrections and packet 2 awaiting | `09-director-packet.md` |
| L-15 | Single-dummy validation | Codex | BLOCKED (no SD/lead infrastructure) | `10-next.md` |
| L-09 | Codex challenge packets 1-7 | Codex | PROPOSED | `04-experiment-contracts.md` |
