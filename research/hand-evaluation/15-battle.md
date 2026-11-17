---
title: "The Battle — 14 Evaluation Methods on 100 000 Random Deals"
subtitle: "Who reaches the double-dummy optimal contract?"
author: "Fermat Institute — Principal Investigator for the Director"
date: "17 November 2026"
mainfont: "DejaVu Serif"
sansfont: "DejaVu Sans"
monofont: "DejaVu Sans Mono"
fontsize: 10pt
geometry: "margin=1.6cm"
colorlinks: true
---

# 1. Verdict

- **There is no single champion — there are two.** On deals with a major fit, the best
  counts are **Kaplan–Rubens and Zar** (tied), then NLTC. On deals without one (played in
  notrump), the best counts are **Fifths, BUM-RAP and plain HCP** (tied).
- **Zar beats NLTC** (pre-registered test P8, confirmed): **+0.21 IMP per partnership
  decision** vulnerable [±0.03], +0.17 non-vulnerable [±0.02]. On major-fit deals alone the
  gap is small (about +0.08); most of it comes from notrump deals, where loser counts fail.
- **Use two lenses.** A partnership that counts Zar (or Kaplan–Rubens) when it has a major
  fit and Fifths when it doesn't loses about **2.52 IMPs** per decision against the DD-best
  level, against 2.66 for the best single method (BUM-RAP) and 2.76 for HCP. That is about
  **+0.14 IMP per decision over any single count, +0.24 over HCP**. These figures are
  derived from the per-regime means below.
- **Classic LTC finishes last** in both regimes again. NLTC beats it by +0.52.
- **Zar's own published lines (52 game / 62 slam / 67 grand) are almost DD-optimal with a
  major fit**: the tuned lines came out at 51½ / 62½ / 68½, and the published lines lose
  only 0.06 IMP there. Used for notrump, the same lines are poor.

# 2. The table (vulnerable; 70 000 test deals, about 75 800 partnership decisions)

"Optimal %" = how often the method chose the double-dummy best level (partscore, game,
small slam or grand) in the shared strain. "IMPs lost" = average IMPs given up against that
best level (0 = perfect). Lower losses are better.

| Method | Optimal % (all) | Optimal % fit | Optimal % NT | IMPs lost (all) | lost: fit deals | lost: NT deals |
|---|---|---|---|---|---|---|
| **BUM-RAP** | **71.0** | 63.3 | **78.4** | **−2.66** | −3.41 | −1.94 |
| **Fifths** | 70.3 | 62.3 | 78.0 | −2.69 | −3.50 | **−1.91** |
| HCP | 68.7 | 60.8 | 76.3 | −2.76 | −3.57 | −1.97 |
| HCP + length | 68.7 | 63.0 | 74.2 | −2.75 | −3.36 | −2.17 |
| **Kaplan–Rubens** | 68.7 | **66.5** | 70.7 | −2.82 | **−3.14** | −2.52 |
| **Zar** | 66.9 | 65.1 | 68.7 | −2.87 | **−3.15** | −2.61 |
| OHE start (HL+D) ¹ | 67.8 | 63.0 | 72.4 | −2.89 | −3.48 | −2.33 |
| HCP + length + shortness | 65.3 | 63.3 | 67.3 | −3.02 | −3.35 | −2.70 |
| OHE full ¹ | 65.4 | 63.1 | 67.7 | −3.04 | −3.42 | −2.67 |
| NLTC | 64.3 | 65.1 | 63.5 | −3.06 | −3.23 | −2.90 |
| Classic LTC | 57.1 | 56.4 | 57.7 | −3.52 | −3.75 | −3.31 |
| *Zar @ 52/62/67 (published)* | 60.2 | 64.8 | 55.8 | −3.31 | −3.21 | −3.41 |
| *HCP @ 25/33/37 (published)* | 68.5 | 58.4 | 78.2 | −3.27 | −4.54 | −2.06 |
| *OHE full @ published* ¹ | 52.9 | 42.0 | 63.4 | −4.61 | −6.24 | −3.05 |

Overall 95% intervals are about ±0.03 IMP. ¹ OHE is a secondary-source codification (see §5).

# 3. Head-to-head (IMPs per partnership decision, row vs column, 95% CI)

| Match | Vulnerable | Not vulnerable |
|---|---|---|
| **Zar vs NLTC (P8)** | **+0.21 ± 0.03** | **+0.17 ± 0.02** |
| Kaplan–Rubens vs Zar | +0.04 ± 0.02 | +0.02 ± 0.02 |
| BUM-RAP vs HCP | +0.12 ± 0.02 | +0.10 ± 0.02 |
| Fifths vs HCP | +0.08 ± 0.02 | +0.05 ± 0.01 |
| BUM-RAP vs Kaplan–Rubens | +0.16 ± 0.03 | +0.14 ± 0.02 |
| Zar vs HCP (all deals mixed) | −0.09 ± 0.03 | −0.07 ± 0.02 |
| NLTC vs classic LTC | +0.52 ± 0.02 | +0.35 ± 0.02 |
| Zar tuned vs Zar published lines | +0.44 ± 0.03 | +0.39 ± 0.02 |
| HCP tuned vs HCP 25/33/37 | +0.65 ± 0.03 | +0.33 ± 0.02 |
| OHE full vs Zar | −0.19 ± 0.03 | −0.14 ± 0.03 |

Non-vulnerable losses are smaller overall (BUM-RAP −1.98, HCP −2.05, Zar −2.15, NLTC −2.29,
classic LTC −2.58), and the ranking is the same.

# 4. What the double-dummy lines say (and why not to copy them to the table)

Tuned lines on major-fit deals (partnership totals): HCP game 21½, slam 29½, grand 32½;
Zar 51½ / 62½ / 68½; Kaplan–Rubens 23.6 / 31.7 / 35.9; NLTC 15¾ / 12¼ / 9¾ combined losers.
The HCP lines are far below the familiar 25 / 33 / 37 because double-dummy play is
clairvoyant: every finesse works when it should, and there is no keycard check. Zar's
published scale, by contrast, already sits on the DD lines. **Read the rankings, not the
thresholds.**

# 5. How the battle was run

- 100 000 uniformly random deals (seed 1000), each double-dummy solved in all strains.
  Every partnership with 20+ combined HCP is one decision (108 234 in all). Opponents are
  assumed silent.
- Strain and declarer are the same for every method: an 8-card major fit if there is one,
  otherwise notrump. Only the **level** is judged: partscore, game, small slam or grand.
- Each method adds both partners' counts (perfect exchange). OHE "full" also uses its fit,
  misfit and wasted-honour adjustments, so it gets more information than the others.
- Each method's three cut points per regime were tuned on the first 30 000 deals and
  scored on the other 70 000. Published-line variants use the authors' thresholds as they
  are. Uncertainty is cluster-robust by deal. The plan was written before the test deals
  were analysed (`05-confirmation-plan.md`, Addendum C and its two amendments).
- **OHE (Darricades)** is implemented from Neil Timm's published overview, not the book.
  It reproduces Timm's two worked examples exactly, but its totals run on a larger scale
  than the book's (tuned game line about 28 to 42 against the published 27), so the
  "@ published" result is not a fair test of Darricades. Details:
  `14-ohe-codification.md`.
- **Limits:** double-dummy play, no opponents' bidding, no keycard checks, no minor-suit
  contracts, strain chosen by a fixed rule. These are simulations, not table results.
  Data: `data/battle-100k-s1000.jsonl` (sha256 prefix 806ce88f…); output and the full
  head-to-head matrices: `confirm/battle-100k-*.txt` and `*.csv`.
