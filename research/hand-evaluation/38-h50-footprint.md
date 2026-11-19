# H-50 — Zar's footprint gate inside Fermat Complete: REFUTED

Runner: `eval_lab footprint <data> 0.5 5000 <out> [competitive] [sayc]`. Two teams were added to the 20-team roster:
- **ADD:** Fermat Complete with Zar's footprint gate on top of its existing coverage and keycard gates. A small slam needs at most 1 footprint loser; a grand needs 0.
- **REPLACE:** the footprint gate takes the place of the H-41 coverage term (−4 per uncovered side suit).

The gate is `sim::hand_eval::footprint_losers` (TDD, R-TOOL-016). The plan was frozen in `gov/hypotheses.yaml` (H-50) before the seed-7000 deals were generated.

Data:
- **Confirmation set:** 30 000 new random deals, seed 7000. sha256 9e2ff4dd…, 478 s to generate.
- **Training set:** 30 000 seed-1000 deals.
- **Format:** 5 000-board matches; one run.
- **Outputs:** `confirm/h50-confirm-s7000-*.md`. The exploratory runs on the seed-6000 deals are in `confirm/h50-explore-s6000-*.md`.

## Result (IMPs per board vs Fermat Complete, 95% CI)

| Condition | ADD, seed 7000 | fit boards | ADD, seed 6000 (explore) |
|---|---|---|---|
| Perfect exchange | −0.127 [−0.154, −0.101] | −0.261 | −0.125 |
| All Fives, silent | −0.093 [−0.120, −0.066] | −0.191 | −0.102 |
| **All Fives, competing (primary)** | **−0.083 [−0.110, −0.057]** | −0.171 | −0.088 |
| SAYC, silent | −0.086 [−0.113, −0.058] | −0.176 | −0.098 |
| SAYC, competing | −0.069 [−0.095, −0.044] | −0.142 | −0.070 |

REPLACE scores identically to ADD in every condition. The primary CI upper bound is below 0, so **H-50 is refuted**.

## Why

Slam bids on fit boards, seed 7000, All Fives, silent:

| | Slam bids |
|---|---|
| Double-dummy optimal | 1 990 |
| Fermat Complete | 1 295 |
| With the footprint gate | 628 |

Read literally, Zar's rule makes FP > CT cost min(2, FP) losers. That charges two fast losers to ordinary holdings such as Axxx opposite xxx or Kxxx opposite Qxx, so the gate vetoes about half of Fermat Complete's slams, most of them good.

Any side suit with no ace or king and 2+ cards in each hand already counts as 2 footprint losers. So the gate blocks every slam the H-41 coverage term would block, which is why ADD and REPLACE are the same.

In Zar Max (H-49), the gate helped only together with the fit re-evaluation: plain Zar has no slam check at all, and the extra fit points pushed many hands over the slam line. Fermat Complete already filters slams by keycards and coverage, so here the footprint rule only adds false alarms.

## Consequence

Keep Fermat Complete's slam gates unchanged.

A softer footprint count (FP > CT counted as 1 loser) would be a new, post-hoc hypothesis. It is not tested here.
