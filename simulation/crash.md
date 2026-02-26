# CRASH Simulation Results

**Date**: 2026-02-26
**Engine**: bridge-tools simulate crash (DDS double-dummy, dds-bridge 0.11)
**Hypothesis**: H-7 (Defensive Bidding Package effectiveness)
**Sample**: 40,000 deals (5,000 per opener type x 8 types), seed=42, all vulnerabilities

## Methodology

- Random deal generation with opponent hand filtering by opener type
- Double-dummy solving for all 5 strains x 4 seats per deal
- CRASH action classification per All Fives spec (LTC thresholds, shape requirements)
- Against Weak 1NT: penalty double (15+ HCP) takes priority over CRASH
- Against Strong 1NT: pure CRASH (no penalty double available)
- **Matchpoint model**: 10-table club game. Our pair uses CRASH (All Fives CC).
  The field (9 other pairs) passes over opponent's opening.
  - **Par score** = optimal outcome with both sides active (CRASH enables our entry)
  - **Pass score** = opponents play their best contract undisturbed
  - **Top** = par > pass (CRASH helped), **Bottom** = par < pass (CRASH hurt)
  - **50% MP** = break even with field

## Summary of Findings

| vs Opener | N | CRASH% | MP% | Avg Gain | Tops | Bottoms |
|---|---|---|---|---|---|---|
| **Strong 2D** | 4,998 | 34.1% | **66.2%** | **+634** | 33.2% | 0.8% |
| **Strong 2C** | 4,998 | 38.2% | **67.2%** | **+629** | 36.3% | 1.8% |
| **Strong 1C** | 4,998 | 41.9% | **64.4%** | **+427** | 35.0% | 6.2% |
| **Strong 1NT** | 4,998 | 42.9% | **59.4%** | **+282** | 30.2% | 11.3% |
| **Short Club** | 4,998 | 48.0% | **58.6%** | **+269** | 31.9% | 14.7% |
| **Garbage 1D** | 4,998 | 47.7% | **57.1%** | **+231** | 30.5% | 16.2% |
| **Weak 1NT** | 4,998 | 47.6% | **54.9%** | **+164** | 28.0% | 18.3% |
| **Multi 2D** | 4,998 | 58.4% | **46.9%** | **+27** | 25.4% | 31.6% |

**7 of 8 scenarios show CRASH as positive-expectation. Multi 2D is the sole exception.**

## When We Act (boards where CRASH says bid)

| vs Opener | Acting | MP% Acting | Top% | Bottom% | Gain:Loss |
|---|---|---|---|---|---|
| Strong 2D | 1,702 | **97.5%** | 97.5% | 2.5% | **13:1** |
| Strong 2C | 1,909 | **95.1%** | 94.9% | 4.8% | **9:1** |
| Strong 1C | 2,092 | **84.4%** | 83.6% | 14.7% | **5:1** |
| Strong 1NT | 2,143 | **71.9%** | 70.3% | 26.5% | **4:1** |
| Short Club | 2,401 | **67.9%** | 66.4% | 30.6% | **3:1** |
| Garbage 1D | 2,384 | **65.0%** | 63.9% | 34.0% | **3:1** |
| Weak 1NT | 2,379 | **60.2%** | 58.8% | 38.4% | **2.5:1** |
| Multi 2D | 2,918 | **44.7%** | 43.4% | 54.1% | **0.8:1** |

## Score Distribution When Acting

| vs Opener | Gains: avg | Gains: median | Losses: avg | Losses: median |
|---|---|---|---|---|
| Strong 2D | +1,914 | +1,300 | -143 | -40 |
| Strong 2C | +1,746 | +1,260 | -185 | -40 |
| Strong 1C | +1,261 | +1,100 | -232 | -70 |
| Strong 1NT | +1,031 | +1,040 | -254 | -70 |
| Short Club | +1,003 | +1,030 | -346 | -360 |
| Garbage 1D | +954 | +1,020 | -372 | -360 |
| Weak 1NT | +791 | +730 | -316 | -110 |
| Multi 2D | +877 | +820 | -618 | -540 |

## Vulnerability Effect

| Vulnerability | Action Rate | MP% Range | Avg Gain Range |
|---|---|---|---|
| Favorable | 53-68% | 48-76% | +97 to +1,113 |
| Equal | 34-61% | 47-67% | +12 to +577 |
| Unfavorable | 16-46% | 46-59% | -28 to +272 |

## CRASH vs 1NT Specifics

### vs Strong 1NT (15-17 HCP balanced)

- Pure CRASH — no penalty double available (their 15-17 makes penalizing unreliable)
- Action rate 42.9%, MP% 59.4%
- When acting: 70% tops, 27% bottoms — healthy 4:1 gain/loss ratio
- Average gain +1,031 vs average loss -254
- **Conclusion**: Clearly positive. Disrupting their Stayman/transfers is valuable.

### vs Weak 1NT (12-14 HCP balanced)

