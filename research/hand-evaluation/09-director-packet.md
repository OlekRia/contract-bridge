# Director Review Packet — hand evaluation, packet 1

For: Ori (Director). From: Claude (PI). Date: 2026-11-17.
Nothing here changes a table agreement. Each item below needs your decision.

## 1. Scoped superiority statement

> **Task**: opener's game decision after an uncontested 1♥/1♠ – 2♥/2♠ in All Fives
> (opener 12-19, not 12-14 balanced; single raise 6-9 HCP with 3+ trumps).
> **Information**: opener's own 13 cards + the auction (responder's own cards for the
> acceptance decision of an invitation). **Outcome**: double-dummy tricks (clairvoyant
> play, no opening-lead model). **Scoring**: IMPs, head-to-head against the named rule
> on the same deal. **Sample**: 40 000 held-out uniformly generated deals (seed 212),
> rules tuned on 20 000 separate deals (seed 201), endpoints frozen in advance
> (`05-confirmation-plan.md`).
>
> | Comparison (vulnerable) | IMP/qualifying deal | 95% CI |
> |---|---|---|
> | Zar vs classic LTC (our current raise note) | **+0.394** | [+0.352, +0.436] |
> | Zar vs HCP | +0.429 | [+0.397, +0.461] |
> | Classic LTC vs HCP (tests F-LTC-004) | +0.035 | [−0.009, +0.079] |
> | Holding table (SHV) vs Zar | +0.069 | [+0.043, +0.096] |
> | Zar invite (3M) vs Zar direct | +0.100 | [+0.072, +0.128] |
> | SHV invite vs Zar direct | +0.228 | [+0.198, +0.258] |
>
> Non-vulnerable: same ordering, about 25-35% smaller (`06-results.md` §2).
> **Limitations**: DD proxy (single-dummy play and opening leads not modelled);
> opponents assumed silent; game decision only (no slam, no partscore competition);
> 1st/2nd seat filter; simulated deals, not table evidence. The auction occurs on
> ≈ 3.7% of boards for our pair, so **+0.39 IMP/qualifying deal ≈ +0.015 IMP per random
> board** (≈ 0.35 IMP per 24-board session) for the LTC→Zar change alone.

What is **not** claimed: global superiority; anything about NT, minors, competition or
slam; any human-performance gain (no usability test has been run).

## 2. Proposed adoption (minimal) — needs your approval

**Proposal A — replace the classic-LTC raise note with Zar for game decisions after a
single major raise.** → **RATIFIED 2026-11-17** by the Director ("ratify now, track at
table"): AMD-34, DEC-15, H-36, R-BID-049, F-ZAR-001, F-SIM-007, S-JUDGE-EVAL-4.
- CURRENT AGREEMENT: `contract-bridge/all-fives.yaml: openings.1_suit.raise_note` =
  "After fit, use LTC: 24 - combined losers = tricks (H-8)".
- PROPOSED: "After 1M–2M, opener counts Zar (HCP + controls (A=2, K=1) + lengths of the
  two longest suits + (longest − shortest)). Vulnerable: 33+ bid game, 31-32 invite with
  3M, 30 or less pass; responder accepts with Zar 20+. Non-vulnerable: 34+ game, 32-33
  invite, responder accepts with Zar 19+. Without an invitation agreement: 32+ game."
  _These integer thresholds are the tuned DD values (V: invite 30.5-32.5, accept 19.5;
  NV: invite 31.5-33.5, accept 18.5; direct 31.5 both) and should be re-checked in the
  prospective pilot before any rounding change._
  _How responder counts Zar for acceptance is the same formula on responder's hand
  (the tuned policy used exactly that)._
- Cascade (if approved):
  - `gov/hypotheses.yaml`: H-8 → record the measured refutation of LTC's advantage in
    this scope; file a new H-ID for "Zar fit lens" with the pilot as its protocol.
  - `gov/facts.yaml`: F-LTC-004 — weaken to "not supported for the 1M–2M game decision
    (DD, 40k deals): LTC ≈ HCP"; add a model fact for Zar in this scope (class: model).
  - `gov/requirements.yaml`: new R-BID for the raise-continuation evaluation rule.
  - `all-fives.yaml` + `all-fives-verbose.yaml` (+ generated md/pdf): replace raise_note.
  - Constitution: no change.

**Proposal B — Fifths for responder's 1NT game decision, vulnerable only (optional).**
Confirmed held-out: Fifths vs HCP +0.043 [+0.026, +0.060] IMP/qualifying deal vulnerable,
+0.006 (null) non-vulnerable; ≈ +0.001 IMP per random board. This is prior art (Andrews)
replicated, not a discovery. Recommendation: teach it as judgment ("tens count, 11-counts
with intermediates bid 3NT vulnerable"), no agreement change. No learned table beat it.

**Proposals C, D, E (added after the full scorecard, `12-scorecard.md` §8)** — awaiting
decision: C) decide 3NT after 1m–3m with HCP 15+, never with Zar/LTC (P6: +0.99
IMP/decision); D) slam-investigation trigger with a major fit = Zar total 61+ or NLTC
total ≤ 13, re-tested with keycard checking first (P7: +1.05 vs HCP total; classic LTC
worse than HCP); E) retire classic LTC as a decision rule, NLTC where a loser count is
wanted.

## 3. Conflicts and source corrections found (separate from any new treatment)

From `02-anomaly-audit.md` (each is a SOURCE ERROR / GAP, not a system change):
- T12 B16 (H-31's founding board): DD is 9 tricks in ♠, not 10; ♠Q is West's; partner had
  2 HCP, not 9. H-31's premise ("missed game") does not hold.
- H-34's T16 B14 account: "two trump losers unless doubleton AK onside" is wrong.
- AMD-31 is cited three times but does not exist; `analysis/2026-09-08.md` was never committed.
- Two loser-count conventions coexist (tool: Qxx = 2 losers; 2NT LTC ask: Qxx = 3).
- `all-fives-verbose.yaml:435` example is 9 HCP, not 10.

## 4. Prospective pilot (for your review)

- Duration: 6 sessions (≈ 150 boards); expected qualifying 1M–2M auctions: ≈ 5-6.
  **This is far too few to confirm a 0.4 IMP effect at the table** (SD per qualifying
  board ≈ 4-5 IMPs). The pilot's purpose is usability (speed, recall, agreement), not
  proof; table confirmation needs ≈ 400+ qualifying boards.
- Record per qualifying board: both players' Zar, LTC and HCP counts written after the
  board, the call made, the result, and the S2 Monte-Carlo answer computed afterwards.
- Stop rule: abandon if either player reports the count takes > 20 s at the table or if
  the partnership disagrees on the count in > 1 of 5 hands.

## 5. Decisions requested

1. Approve, amend or reject Proposal A (and whether to pilot it first).
2. Approve the source corrections in §3 as a separate housekeeping amendment.
3. Approve packet 2 scope (`10-next.md`): Codex challenge packets, NT confirmation,
   single-dummy validation, competitive auctions, slam zone.
