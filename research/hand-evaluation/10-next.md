# Packet 2 — next discriminating questions and Codex handoff

## Codex handoff (self-contained; run before any promotion)

Repository state: see `00-ledger.md`. Run `cargo test --lib hand_eval eval_lab` (green at
handoff), then work through `04-experiment-contracts.md` §"Codex challenge packets":

1. Leakage audit of `src/bin/eval_lab.rs` (`Case.partner` must only feed `DIAG:` rows and
   the second player's own decision in `invite`).
2. Re-derive LTC / NLTC / Zar / Fifths / BUM-RAP / KnR from `01-prior-art.md` §3 independently;
   add Zar's five published worked examples (§3.4) as unit tests.
3. Independent IMP / score checks.
4. Seed disjointness: hash PBNs across dev/test sets.
5. λ sensitivity for SHV (dev only).
6. Counterexample search: 20 test deals where SHV and Zar disagree most; inspect by hand.
7. Challenge the invite result: does the 3M level change outcomes only through DD
   optimism at the 9-trick level? Re-run with outcomes shifted −0.3 tricks (pessimistic).

## Next experiments (Claude)

| ID | Question | Why it discriminates | Needs |
|---|---|---|---|
| N1 | Does the Zar/invite gain survive a single-dummy proxy? | DD optimism could inflate both game and 3M results | lead model + SD play policy (absent) |
| N2 | Minor-suit fits and 4-4 vs 5-3 strain choice with two lenses | tests perception #1 where strain choice is live | new auction filter |
| N3 | Competitive: overcall/double/pass with an offense–defense vector | brief invitation "offense and defense together" | defensive DD outcomes are already in the records |
| N4 | Slam zone after 2/1 GF with control-weighted lens | H-34, T16 B6/B13/B14 | new scenario |
| N5 | Human test of the memory card | distillation loss | Ori + Urs, seed 213 positions |
| N6 | "11 + two tens" NT shortcut vs Fifths | distillation of P1 | no new DDS (EC-1 data) |
| N7 | Reserved confirmation seeds s113 / s213 | replication | 25 min compute |

## Compute plans

| Size | Deals | Wall time (28 threads, all strains) | CI half-width (EC-2, V) |
|---|---|---|---|
| small | 10k | ~4 min | ±0.05 |
| medium | 40k | ~15 min | ±0.03 |
| large | 160k | ~60 min | ±0.015 |