- Differentiated defense: penalty double (15+ HCP) available, then CRASH shifts up
- Action rate 47.6%, with **penalty double comprising 303 of 2,379 actions (12.7%)**
- Penalty double is the strongest single action: 34.7% game rate, avg 8.8 tricks
- Overall MP% 54.9% — positive but the smallest edge among standard openings
- **Conclusion**: Positive expectation, but barely. The penalty double is excellent
  (converts frequently against minimum hands). The CRASH component provides a smaller
  edge because their weak opening already limits their potential.

### vs Multi 2D (5-10 HCP, 6-card major)

- **The only negative-expectation scenario** when acting (44.7% MP when acting)
- Why: Multi 2D is a PREEMPTIVE opening. The opener is WEAK (5-10 HCP), meaning
  we typically hold the balance of power. But:
  - Their 6-card major suit is already known to be strong (by suit length)
  - We act 58.4% of the time (highest rate) — the thresholds are too loose
  - Our losses average -618 (highest of any scenario) vs gains of +877
  - Bottom rate 54.1% when acting — majority of actions lose
- **Conclusion**: CRASH thresholds are NOT calibrated for preemptive openings.
  Against Multi 2D, we should be more conservative or use a different defense
  entirely. This is a genuine anomaly worth investigating (see Recommendations).

## Conclusions

### 1. CRASH Is Positive Against All Artificial/Strong Openings

Every artificial or strong opening (1C, 2C, 2D, 1NT strong, garbage 1D, short club)
produces MP% > 54%. The convention never has negative expectation against these
targets. This corroborates DEC-5.

### 2. The Stronger the Opening, the Better CRASH Performs

| Category | MP% | Mechanism |
|---|---|---|
| Strong (2C, 2D: 22+ HCP) | 66-67% | Pure disruption — they need space for slam |
| Medium (1C: 16+) | 64% | Disruption + partscore upside |
| 1NT (15-17) | 59% | Disruption of system (Stayman, transfers) |
| Weak artificial (garbage 1D, short C) | 57-59% | Partscore competition |
| Weak 1NT (12-14) | 55% | Penalty double + modest CRASH edge |

### 3. CRASH Fails Against Preemptive Openings

Multi 2D (46.9% MP) is the clear outlier. Preemptive openings are fundamentally
different from artificial/strong openings:
- The opener is WEAK, so we likely hold the majority of points
- Their suit is KNOWN (6-card major), reducing our disruption value
- Our CRASH entry competes against our own ability to bid naturally
- The LTC/shape thresholds, designed for defensive positions, are too aggressive
  here

### 4. Risk Profile Follows a Clear Pattern

| Category | Gain:Loss Ratio | When to Act |
|---|---|---|
| Strong openings | 9-13:1 | Always (near-free) |
| Medium openings | 4-5:1 | Always (strongly positive) |
| Weak artificial | 2.5-3:1 | Always (positive) |
| Preemptive | 0.8:1 | **Reconsider** |

### 5. Vulnerability Thresholds Are Well-Calibrated for Artificial Openings

Favorable/Equal/Unfavorable all produce positive MP% against non-preemptive
openings. The 5-5+ shape requirement at unfavorable correctly limits exposure.

## Recommendations

1. **Continue using CRASH against all artificial/strong openings.** The data is
   unambiguous: MP% 55-67%, gain:loss 2.5:1 to 13:1.

2. **CRASH excluded against Multi 2D.** Negative expectation (46.9% MP, 0.8:1).
   See `2d-multi.md` for the Fighters defense simulation.

3. **Penalty double against Weak 1NT is excellent.** The 15+ HCP threshold is
   well-placed — 34.7% game rate from penalty double alone.

4. **Against Strong 1NT, emphasize CRASH entry.** 70% tops when acting, with
   the disruption value to their system (Stayman/transfers) being the key.

## Limitations

- **Double-dummy**: Results assume perfect play. Real-table results will be noisier
  but the directional conclusions should hold.
- **Field model**: Assumes all other pairs pass. In practice, some pairs may have
  their own defenses (Cappelletti, DONT, etc.), compressing the advantage by 2-5%.
- **No bidding simulation**: Par is a proxy for CRASH entry outcome. Real MP% depends
  on how well we handle the subsequent auction.

## Reproducibility

```bash
cargo run --release -- simulate crash -n 5000 --seed 42 --opener strong-2c
cargo run --release -- simulate crash -n 5000 --seed 42 --opener strong-2d
cargo run --release -- simulate crash -n 5000 --seed 42 --opener strong-1c
cargo run --release -- simulate crash -n 5000 --seed 42 --opener strong-1nt
cargo run --release -- simulate crash -n 5000 --seed 42 --opener weak-1nt
cargo run --release -- simulate crash -n 5000 --seed 42 --opener garbage-1d
cargo run --release -- simulate crash -n 5000 --seed 42 --opener short-club
cargo run --release -- simulate crash -n 5000 --seed 42 --opener multi-2d
```
