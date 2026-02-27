# Sandwich Position Simulation Results

**Date**: 2026-02-27
**Engine**: bridge-tools simulate sandwich (DDS double-dummy, dds-bridge 0.11)
**Hypothesis**: H-7 (Defensive Bidding Effectiveness — Sandwich Position component)
**Sample**: 19,998 deals (1,111 per auction x vulnerability combo), seed=42

## What Is the Sandwich Position?

When opponents bid two suits at the 1-level — (1x)-Pass-(1y)-? — South is
"sandwiched" between two bidders. All Fives actions in this dangerous position:

| Action | Meaning | Requirements |
|---|---|---|
| **Double** | Both unbid suits, strength | 12+ HCP, 3+ in each unbid suit |
| **Unusual 1NT** | Both unbid suits, shape | 5+ in each unbid suit (not natural) |
| **Overcall** | Natural, good suit | 10-16 HCP, 5+ in unbid suit, 2+ top honours |
| **Pass** | Default in dangerous position | Anything else |

All actions require LTC ≤ 9 (favorable), 8 (equal), or 7 (unfavorable).

There are exactly 6 valid auctions (response must be higher-ranking than opening):

| Auction | Unbid Suits |
|---|---|
| (1♣)-P-(1♦) | ♠, ♥ |
| (1♣)-P-(1♥) | ♠, ♦ |
| (1♣)-P-(1♠) | ♥, ♦ |
| (1♦)-P-(1♥) | ♠, ♣ |
| (1♦)-P-(1♠) | ♥, ♣ |
| (1♥)-P-(1♠) | ♦, ♣ |

## Methodology

- Unconstrained deal generation: West = natural opener (12-21 HCP, suit length),
  East = responder (6+ HCP, 4+ in response suit)
- Double-dummy solving for all 5 strains x 4 seats per deal
- Sandwich classification per All Fives spec (priority: Double > Unusual 1NT > Overcall > Pass)
- **Matchpoint model**: 10-table club game. Our pair acts in the sandwich seat (All Fives CC).
  The field (9 other pairs) passes in the sandwich position.
  - **Par score** = optimal outcome with both sides active
  - **Pass score** = opponents play their best contract undisturbed
  - **Top** = par > pass, **Bottom** = par < pass

## Raw Data

```
=======================================================
Sandwich Position Simulation -- 19,998 deals
=======================================================

Deals generated: 616,614 | Accepted: 19,998 (3.2%)
DD solve time: 114s
Overall action rate: 13.5%

-- By Auction ------------------------------------------
                  Deals      Act%  AvgTricks  TheirTricks   Diff
(1c)-P-(1d)       3,333    12.4%        7.7          9.8   -2.2
(1c)-P-(1h)       3,333    12.7%        7.7          9.8   -2.1
(1c)-P-(1s)       3,333    13.2%        7.8          9.8   -2.0
(1d)-P-(1h)       3,333    14.4%        7.7          9.8   -2.0
(1d)-P-(1s)       3,333    12.8%        7.9          9.7   -1.9
(1h)-P-(1s)       3,333    15.4%        7.8          9.8   -2.0

-- By Vulnerability ------------------------------------
                  Deals      Act%  AvgTricks
Favorable         6,666    14.3%        7.7
Equal             6,666    14.4%        7.7
Unfavorable       6,666    11.6%        7.9

-- By Action -------------------------------------------
                  Count  AvgTricks  Game%
Double            1,505        7.7  13.3%
Unusual 1NT         279        7.9  16.5%
Overcall S          221        7.7  12.7%
Overcall H          213        8.0  16.0%
Overcall D          243        7.9  16.9%
Overcall C          234        7.8  12.8%
Pass             17,303

-- Sandwich vs Pass ------------------------------------
When we act:  Avg our tricks = 7.8, their tricks = 9.8

== MATCHPOINT TOURNAMENT ANALYSIS ======================

-- Overall Results -------------------------------------
Action tops   (par > pass):  2,146 ( 10.7%)
Action bottoms (par < pass):   497 (  2.5%)
Neutral (same result):      17,355 ( 86.8%)

Expected matchpoint percentage:  54.1%
  (50.0% = break even with field)
Average score gain per board:    +100 points
```

## Summary of Findings

