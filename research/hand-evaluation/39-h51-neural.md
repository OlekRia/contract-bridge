# H-51 — Neural hand evaluation

A neural network trained on our double-dummy data and used as an evaluation method.
It plugs into the existing tournament like any other method.

## Result in one line

**With both hands seen, the network is the best evaluator we have ever tested.**
Fermat Neural beats Fermat Complete by +0.13 IMP/board (silent) and +0.08 (competing), and wins all 21 matches.
**At the table, with partner's hand known only through the auction, it loses** (−0.06 to −0.10).

The pre-registered primary test (All Fives auction, competing) is **refuted**.

## 1. The method

| Part | Choice |
|---|---|
| Input | ONE hand, 56 numbers: four suit blocks of 13 card bits (A…2) plus length/13. Trump contracts: trump block first, side suits by length. Notrump: suits by length. |
| Networks | Trump net and notrump net, each 56 → 64 → 32 → 1, ReLU, 5 793 weights |
| Target | value(hand A) + value(hand B) = the partnership's double-dummy tricks (best declarer) in that strain. Each hand's output is a trick contribution, so the networks fit the auction model (my value + partner's expected value) unchanged. |
| Training cases | every side with 18+ HCP. Trump net: each suit with an 8+ card fit, 365 000 cases. Notrump net: 348 000 cases. |
| Training deals | seed 1000 (deals 0-89 999), seed 2000 (70 000), seeds 3000/4000/5000 (30 000 each). Validation: seed-1000 deals 90 000-99 999. |
| Training | Adam, batch 256, 30 epochs, learning rate 1e-3 decaying to 1e-4, init seed 51. Pure Rust (`sim::neural`), no ML library; 2 min 42 s on 28 cores. |
| Weights | `data/neural-h51.json` (sha256 d3b55d42…) |
| Fermat Neural | the network's count inside Fermat Complete's gates. The gates are converted to tricks without tuning: −1 per unstopped suit in notrump, −1 per uncovered side suit for slam, and the keycard gate unchanged. |

Code: `src/sim/neural.rs` (6 tests, including a numerical gradient check), `eval_lab nn-train`, `eval_lab neural`. R-TOOL-017.

## 2. How well it predicts tricks (validation deals, RMSE in tricks)

| Predictor (linear fit on the two hands' sum) | Trump fits | Notrump |
|---|---|---|
| HCP | 1.137 | 1.361 |
| Zar | 1.017 | 1.707 |
| Fermat (fit count / NT count) | 0.998 | 1.337 |
| **Neural (H-51)** | **0.861** | **1.276** |

In trump contracts the network's error is 14% lower than Fermat's.
Train and validation errors stay together (0.848 / 0.861), so the network is not overfitting.

## 3. Tournament, 30 000 brand-new seed-8000 deals

Plan frozen before these deals were read. One run. IMPs per board vs Fermat Complete, 95% CI.

| Condition | Fermat Neural | fit boards | Neural (pure) | Fermat Neural's place |
|---|---|---|---|---|
| Both hands seen, silent | **+0.133 [+0.099, +0.167]** | +0.213 | +0.070 | **1st, 21/21** |
| Both hands seen, competing | **+0.083 [+0.050, +0.117]** | +0.118 | +0.011 | **1st, 21/21** |
| All Fives auction, silent | −0.082 [−0.113, −0.052] | −0.109 | −0.211 | 5th |
| **All Fives auction, competing (primary)** | **−0.097 [−0.126, −0.068]** | −0.143 | −0.230 | 5th |
| SAYC auction, silent | −0.064 [−0.096, −0.032] | −0.124 | −0.205 | 5th |
| SAYC auction, competing | −0.062 [−0.092, −0.033] | −0.109 | −0.190 | 5th |

With both hands seen, Fermat Neural also beats H-48 Fit-Quality (+0.050 silent, +0.037 competing), the previous best.
In the SAYC auction it ties Zar Max (+0.003 competing).
The seed-6000 exploration gave the same picture (+0.108 with both hands seen, −0.072 in the All Fives auction).

Full tables: `confirm/h51-confirm-s8000-*.md`. Training log: `confirm/nn-train-h51.{md,err}`.

## 4. Why it wins with both hands and loses in the auction

In the auction model the captain knows their own hand and only the **average** value of partner's hands that make the same bids.
Those bid ranges are defined in HCP.

- **Fermat's count** stays close to HCP inside a range, so averaging partner's value loses little.
- **The network** values partner's hand largely by its trump length and shape. A 2♥ raise or a 1NT response says almost nothing about that in HCP terms.
- So the network's extra knowledge is exactly the part the auction does not transmit, and replacing it with an average costs more than it gains.

This matches `36-voi.md`: almost all the value of bidding lies in how precisely partner's count is known.

The network is not wrong. It answers a question (both hands' tricks) that the table lets you ask only partly.

## 5. What follows

1. **Fermat Complete stays the table method.** The network's count is not usable through an HCP-ranged auction.
2. **The network is our best double-dummy oracle** for two known hands. Use it as the reference in analysis and post-mortems, and as a ceiling for future hand-designed counts. It finds +0.13 that Fermat Complete misses.
3. **Candidate H-52 (not filed): an auction-aware network.** Train it to predict tricks from *my hand + partner's shown range*, instead of from my hand alone. Its target is then exactly what the table offers, so the information loss is learned rather than suffered.
4. **Candidate: distil the network into a human count.** Probe the learned value of each card by suit length and trump/side, find where it differs from Fermat, and test those differences as hand-designed terms.

## Caveats

- Double-dummy targets and play. The range-information auction model is not bid-by-bid.
- One architecture with no tuning. A larger network might predict tricks better, but that cannot fix the auction-information problem.
