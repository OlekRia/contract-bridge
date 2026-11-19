# H-46 — Partnership Shortness (refuted) — and why the H-45 diagnostic misled

Plan: `05-confirmation-plan.md` Addendum F, frozen before any H-46 code ran and before the
seed-3000 deals existed. Code: `sim::hand_eval::partnership_shortness` (5 unit tests);
runner `eval_lab shortness <data> <train frac> 32`, sharing one core (`fermat_family`) with
`eval_lab interlock` — the refactor reproduces the H-45 outputs byte for byte.

Confirmation data: **30 000 new uniformly random deals, seed 3000**
(`data/battle-fresh30k-s3000.jsonl`, sha256 prefix ed1f52a4…, generated in 535 s with ddss),
appended to the usual 30 000 seed-1000 training deals
(`data/train30k-s1000-plus-fresh30k-s3000.jsonl`, train fraction 0.5).
Outputs: `confirm/h46-shortness-confirm.md` / `.err`; exploratory
`confirm/h46-shortness-explore-s1000.*`, `confirm/h46-shortness-explore-s2000.*`.

## Verdict

| Endpoint | Criterion | Seed 3000 (30 000 new boards) | Result |
|---|---|---|---|
| **P-H46** H-46 vs Fermat Complete | CI lower > +0.03 | **−0.047 [−0.066, −0.027]** IMP/board (fit boards −0.096) | **Refuted** (CI upper < 0) |

The training deals chose the smallest weight on the grid (w = 1) every time, i.e. the best
weight is about zero. The exploratory regions agreed (seed 1000: −0.030; seed 2000: −0.042).

| Team (IMPs/board vs Fermat Complete) | seed 1000 (expl.) | seed 2000 (expl.) | **seed 3000 (confirm)** |
|---|---|---|---|
| H-46 shortness (ruff bonus + mirror penalty) | −0.030 | −0.042 | **−0.047** |
| ruff bonus only | −0.022 | −0.024 | −0.029 |
| mirror penalty only | −0.019 | −0.021 | −0.013 (CI crosses 0) |
| H-46 + wasted honours (H-45 term) | +0.023 | +0.024 | **+0.036 [+0.018, +0.053]** |
| Ceiling, two-hand table (DIAG) | +0.259 | +0.269 | +0.243 |
| All Fives today (Zar / HCP) | −0.184 | −0.164 | −0.192 |

## Why the diagnostic misled (the methodological finding)

H-45 read the ceiling model's ridge weights as "what Fermat Complete is missing". That was
wrong: the ceiling has **no Fermat count inside it**, so its shortness weights describe
shortness relative to a shapeless baseline. Fermat's shape number (2 × longest + second −
shortest) already pays for most of that — adding it again double-counts, exactly as the H-45
fitting/double-fit terms did.

**Correction for the next search:** fit the ceiling on the *residual* of Fermat Complete
(Fermat value as an input, or tricks minus the Fermat prediction as the target). Only weights
that survive with Fermat inside the model point at what Fermat misses.

## The one term that keeps working

The **wasted-honours** term (−1 per K/Q/J opposite partner's singleton; −2 K, −1 Q/J/A
opposite a void) is positive in every region it has been run on: +0.021 (H-45 seed 1000),
+0.021 (H-45 seed 2000) alone, and +0.023 / +0.024 / **+0.036** in combination here, where
the shortness part is roughly neutral to harmful. It was never a pre-registered primary, so it
is a candidate — "Fermat Complete + wasted honours" — for its own frozen test on new deals.

## Limits

Perfect exchange, double-dummy play, silent opponents, fixed strain rule, five-level not
modelled. The ceiling (+0.24 to +0.27) still shows room above Fermat Complete; neither
hand-made two-hand term so far captures more than a fraction of it.