| Auction | N | Act% | Tops% | Bottoms% | Top:Bottom |
|---|---|---|---|---|---|
| **(1♥)-P-(1♠)** | 3,333 | **15.4%** | 12.7% | 2.7% | **4.7:1** |
| **(1♦)-P-(1♥)** | 3,333 | 14.4% | 11.7% | 2.7% | **4.3:1** |
| **(1♣)-P-(1♠)** | 3,333 | 13.2% | 10.5% | 2.6% | **4.0:1** |
| **(1♦)-P-(1♠)** | 3,333 | 12.8% | 10.6% | 2.2% | **4.8:1** |
| **(1♣)-P-(1♥)** | 3,333 | 12.7% | 10.3% | 2.1% | **4.9:1** |
| **(1♣)-P-(1♦)** | 3,333 | 12.4% | 9.5% | 2.3% | **4.1:1** |

**All 6 auctions show sandwich action as strongly positive-expectation. No negative outliers.**

## Action Breakdown

| Action | Count | % of Actions | AvgTricks | Game% |
|---|---|---|---|---|
| **Double** | 1,505 | **55.8%** | 7.7 | 13.3% |
| **Unusual 1NT** | 279 | **10.3%** | 7.9 | 16.5% |
| **Overcall** | 911 | **33.8%** | 7.8 | 14.7% |
| — Overcall ♦ | 243 | 9.0% | 7.9 | 16.9% |
| — Overcall ♣ | 234 | 8.7% | 7.8 | 12.8% |
| — Overcall ♠ | 221 | 8.2% | 7.7 | 12.7% |
| — Overcall ♥ | 213 | 7.9% | 8.0 | 16.0% |

Double is the most common action (55.8%), reflecting that many hands with values in
the sandwich position have support for both unbid suits rather than a single dominant
suit. Overcalls distribute roughly evenly across the four suits, as expected.

## Vulnerability Effect

| Vulnerability | Action Rate | AvgTricks |
|---|---|---|
| Favorable | 14.3% | 7.7 |
| Equal | 14.4% | 7.7 |
| **Unfavorable** | **11.6%** | 7.9 |

The LTC gate correctly tightens at unfavorable vulnerability (≤7 vs ≤8 equal vs ≤9
favorable). The unfavorable action rate drops by ~20% relative to favorable/equal,
showing the gate is binding. This is good calibration — the dangerous position
warrants extra caution when vulnerable against not.

## When We Act (2,695 boards)

At 13.5% action rate, sandwich actions fire on roughly **1 in 7 boards** where
opponents bid two suits at the 1-level. When they fire:

- **2,146 tops vs 497 bottoms** — **4.3:1** success ratio
- **54.1% overall MP** — substantial positive edge
- Average gain of **+100 points per board** across all deals

The 54.1% MP is the highest overall expectation of any defensive convention tested,
and the 4.3:1 top:bottom ratio confirms that acting in the sandwich position is
both frequent enough and reliable enough to be a significant source of matchpoints.

## Comparison with Other Defensive Conventions

| Convention | Action Rate | Top:Bottom | MP% | Avg Gain |
|---|---|---|---|---|
| **Sandwich** | **13.5%** | **4.3:1** | **54.1%** | **+100** |
| Jump Overcall | 3.9% | 4.7:1 | 51.3% | +42 |
| Michaels/UNT | 8.5% | ~2.5:1 | 51.5% | +30 |
| RAPTOR | 4.2% | 2.1:1 | 50.7% | +19 |

The sandwich position combines high frequency (13.5%) with an excellent top:bottom
ratio (4.3:1), producing by far the highest average gain per board (+100). This is
the single most valuable defensive convention in the All Fives system.

## Key Observations

### 1. The "Dangerous Position" Reputation Is Overstated

Conventional wisdom says the sandwich position is dangerous because both opponents
have shown values. The data shows the opposite — when our hand qualifies (shape +
values + LTC gate), acting is overwhelmingly positive. The triple gate ensures we
only enter with genuine hands, and the information that both opponents have bid
actually *helps* us — we know where the missing strength is and can evaluate our
hand more accurately.

### 2. Double Is the Workhorse Action

At 55.8% of actions, the takeout double (showing both unbid suits with 12+ HCP) is
the primary sandwich tool. This makes sense: when sandwiched between two bidders,
having support for BOTH unbid suits is the safest way to compete — partner can
choose either suit.

