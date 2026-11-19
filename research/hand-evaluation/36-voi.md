# Value of information — what should partner's bids tell? (and do game invitations recover it?)

Runner `eval_lab voi <data> <train frac> 5000 [competitive]`; helper `sim::battle::info_extra` (3 unit tests). Fermat Complete in the All Fives auction model (captain + partner's bucket, as in `35-auction20.md`); each row gives the captain **one extra fact about partner's hand** (or removes a control bid), with the lines re-tuned on the 30 000 training deals under that information. Test: the 30 000 new seed-6000 boards. "Share of the information gap" = gain ÷ (perfect exchange − All Fives as it is). Exploratory study. Outputs `confirm/voi-s6000-silent.md`, `confirm/voi-s6000-competitive.md` (+ `.err` for the tuned invitation bands).

| Information | silent: IMPs/board vs All Fives as it is | silent: share of gap | competing: IMPs/board | competing: share |
|---|---|---|---|---|
| All Fives as it is (partner's range only) | +0.000 [+0.000, +0.000] | 0% | +0.000 [+0.000, +0.000] | 0% |
| + support length (3 / 4 / 5+) | +0.024 [+0.009, +0.040] | 9% | +0.001 [-0.014, +0.017] | 2% |
| + side-suit shortness (void / singleton) | +0.042 [+0.023, +0.060] | 17% | +0.021 [+0.002, +0.039] | 6% |
| + support length + shortness | +0.054 [+0.034, +0.074] | 21% | +0.025 [+0.006, +0.044] | 7% |
| + partner's count within 3 points | +0.190 [+0.158, +0.222] | 81% | +0.361 [+0.323, +0.400] | 87% |
| + partner's exact count | +0.217 [+0.184, +0.250] | 94% | +0.402 [+0.363, +0.441] | 99% |
| + exact count + support + shortness | +0.216 [+0.182, +0.249] | 95% | +0.396 [+0.358, +0.435] | 97% |
| − stoppers NOT shown (no stopper check) | -0.056 [-0.070, -0.041] | -25% | -0.059 [-0.079, -0.038] | -15% |
| − keycards / controls NOT shown (no slam gate) | -0.104 [-0.126, -0.082] | -41% | -0.129 [-0.154, -0.103] | -27% |
| + game invitation (accept with the top of the range) | +0.006 [-0.004, +0.016] | 2% | +0.145 [+0.126, +0.164] | 35% |
| Perfect exchange (both hands seen) | +0.236 [+0.201, +0.271] | 100% | +0.411 [+0.371, +0.451] | 100% |

Baseline (All Fives as it is) reproduces Fermat Complete in `35-auction20.md` exactly (−2.363 silent, −2.580 competing IMPs lost vs DD per board).

## Reading

1. **Almost all the value is in how precisely partner's strength is known.** Partner's Fermat count to within 3 points recovers 81% (silent) / 87% (competing) of the whole cost of bidding blind: +0.19 / +0.36 IMP per board. The exact count adds little more (94% / 99%).
2. **Shape facts are worth little once strength is known:** support length +0.02 / +0.00, side shortness +0.04 / +0.02, both +0.05 / +0.03.
3. **The control bids already assumed are valuable:** without stopper showing −0.056 / −0.059; without keycard/control bidding −0.104 / −0.129 (all on slam decisions).
4. **A game invitation (accept with the top of the shown range) recovers almost nothing with silent opponents** (+0.006, band tuned to ½ point): a declined invite plays at the 3-level, and going down there costs about what the information is worth. The competing figure (+0.145) is **an upper bound only** — the competitive model has no 3-level partscore, so a declined invitation was scored like a sign-off (a free look at partner's count); the real cost of competing at the 3-level or having no room to invite is not charged.

## What it means for All Fives

The auction loses value where partner's **range is wide**, not where shape is hidden; invitations near the game line are not the remedy in DD. The next question is *which* All Fives sequences leave partner's count widest — a per-bucket regret map (bucket × spread of partner's count × IMPs lost vs perfect exchange) would point at the opening/response ranges worth narrowing.
