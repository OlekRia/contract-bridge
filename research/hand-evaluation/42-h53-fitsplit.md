# H-53 — Fit-Quality split: INCONCLUSIVE; the trump term alone does better

H-53 asked whether H-48's Fit-Quality works at the table if its shortness terms (wasted honours, ruffs) are switched on only when the auction is uncontested.
**It doesn't add anything.**
The data instead point to the **trump-quality term alone** (Fit-Quality L1), which was a secondary team in this test.

## Result in one line

- **Fit-Split vs Fermat Complete (primary: All Fives, competing, new seed-11000 deals): +0.006 IMP/board [−0.019, +0.032] → inconclusive.**
- **L1 (trump term only) vs Fermat Complete: +0.024 [+0.001, +0.048].**
- **Fit-Split vs L1: −0.018 [−0.036, −0.000].**

## 1. Method

Runner `eval_lab fitsplit` with the `auction20` roster plus Fermat Fit-Split (H-53) and Fit-Quality L1. L2 was already in the roster.
- **Switching rule:** `sim::hand_eval::fit_split` (TDD).
- **Contested** = the model's interference flag (opponents enter the auction; 68% of boards). With silent opponents nothing is contested, so Fit-Split = L2.
- **Deals:** lines tuned on the 30 000 seed-1000 deals; tested on 30 000 brand-new seed-11000 deals (sha256 [exited …), generated after H-53 was filed. One run. Exploration on seed 6000.

## 2. Results, seed 11000 (IMPs per board vs Fermat Complete, 95% CI)

| Condition | Fit-Split (H-53) | L1: trump term only | L2: all terms in game zone | Fit-Split vs L1 |
|---|---|---|---|---|
| **All Fives, competing (primary)** | **+0.006 [−0.019, +0.032]** | **+0.024 [+0.001, +0.048]** | −0.019 | −0.018 [−0.036, −0.000] |
| All Fives, silent | +0.018 (= L2) | **+0.043 [+0.017, +0.068]** | +0.018 | −0.026 |
| SAYC, competing | +0.006 | +0.018 [−0.007, +0.042] | +0.017 | −0.018 |
| SAYC, silent | +0.026 (= L2) | **+0.033 [+0.007, +0.060]** | +0.026 | −0.009 |

League places:

| League | L1 | Fit-Split | Fermat Complete |
|---|---|---|---|
| All Fives, competing | **1st** | 2nd | 5th |
| All Fives, silent | **1st** | 4th | 5th |
| SAYC, silent | **1st** | 4th | 5th |
| SAYC, competing | 2nd | 5th | 4th |

Split by board type (All Fives, competing):

| | Contested fit boards | Uncontested fit boards |
|---|---|---|
| Fit-Split vs Fermat Complete | +0.040 | −0.059 |
| L1 vs Fermat Complete | +0.066 | +0.008 |

The shortness terms lose even on the uncontested boards, where they are switched on (SAYC: −0.17 against L1).
Seed-6000 exploration (competing): Fit-Split +0.012, L1 +0.012, Fit-Split vs L2 +0.037.

Full tables: `confirm/h53-confirm-s11000-*.md`. The "contested" column shows NaN with silent opponents, because no board is contested.

## 3. Reading

1. **H-53 is inconclusive.** Switching the shortness terms by auction type removes L2's loss under competition, but adds nothing over the trump term alone.
2. **The wasted-honour and ruff terms don't earn their place**, contested or not. In the auction model they cost about 0.02-0.03 IMP/board against L1, and up to 0.17 on uncontested SAYC fit boards.
3. **The trump-quality term is the part of H-48 that survives**: positive in all four leagues, significant in three, 1st in three of four league tables.
   The rule: **+1½ per card of the shorter trump holding, +1½ for a 9-card or longer fit.**
   But it was a secondary team here; its gain over Fermat Complete is small (+0.02 to +0.04), and it was only marginal on seed 6000 (+0.012).

## 4. Next

**H-54 (candidate, not filed): Fermat Complete + the trump-quality term only**, tested as the primary on brand-new deals, one frozen run.
- If corroborated, it is a one-line addition to the memory card, shown by the raise structure (3- vs 4-card support).
- Usability matters: the H-42 kit currently has no trump term.
- The cheapest path is to fold it into the H-42 adoption only if H-54 confirms. Otherwise Fermat Complete stays as is.

H-48 can be closed: its trump term continues as H-54, and its shortness terms are refuted for table use.