### 3. Unusual 1NT Is Rare But Effective

Only 10.3% of actions are Unusual 1NT (5-5+ in unbid suits). Despite its low
frequency, it shows the highest game percentage (16.5%) and good trick-taking (7.9).
The 5-5 shape with LTC ≤ 8 equal (≤ 9 favorable) ensures these hands have
genuine playing strength.

### 4. (1♥)-P-(1♠) Is the Highest-Frequency Auction

At 15.4% action rate, the auction where opponents bid both majors produces the most
sandwich opportunities. The unbid suits are both minors (♦, ♣), and distributional
hands with length in both minors are relatively common in the 12+ HCP range.

### 5. Vulnerability Gate Works Correctly

The 20% drop in action rate at unfavorable (11.6% vs 14.3% favorable) shows the LTC ≤7
gate is doing its job. In the dangerous position, we rightly require extra shape
discipline when vulnerable against not.

## Conclusions

### 1. Sandwich Actions Are the Highest-Value Defensive Tool

54.1% MP and +100 average gain per board represent the strongest result in the entire
defensive simulation suite. The combination of frequency (13.5%) and reliability
(4.3:1 top:bottom) makes this a cornerstone of defensive strategy.

### 2. Shape Is More Important Than Strength

The qualification gates prioritize shape (3+ each unbid for Double, 5+ each for
Unusual 1NT, 5+ with quality for Overcall) alongside values. The LTC gate ensures
playing strength. This shape-first approach is correct for the dangerous position.

### 3. The LTC Gate Is Essential

Without the LTC gate, many more hands would qualify — and the bottom rate would
increase sharply. The vulnerability-adjusted thresholds (9/8/7) correctly manage
risk by requiring tighter shape at unfavorable vulnerability.

### 4. All 6 Auctions Are Profitable

No auction shows a negative expectation. The system works uniformly across all
opening-response combinations.

## Recommendations

1. **Act confidently in the sandwich position when qualified.** The data shows this
   is the most profitable defensive position, not the most dangerous.

2. **Prioritize the takeout double.** With 12+ HCP and support for both unbid suits,
   doubling is correct more than half the time. Don't search for exotic alternatives.

3. **Use Unusual 1NT only with genuine 5-5+ shape.** It's rare (10.3%) but effective.
   Don't stretch the shape requirement.

4. **Tighten at unfavorable.** The LTC ≤7 gate correctly reduces action rate by 20%.
   Respect this — the occasional large penalty at unfavorable is not worth the extra
   frequency.

5. **Partnership drill**: sandwich situations require practiced responses. Partner
   of the sandwich bidder needs clear methods for advancing, passing, or competing
   further.

## Limitations

- **Double-dummy**: Results assume perfect play. The sandwich position is particularly
  affected by information — opponents know their combined strength, which may help
  them defend or compete more effectively in practice.
- **Field model**: Assumes all other pairs pass in the sandwich seat. Some pairs use
  their own sandwich treatments, compressing the advantage.
- **No bidding continuation**: The model doesn't simulate opponents' reactions (penalty
  doubles, further competition). In practice, sandwich actions may provoke more
  aggressive opponent responses.
- **Unconstrained filter complexity**: The filter requires both West and East to have
  specific hand types, producing a low acceptance rate (3.2%). This means the deal
  distribution is somewhat artificial — in real play, the frequency of sandwich
  situations will vary.
- **13.5% action rate is high**: This is the highest action rate of any defensive
  convention. In practice, some qualifying hands may be better served by passing
  (e.g., when partner is a passed hand and unlikely to hold much). The simulation
  doesn't model this judgment call.

## Reproducibility

```bash
cargo run --release -- simulate sandwich -n 20000 --seed 42 --opener all
cargo run --release -- simulate sandwich -n 3000 --seed 42 --opener 1c-1d
cargo run --release -- simulate sandwich -n 3000 --seed 42 --opener 1c-1h
cargo run --release -- simulate sandwich -n 3000 --seed 42 --opener 1c-1s
cargo run --release -- simulate sandwich -n 3000 --seed 42 --opener 1d-1h
cargo run --release -- simulate sandwich -n 3000 --seed 42 --opener 1d-1s
cargo run --release -- simulate sandwich -n 3000 --seed 42 --opener 1h-1s
```
