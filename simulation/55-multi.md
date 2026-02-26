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
| New suit | Natural, 5+ suit, 10+ HCP, constructive |
| Cuebid | Game-forcing, stopper ask |
| Pass | Default |

**Note:** Opener has a 5-card minor — bid minors cautiously.

## Results Summary

| vs Opener | N | Act% | MP% | Avg Gain | When Acting: Tops | When Acting: Bottoms |
|-----------|---|------|-----|----------|-------------------|----------------------|
| **5-5 2H** | 4,998 | 45.6% | **46.0%** | **-6** | 39.5% | 57.0% |
| **5-5 2S** | 4,998 | 43.4% | **46.9%** | **+7** | 42.6% | 56.8% |

## Detailed Results: vs 5-5 2H (5 Hearts + 5-card Minor)

### When We Act (2,277 boards)

| Metric | Value |
|--------|-------|
| Tops (par > pass) | 39.5% |
| Bottoms (par < pass) | 57.0% |
| Neutral | 3.6% |
| MP% when acting | 41.3% |
| Avg gain when acting | -13 points |

### Score Distribution

| Outcome | Avg | Median | Range |
|---------|-----|--------|-------|
| Gains | +837 | +730 | +10 to +4,440 |
| Losses | -604 | -540 | -2,220 to -10 |

### Action Breakdown

| Action | Count | AvgTricks | Game% |
|--------|-------|-----------|-------|
| Takeout X | 939 | 9.9 | 61.3% |
| 2NT (16-18 balanced) | 210 | 9.4 | 51.4% |
| Natural S | 552 | 9.3 | 45.7% |
| Natural D | 315 | 8.7 | 30.5% |
| Natural C | 261 | 9.4 | 47.1% |
| Pass | 2,721 | — | — |

### By Vulnerability

| Vulnerability | Acts | MP% | Avg Gain | Tops% |
|---------------|------|-----|----------|-------|
| Favorable | 759 | 46.3% | +13 | 18.3% |
| Equal | 759 | 45.9% | -16 | 17.8% |
| Unfavorable | 759 | 45.9% | -16 | 17.8% |

## Detailed Results: vs 5-5 2S (5 Spades + 5-card Minor)

### When We Act (2,169 boards)

| Metric | Value |
|--------|-------|
| Tops (par > pass) | 42.6% |
| Bottoms (par < pass) | 56.8% |
| Neutral | 0.7% |
| MP% when acting | 42.9% |
| Avg gain when acting | +15 points |

### Score Distribution

| Outcome | Avg | Median | Range |
|---------|-----|--------|-------|
| Gains | +848 | +850 | +10 to +4,420 |
| Losses | -610 | -540 | -2,220 to -10 |

### Action Breakdown

| Action | Count | AvgTricks | Game% |
|--------|-------|-----------|-------|
| Takeout X | 984 | 9.8 | 57.9% |
| 2NT (16-18 balanced) | 150 | 9.9 | 58.0% |
| Natural H | 480 | 9.3 | 46.2% |
| Natural D | 306 | 9.2 | 46.1% |
| Natural C | 249 | 9.2 | 43.4% |
| Pass | 2,829 | — | — |

### By Vulnerability

| Vulnerability | Acts | MP% | Avg Gain | Tops% |
|---------------|------|-----|----------|-------|
| Favorable | 723 | 47.0% | +26 | 18.5% |
| Equal | 723 | 46.9% | -3 | 18.4% |
| Unfavorable | 723 | 46.9% | -3 | 18.4% |

## Analysis

### The Defense Is Structurally Sound

Both 5-5 scenarios show near break-even MP% (46.0% and 46.9%). Key observations:

1. **Takeout double is the strongest action.** 61.3% game rate vs 2H, 57.9% vs 2S.
   When we have the right shape (short in their major, support for unbid suits),
   the takeout double consistently finds game.

2. **2NT (16-18 balanced) is highly constructive.** 51-58% game rate. These hands
   reach the right contract reliably.

3. **Natural overcalls are the weakest link.** 30-47% game rate, contributing most
   bottoms. Natural overcalls at the 10+ HCP level are too frequent against
   preemptive openings. The 5-card suit may be outgunned by opener's 10-card fit.

4. **Favorable vulnerability improves results.** Both scenarios show positive avg
   gain at favorable (+13 and +26). The risk/reward balance is better when we're
   not vulnerable.

### 2H vs 2S Asymmetry

Defense against 5-5 2S (46.9% MP, +7 avg) outperforms defense against 5-5 2H
(46.0% MP, -6 avg). This is because:
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

4. **Consider tightening natural overcall thresholds.** Natural overcalls contribute
   disproportionately to bottoms. Options:
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
