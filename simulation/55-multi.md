# Defense to 5-5 Openings — Simulation Results

**Date**: 2026-02-26
**Engine**: bridge-tools simulate crash (DDS double-dummy, dds-bridge 0.11)
**Hypothesis**: H-7 / AMD-5 (5-5 defense effectiveness)
**Sample**: 9,996 deals (4,998 per opener type), seed=42, all vulnerabilities
**Opener model**: East opens 2H (5H + 5-card minor) or 2S (5S + 5-card minor), 5-10 HCP

## Defense Specification (AMD-5)

Standard weak-two defense of the bid major:

| Bid | Meaning |
|-----|---------|
| X | Takeout of the bid major. Lebensohl applies. |
| 2NT | 16-18 HCP balanced (systems on) |
| New suit | Natural, 5+ suit, 10+ HCP, LTC-gated by vulnerability |
| Cuebid | Game-forcing, stopper ask |
| Pass | Default |

**Note:** Opener has a 5-card minor — bid minors cautiously.

**LTC gates** (added v2): all overcalls and 12-16 takeout doubles are gated by
Losing Trick Count adjusted for vulnerability (Fav ≤ 9, Equal ≤ 8, Unfav ≤ 7).
17+ takeout doubles and 2NT are not gated (strong hands almost always qualify).

## Results Summary

| vs Opener | N | Act% | MP% | Avg Gain | When Acting: Tops | When Acting: Bottoms |
|-----------|---|------|-----|----------|-------------------|----------------------|
| **5-5 2H** | 4,998 | 43.7% | **45.9%** | **-12** | 38.9% | 57.5% |
| **5-5 2S** | 4,998 | 41.5% | **46.7%** | **+0** | 41.8% | 57.5% |

## Detailed Results: vs 5-5 2H (5 Hearts + 5-card Minor)

### When We Act (2,183 boards)

| Metric | Value |
|--------|-------|
| Tops (par > pass) | 38.9% |
| Bottoms (par < pass) | 57.5% |
| Neutral | 3.6% |
| MP% when acting | 40.7% |
| Avg gain when acting | -26 points |

### Score Distribution

| Outcome | Avg | Median | Range |
|---------|-----|--------|-------|
| Gains | +834 | +730 | +10 to +4,440 |
| Losses | -611 | -540 | -2,220 to -10 |

### Action Breakdown

| Action | Count | AvgTricks | Game% |
|--------|-------|-----------|-------|
| Takeout X | 922 | 10.0 | 62.1% |
| 2NT (16-18 balanced) | 210 | 9.4 | 51.4% |
| Natural S | 519 | 9.3 | 46.2% |
| Natural D | 288 | 8.8 | 31.6% |
| Natural C | 244 | 9.4 | 47.1% |
| Pass | 2,815 | — | — |

### By Vulnerability

| Vulnerability | Acts | MP% | Avg Gain | Tops% |
|---------------|------|-----|----------|-------|
| Favorable | 759 | 46.3% | +13 | 18.3% |
| Equal | 751 | 45.7% | -22 | 17.4% |
| Unfavorable | 673 | 45.8% | -26 | 15.3% |

## Detailed Results: vs 5-5 2S (5 Spades + 5-card Minor)

### When We Act (2,075 boards)

| Metric | Value |
|--------|-------|
| Tops (par > pass) | 41.8% |
| Bottoms (par < pass) | 57.5% |
| Neutral | 0.7% |
| MP% when acting | 42.2% |
| Avg gain when acting | +0 points |

### Score Distribution

| Outcome | Avg | Median | Range |
|---------|-----|--------|-------|
| Gains | +845 | +820 | +10 to +4,420 |
| Losses | -614 | -540 | -2,220 to -10 |

### Action Breakdown

| Action | Count | AvgTricks | Game% |
|--------|-------|-----------|-------|
| Takeout X | 966 | 9.8 | 58.1% |
| 2NT (16-18 balanced) | 150 | 9.9 | 58.0% |
| Natural H | 451 | 9.4 | 47.2% |
| Natural D | 282 | 9.2 | 46.8% |
| Natural C | 226 | 9.3 | 45.6% |
| Pass | 2,923 | — | — |

### By Vulnerability

