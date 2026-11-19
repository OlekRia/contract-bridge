# H-52 — Auction-aware neural evaluation: INCONCLUSIVE (ties Fermat Complete)

H-51 trained a network on both hands and lost its edge through the auction.
H-52 trains the network on exactly what the captain knows at the table:
**own hand + strain + partner's shown range** → partnership double-dummy tricks.

## Result in one line

Fermat Auction Neural **ties Fermat Complete** at the table.
In the primary test (All Fives auction, competing) it scored −0.025 IMP/board [−0.058, +0.008], so H-52 is **inconclusive**.
It beats H-51's network by +0.05 to +0.10 in every auction condition: learning from the table's information fixed H-51's failure, but did not surpass the hand-designed count.

## 1. The method

| Part | Choice |
|---|---|
| Input | captain's 13 cards (56 numbers, as H-51) + partner's shown range: a one-hot over the system's bid-range buckets (25 for All Fives, 14 for SAYC, fine and interference-coarsened) + an unknown flag |
| Output | the partnership's double-dummy tricks in its strain (the scored declarer) |
| Networks | one model per system (All Fives, SAYC), each with a trump net and a notrump net: (56 + buckets + 1) → 64 → 32 → 1 ReLU |
| Training cases | every scored side × 4 dealers × (fine bucket, coarse bucket). All Fives 1.90 M cases, SAYC 1.99 M. |
| Training deals | the same as H-51 (seeds 1000/2000/3000/4000/5000), validation on seed-1000 deals 90 000-99 999 |
| Training | Adam, 20 epochs, batch 256, learning rate 1e-3 decaying to 1e-4, seeds 52/53. The shared trainer was refactored, with the H-51 tests green; 4 min for both systems. |
| Teams | **Auction Neural** (network alone, tuned lines). **Fermat Auction Neural**: network count inside Fermat Complete's gates, −1 trick per unstopped suit / uncovered side suit, keycard gate. |
| Weights | `data/neural-h52-allfives.json` (858e873e…), `data/neural-h52-sayc.json` (b0464be1…) |

Code: `sim::neural::{encode_auction, AuctionNet, Case, train_cases}` (3 new tests; 9 in the module). Commands `eval_lab nn-train-auction`, `eval_lab auction-neural`. R-TOOL-018.

## 2. Trick prediction with the same information (validation RMSE, tricks)

| Predictor: captain + partner's shown range | All Fives trump | All Fives NT | SAYC trump | SAYC NT |
|---|---|---|---|---|
| HCP: captain + training mean of partner | 1.225 | 1.543 | 1.241 | 1.514 |
| Fermat: captain + training mean of partner | 1.154 | 1.534 | 1.186 | 1.507 |
| **Auction-aware network** | **1.119** | **1.524** | **1.154** | **1.493** |

The network is better than Fermat by only 0.03 tricks in trump (H-51 with both hands: 0.14).
Validation error rises slightly after the first epochs (SAYC trump 1.146 → 1.154): a mild overfit, left as frozen.

## 3. Tournament, 30 000 brand-new seed-9000 deals

Plan frozen before the deals were read. One run. IMPs per board vs Fermat Complete, 95% CI.

| Condition | Fermat Auction Neural | place | Auction Neural (pure) | vs H-51 Fermat Neural |
|---|---|---|---|---|
| All Fives, silent | −0.017 [−0.048, +0.015] | 5th | −0.113 | +0.052 |
| **All Fives, competing (primary)** | **−0.025 [−0.058, +0.008]** | 5th | −0.118 | +0.064 |
| SAYC, silent | +0.026 [−0.006, +0.059] | 4th | −0.104 | +0.104 |
| SAYC, competing | +0.001 [−0.032, +0.034] | 3rd | −0.104 | +0.079 |

The exploratory seed-6000 runs (competing) gave +0.002 in All Fives and −0.003 in SAYC.
Across both deal sets and both systems the result is consistently **≈ 0 against Fermat Complete**.

Against H-48 Fit-Quality L2 it is level: −0.055 to +0.015, with only All Fives silent clearly negative.

With both hands seen, the auction teams finish near the bottom, as designed: they never look at partner's cards.

Full tables: `confirm/h52-confirm-s9000-*.md`. Training logs: `confirm/nn-train-h52-*.{md,err}`.

## 4. Reading

1. **The H-51 diagnosis was right.** Training on what the table shows recovers +0.05 to +0.10 IMP/board over H-51's network in every auction condition.
2. **But Fermat Complete is already near the ceiling of this information.** The network predicts tricks only 3% better than "Fermat + average partner", and that doesn't turn into IMPs. Most of the remaining loss lies in information the auction does not carry (`36-voi.md`: partner's exact count is worth +0.2 to +0.4 IMP/board), not in how the captain's own hand is counted.
3. **The gates carry real knowledge.** The pure network loses 0.10-0.12 to Fermat Complete, while the same network inside Fermat's stopper, coverage and keycard gates ties it. Gates use information shown by control bids, which the network never sees.
4. **Practical consequence: none for the table.** Fermat Complete stays. Its simple count is as good as a 6 000-weight network given the same information.

## 5. What would move the needle

The data point to the auction itself, not the evaluation:
- **narrower ranges for partner's count** (`36-voi.md`: within 3 points recovers 81-87% of the gap);
- or richer shown information (support length, shortness) where the network could exploit it.

A network over a finer auction model (per-call ranges instead of buckets) is the natural H-53, but it needs a bid-by-bid model first.

## Caveats

- Range-information auction model (buckets), double-dummy play and target.
- One architecture with frozen epochs; the slight overfit was not tuned away.
