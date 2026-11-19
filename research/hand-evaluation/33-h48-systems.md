# H-48 step 2 — Fermat Fit-Quality inside the auction models (primary inconclusive)

Plan: `05-confirmation-plan.md` Addendum H2, frozen before any step-2 code ran. Models: the
H-42 `eval_lab systems` information models (captain knows own hand + partner's count bucket per
system; coverage gates shown by stopper/control bids). Fit-Quality information levels:
**L1** trump term only (support length shown by the raise structure); **L2** trump term + the
wasted/ruff terms only when the regime's Fermat estimate ≥ 50 (game/slam-try zone); **L3** all
known. Outputs: `confirm/h48-systems-s5000-{competitive,silent}.md` (primary data: 30 000
seed-5000 boards) and `confirm/h48-systems-s2000-{competitive,silent}.md` (secondary: 70 000
seed-2000 boards). The added teams leave the H-42 head-to-heads unchanged (checked: Fermat
Complete vs Zar / HCP reproduces `h42-systems-competitive.md` exactly).

## Verdict

| Endpoint | Criterion | Result |
|---|---|---|
| **P-H48b** L2 vs Fermat Complete, All Fives model, competitive, seed 5000 | CI lower > 0 | **−0.019 [−0.047, +0.008]** → **inconclusive** |

## Fit-Quality vs Fermat Complete (IMPs per board, 95% CI) — All Fives model

| Opponents | Level | seed 5000 | seed 2000 |
|---|---|---|---|
| **silent** | L1 trumps shown | **+0.042** [+0.017, +0.067] | **+0.047** [+0.030, +0.063] |
| silent | L2 + shortness in game zone | +0.031 [+0.002, +0.060] | +0.039 [+0.020, +0.058] |
| silent | L3 all known | +0.034 [+0.004, +0.063] | +0.040 [+0.022, +0.059] |
| **competitive** | L1 trumps shown | −0.004 [−0.028, +0.020] | +0.024 [+0.009, +0.039] |
| competitive | **L2 (primary)** | **−0.019 [−0.047, +0.008]** | −0.007 [−0.024, +0.010] |
| competitive | L3 all known | −0.018 [−0.045, +0.009] | −0.005 [−0.023, +0.012] |

Perfect exchange for reference: silent +0.080 / +0.071, competitive +0.056 / +0.044 (L2).

Across systems (silent, L2): SAYC +0.045 / +0.052, Precision +0.045 / +0.049, Acol +0.032 /
+0.043, 2/1 +0.048 / +0.047 — the gain survives every system's information. Competitive, L2:
between −0.045 (Precision) and +0.020 (Acol) — nowhere clearly positive.

## Reading

1. **With silent opponents the gain survives the auction** (+0.03 to +0.05 in every system),
   and **the trump term alone (L1) keeps most of it**: knowing 3- vs 4-card support is enough.
2. **Knowing shortness only in the game zone (L2) costs nothing** against knowing it always (L3).
3. **When opponents compete, the gain disappears.** Even with perfect exchange it shrinks
   (+0.08 → +0.05). In the competitive model, interference coarsens partner's range on 68% of
   boards and many fit boards turn into partscore battles or sacrifices, where a sharper game/slam
   count has little to act on. The competitive model is also the one with known structural biases
   (H-37), so this is a limit of the evidence as much as of the method.
4. Against today's All Fives evaluation (Zar / HCP) Fit-Quality L2 still gains +0.14 / +0.14 IMP
   per board in the competitive All Fives model — but that is Fermat Complete's gain (+0.18 /
   +0.16), not Fit-Quality's.

## Status

H-48: step 1 corroborated (perfect exchange, +0.077); step 2 primary inconclusive. Not ready for
an amendment. Candidate for a narrower claim: the **trump-quality term** (+1½ per card of the
shorter trump holding, +1½ for a ninth trump) in uncontested auctions — +0.04 to +0.05 with
silent opponents in both regions; needs its own frozen test, including a split by contested vs
uncontested boards.
