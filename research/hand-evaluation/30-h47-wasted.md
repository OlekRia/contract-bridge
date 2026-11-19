# H-47 — Fermat Complete + wasted honours (inconclusive: the earlier gain did not replicate)

Plan: `05-confirmation-plan.md` Addendum G, frozen before the seed-4000 deals were generated;
one run. Code: `interlock_fit` with only the wasted term (unchanged since H-45), runner
`eval_lab wasted`. Confirmation data: 30 000 new uniformly random deals, seed 4000
(`data/battle-fresh30k-s4000.jsonl`, sha256 prefix 41000842…, 503 s with ddss), after the usual
30 000 seed-1000 training deals (`data/train30k-s1000-plus-fresh30k-s4000.jsonl`, fraction 0.5).
Output: `confirm/h47-wasted-confirm.md` / `.err`.

## Verdict

| Endpoint | Criterion | Seed 4000 (30 000 new boards) | Result |
|---|---|---|---|
| **P-H47** H-47 vs Fermat Complete | mean ≥ +0.015 and CI lower > 0 | **−0.008 [−0.028, +0.013]** IMP/board (fit boards −0.016) | **Inconclusive** — effect not replicated |

| Region | Wasted honours vs Fermat Complete (IMPs/board) |
|---|---|
| seed 1000 (H-45, alone) | +0.020 [+0.007, +0.033] |
| seed 2000 (H-45, alone) | +0.021 [+0.008, +0.034] |
| **seed 4000 (H-47, frozen, alone)** | **−0.008 [−0.028, +0.013]** |

The term was picked *because* it looked best among H-45's ablations; on deals it had never seen
it gives nothing measurable. The honest reading: its true effect is somewhere between about
−0.01 and +0.02 IMP/board — too small to be worth a counting step at the table.

## Where this leaves the search

| Attempt | On deals never used to choose it | Result |
|---|---|---|
| H-45 Fermat Interlock (honour placement) | −0.113 | refuted |
| H-46 partnership shortness | −0.047 | refuted |
| H-47 wasted honours | −0.008 | inconclusive (≈ 0) |
| Ceiling: learned two-hand table (DIAG) | **+0.265** [+0.225, +0.304] | the room is real (fit boards +0.43) |

Three hand-made two-hand terms, each motivated by a reading of the ceiling, have failed. The
ceiling's gain is real and stable (+0.24 to +0.27 in four regions), but its ridge weights were
read without Fermat Complete inside the model. The next step must find the gap the right way:
fit the ceiling on Fermat Complete's **residual** (Fermat value as an input), on old deals, and
test whatever survives on new deals.
