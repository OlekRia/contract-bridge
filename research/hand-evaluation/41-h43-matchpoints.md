# H-43 — Matchpoint lines for Fermat Complete: CORROBORATED

Our events (Winterthur) are matchpointed pairs, but every line in this laboratory was tuned for IMPs.
H-43 re-tunes Fermat Complete's lines to maximise **matchpoints against the field**.

## Result in one line

**MP lines beat Fermat Complete's IMP lines by +0.27 MP-percentage points per board [+0.13, +0.41]** in the primary test (All Fives auction, competing, new seed-10000 deals).
They do it by bidding about 6-10% fewer thin games and fewer grands.
The proposed field-protection term adds nothing (+0.015, n.s.).

## 1. Method

| Part | Choice |
|---|---|
| Event | a pairs event over the 20-method league: every board's result is matchpointed against the 19 other methods (1 per result beaten, ½ per tie) |
| Counts and gates | Fermat Complete's, unchanged (NT count with stopper check, fit count, slam coverage, keycard gate) |
| MP lines | game, slam, grand and NT lines tuned to maximise matchpoints vs the 19-method field |
| MP + field term | separate game and NT lines when the captain's view of the partnership's HCP (own HCP + partner's expected HCP from the auction) is 25+, i.e. when the field is likely in game |
| No look-ahead | lines tuned on odd 5 000-board blocks and applied to even ones, and vice versa (cross-fit), so no board scores itself |
| Deals | 30 000 brand-new seed-10000 deals, generated after the plan was filed (`gov/hypotheses.yaml` H-43); exploration on seed 6000 |

Code: `sim::eval_lab::mp_vs_field` (TDD), `eval_lab mp20`. R-TOOL-019.

## 2. Results, seed 10000

MP-percentage points per board vs Fermat Complete with IMP lines, 95% CI (a session of 24 boards ≈ ×24 MP-% points summed per board):

| Condition | MP lines | fit boards | NT boards | MP + field term | IMP cost of MP lines |
|---|---|---|---|---|---|
| All Fives, silent | **+0.310 [+0.150, +0.470]** | +0.126 | +0.486 | +0.281 | −0.059 |
| **All Fives, competing (primary)** | **+0.273 [+0.134, +0.413]** | +0.204 | +0.340 | +0.288 | −0.018 |
| SAYC, silent | +0.131 [−0.004, +0.265] | +0.116 | +0.144 | +0.137 | −0.055 |
| SAYC, competing | +0.064 [−0.058, +0.186] | +0.041 | +0.086 | +0.058 | −0.034 |

The seed-6000 exploration (All Fives, competing) agreed: +0.267 [+0.101, +0.433].

**Field term vs plain MP lines:** −0.028, +0.015, +0.007 and −0.007. None of these is significant, so the term is not needed.

**Pairs-event standings** (seed 10000, All Fives, competing): Fermat Complete is at 51.1% against the 19 other methods, and its MP-line version at 51.4%. SAYC classic finishes last, as in every IMP event. Full tables: `confirm/h43-confirm-s10000-*.md`.

The per-board figures look small because they are averages over every board, including the many boards where every method reaches the same contract.
The method changes the level on about 6-8% of boards (1 839 of 30 000 competing; 2 402 silent).

## 3. The matchpoint lines (All Fives auction model, Fermat points)

| Line | IMP lines (competing / silent) | MP lines (competing / silent, range over folds) | Change at pairs |
|---|---|---|---|
| Game in a major fit | 51.7 / 52.5-52.9 | 52.1-53.2 / 53.4-53.9 | **+1** (most when vulnerable) |
| 3NT (Fermat NT count) | 24.0 / 24.0-24.3 | 24.3-24.7 / 24.5-24.9 | **+½** |
| Small slam | 61.3 / 61.6 | 60.6-61.3 / 60.5-61.4 | unchanged |
| Grand slam | 65.1 / 65.1 | 66.5-68.2 / 64.5-67.7 | **+2** |

Why: at IMPs a vulnerable game pays when it makes about 37% of the time, because the game bonus outweighs the undertrick. At matchpoints only the ranking counts, so every decision moves towards 50%. That moves the game line up by about a point and the grand line up. The small-slam line hardly moves, because slams near the line are close to 50% either way.

## 4. For the table (Director's decision)

At **pairs**, with Fermat Complete:

- **Game:** bid it at **54** instead of 53. When vulnerable, don't stretch: invite rather than jump on 53.
- **3NT:** with **25** (Fermat NT count).
- **Slam:** unchanged.
- **Grand:** only with about **2 points to spare** over the IMP line.
- **Teams events:** keep the IMP lines.

This would amend the line rows of the Fermat Complete memory card (H-42) and, once Fermat Complete is in the system, the All Fives raise notes.
It is not ratified yet; it needs the usual amendment cascade (`/amend`).

## 5. What H-43 does not cover

- **Strain choice:** notrump vs a major fit, worth +10 per board at matchpoints, is fixed by the model.
- **Overtricks, sacrifices and doubles at matchpoints:** the competitive model scores them as at IMPs.
- **A real club field:** the field is our 19 methods, not Winterthur players. B6 at T17, where the whole field bid game, suggests the real field bids games more readily than our field does. That would favour the MP lines even more, because a pair that stays out of a failing game beats the whole field.
- Double-dummy play and the range-information auction, as everywhere in this laboratory.