| Vulnerability | Acts | MP% | Avg Gain | Tops% |
|---------------|------|-----|----------|-------|
| Favorable | 723 | 47.0% | +26 | 18.5% |
| Equal | 717 | 46.8% | -5 | 18.2% |
| Unfavorable | 635 | 46.4% | -21 | 15.4% |

## Analysis

### LTC Gates: Correct but Modest Impact

The v2 fix adds vulnerability-adjusted LTC gates to natural overcalls and 12-16
takeout doubles, aligning `classify_action_vs_55` with the CC specification. Before
this fix, vulnerability was ignored entirely (`_vul` parameter unused).

**Before vs After:**

| Metric | 2H v1 | 2H v2 | 2S v1 | 2S v2 |
|--------|-------|-------|-------|-------|
| Act% | 45.6% | 43.7% | 43.4% | 41.5% |
| MP% | 46.0% | 45.9% | 46.9% | 46.7% |
| Avg Gain | -6 | -12 | +7 | +0 |

The LTC gate correctly reduces action rates at unfavorable vulnerability (Unfav acts
dropped from 759→673 for 2H, 723→635 for 2S). But overall MP% barely changed because
the filtered hands (10 HCP with LTC 8-9) weren't systematically the worst performers.
Against preemptive openings, partner's holding matters more than declarer's LTC alone.

### The Defense Is Structurally Sound

Both 5-5 scenarios show near break-even MP% (45.9% and 46.7%). Key observations:

1. **Takeout double is the strongest action.** 62.1% game rate vs 2H, 58.1% vs 2S.
   When we have the right shape (short in their major, support for unbid suits),
   the takeout double consistently finds game.

2. **2NT (16-18 balanced) is highly constructive.** 51-58% game rate. These hands
   reach the right contract reliably.

3. **Natural overcalls are the weakest link.** 31-47% game rate, contributing most
   bottoms. Natural overcalls at the 10+ HCP level are too frequent against
   preemptive openings. The 5-card suit may be outgunned by opener's 10-card fit.

4. **Favorable vulnerability improves results.** Both scenarios show positive avg
   gain at favorable (+13 and +26). The risk/reward balance is better when we're
   not vulnerable.

### 2H vs 2S Asymmetry

Defense against 5-5 2S (46.7% MP, +0 avg) outperforms defense against 5-5 2H
(45.9% MP, -12 avg). This is because:
- When they open 2S, we can overcall 2-level hearts (cheap). When they open 2H,
  we must bid 2S or higher (more committed).
- Spade overcalls require committing at a higher level, increasing risk.

### Critical Model Limitation

Same caveat as all preemptive simulations: the matchpoint model assumes **the field
passes**. Against 5-5 openings, we hold the HCP majority (20-25 combined). The
field also wants to compete. The simulation measures "structured defense vs passing"
but the real comparison is "structured defense vs ad hoc defense."

**Real-table results will be better than the simulation suggests** because:
- Lebensohl gives us structured weak/invitational/GF channels where the field guesses
- The takeout double provides accurate hand description
- 2NT (16-18 balanced) is precise and rare in the field's toolkit

## Conclusions

1. **Use standard takeout defense as ratified (AMD-5).** Near break-even in
   simulation, with significant model limitations for preemptive contexts.

2. **Takeout double is the key action.** High game rate, good structural fit.
   Prioritize doubling with 12+ HCP and ≤2 cards in their major.

3. **2NT (16-18 balanced) is excellent.** Always bid 2NT when the hand qualifies.

4. **LTC gates are now correct but insufficient.** The LTC filter reduced action
   rates (especially at unfavorable) but the remaining natural overcalls still
   produce ~57% bottoms. Further improvement options:
   - Raise minimum HCP from 10 to 12
   - Require 6-card suit for 2-level overcall
   - Only overcall with good suit quality (2 of top 3 honours)

5. **Track real-table results under H-7.** Table evidence will be more informative
   for preemptive contexts than DD simulation.

6. **Opener's 5-card minor is important context.** When advancer chooses between
   minors after partner's takeout double, prefer the longer one — opener's second
   suit (a minor) could be sitting over us.

## Reproducibility

```bash
cargo run --release -- simulate crash -n 5000 --seed 42 --opener 55-2h
cargo run --release -- simulate crash -n 5000 --seed 42 --opener 55-2s
```
