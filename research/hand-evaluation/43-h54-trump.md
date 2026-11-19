# H-54 — Fermat Complete + trump quality: CORROBORATED

**The one-line rule:** add **+1½ per card of the shorter trump holding, and +1½ more for a 9+ card fit**, to Fermat Complete's fit total.
This is the trump-quality term of H-48, without its wasted-honour and ruff terms.

## Result

Test deals: 30 000 brand-new seed-12000 deals (sha256 [exited …), generated after H-54 was filed, one frozen run (`eval_lab fitsplit`, team "Fit-Quality L1").
Lines were tuned on the 30 000 seed-1000 deals.
Figures are IMPs per board vs Fermat Complete, with 95% CIs.

| Condition | H-54 | Fit boards | League place (of 22) |
|---|---|---|---|
| **All Fives, competing (primary)** | **+0.051 [+0.027, +0.074]** | +0.102 | **1st** |
| All Fives, silent | +0.064 [+0.038, +0.090] | +0.130 | 2nd |
| SAYC, competing | +0.024 [−0.000, +0.049] | +0.049 | 3rd |
| SAYC, silent | +0.063 [+0.036, +0.089] | +0.127 | 1st |

Two further readings:

- **Contested vs uncontested.** All Fives, competing: +0.131 on contested fit boards, +0.029 on uncontested ones. The term helps most when the auction is fought, because it needs only the fit length.
- **H-53 Fit-Split vs H-54:** −0.025 [−0.043, −0.007] (All Fives, competing). Adding back the shortness terms costs again, in line with `42-h53-fitsplit.md`.

Earlier evidence for the same term (not part of this test):

| Run | Silent | Competing |
|---|---|---|
| seed 11000 (H-53 secondary) | +0.043 | +0.024 |
| seed 6000 | — | +0.012 |
| H-48 step 2 | +0.042 / +0.047 | — |

All of them are positive.

**Primary criterion met (CI lower bound > 0): H-54 corroborated.**

## What changes

Per H-54's plan, the term joins the Fermat Complete card **before** ratification (roadmap item 3), after a usability re-check. The work:

1. `sim::fermat_card`: add the trump term to the fit total (TDD).
2. The memory card: one new line under "Major fit".
3. Regenerate the 40 usability positions and solutions from the same seed-213 deals, so the partners test the final card.

**Done the same day.** The card's fit lines were re-derived from the both-hands tuning with the term, rounded up to ½ (the same rule that produced the H-42 card from Fermat Complete's 53.25 / 52.25 / 61.75 / 65.25):

| Line | Teams | Pairs (DEC-16, relative) |
|---|---|---|
| Game, vulnerable | 58½ | 59½ |
| Game, not vulnerable | 59 | 59½ |
| Small slam | 68 | 68 |
| Grand | 71½ | 73½ |

The notrump lines are unchanged. The usability kit was regenerated with these lines.

## Limits

- Double-dummy play and the range-information auction.
- Support length (3 vs 4 cards) is assumed to be shown by the raise structure.

Full tables: `confirm/h54-confirm-s12000-*.md`.
