# Systems study — SAYC, Precision, Acol, 2/1 GF, All Fives (and perfect exchange)

Requested by the Director. Same random deals as the tournaments (`data/battle-100k-s1000.jsonl`;
train deals 1-30 000, test deals 30 001-100 000), 41 two-lens teams, 5 000-board round robins,
DD scoring. Command: `eval_lab systems data/battle-100k-s1000.jsonl 0.3 5000 out.md`.
Code: `sim::battle::{System, a_first, exchange}` (unit-tested), `bin/eval_lab.rs::systems`.

## Information model (opponents silent)

1. **Who opens:** the first partner in rotation from the dealer (dealer N/E/S/W by board
   number) holding an opening hand: SAYC / Acol / 2/1 12+ HCP, Precision 11+ HCP, or 10+
   with the Rule of 20. If neither qualifies, the stronger hand opens light.
2. **Captain:** one player decides the level. It knows its own hand exactly and its partner
   only by the range the auction showed (a "bucket"):

| Opener | SAYC | Precision | Acol | 2/1 GF |
|---|---|---|---|---|
| Strong | 22+ 2♣; opener captains; responder in 4-HCP steps | 16+ 1♣; opener captains; relays show responder to 2 HCP + balanced/not | 23+ 2♣; as SAYC | 22+ 2♣; as SAYC |
| Balanced | 1NT 15-17, 2NT 20-21, 18-19 via 2NT rebid, 12-14 via 1NT rebid | 1NT 13-15; 11-12 via 1♦ | 1NT 12-14, 2NT 20-22, 15-17 via 1NT rebid, 18-19 via 2NT rebid | as SAYC |
| Unbalanced | rebid shows 12-15 / 16-18 / 19-21 | 11-15, 2NT ask splits 11-13 / 14-15 | 12-15 / 16-19 / 20-22 | responder 12+ (game force): 12-14 / 15-17 / 18+; otherwise as SAYC |

**All Fives** (from `contract-bridge/all-fives.yaml`; seat-dependent — the model knows whether
the dealer is in the partnership):

| Opener | All Fives model |
|---|---|
| 24+ HCP, or ≤ 4 losers with 16+ HCP (not 5-5) | 2♦ game force: opener captains; responder shows **controls** (0-1 / 2 / 3 / 4 / 5 / 6+) |
| 22-23 balanced; strong major (≤ 5 losers, 16+, 5+ major with A-K-Q); strong two-suiter (≤ 4 losers, 5-5) | Benji 2♣ types 1 / 2 / 3: responder captains |
| 20-21 balanced | 2NT |
| 12-14 balanced | 1NT in 1st/2nd seat; via 1♣ (best minor) in 3rd/4th |
| 15-17 balanced | 1NT in 3rd/4th seat; one-suit opening + 2NT rebid in 1st/2nd |
| 18-19 balanced | one-suit opening, strong balanced rebid |
| 6-10 with a 6-card major (cannot open) | weak two: partner captains |
| Opening strength | 1st/2nd seat 12+ (or 10+ with ≤ 7 losers); 3rd/4th seat 8+ |
| Unbalanced one-suit | responder 13+ (2/1 game force): 12-14 / 15-17 / 18+; otherwise forcing-1NT/raise rebids 12-15 / 16-18 / 19+ |

3. **Decision:** captain's own count + the *expected* count of partner's hand given the bucket
   and whether the partnership has a major fit (learned per count and per system on the
   training deals); three cuts per strain regime tuned on the training deals.
4. **Perfect exchange** = both counts known exactly (the earlier tournaments).

**Limits:** opponents never bid (this flatters strong-club relay systems, whose known
weakness is interference, and ignores the pre-emptive value of weak twos); no strain search
(shared rule); no keycard checking; double-dummy play; the bucket tables are stylised versions
of each system, not full system notes.


## Perfect exchange — round robin (41 teams, 820 matches of 5000 boards)

| # | Team (fit lens / NT lens) | Won | VP/match | IMPs/board vs field | % optimal vs DD | IMPs lost vs DD/board |
|---|---|---|---|---|---|---|
| 1 | Zar / Fifths | 38/40 | 17.69 | +0.229 | 70.9% | -2.324 |
| 2 | Kaplan-Rubens / BUM-RAP | 38/40 | 17.47 | +0.217 | 71.4% | -2.324 |
| 3 | Kaplan-Rubens / Fifths | 37/40 | 17.38 | +0.220 | 71.4% | -2.312 |
| 4 | Zar / HCP | 37/40 | 17.03 | +0.209 | 70.5% | -2.348 |
| 5 | Kaplan-Rubens / HCP | 36/40 | 16.94 | +0.201 | 70.9% | -2.336 |
| 6 | Zar / BUM-RAP | 37/40 | 16.93 | +0.191 | 71.0% | -2.336 |
| 7 | NLTC / Fifths | 34/40 | 16.31 | +0.186 | 70.7% | -2.361 |
| 8 | NLTC / BUM-RAP | 32/40 | 15.29 | +0.163 | 70.8% | -2.373 |
| 9 | NLTC / HCP | 32/40 | 15.20 | +0.143 | 70.3% | -2.385 |
| 10 | Kaplan-Rubens / HCP+len | 34/40 | 14.85 | +0.127 | 69.7% | -2.426 |
| … | … | | | | | |
| 39 | HCP / Kaplan-Rubens | 2/40 | 1.28 | -0.292 | 65.0% | -2.774 |
| 40 | Classic LTC / Kaplan-Rubens | 1/40 | 0.64 | -0.427 | 63.8% | -2.852 |
| 41 | Classic (HCP 25/33/37) | 0/40 | 0.07 | -0.566 | 66.9% | -2.936 |

## SAYC — round robin (41 teams, 820 matches of 5000 boards)

| # | Team (fit lens / NT lens) | Won | VP/match | IMPs/board vs field | % optimal vs DD | IMPs lost vs DD/board |
|---|---|---|---|---|---|---|
| 1 | Zar / HCP | 36/40 | 16.05 | +0.158 | 68.4% | -2.527 |
| 2 | Kaplan-Rubens / BUM-RAP | 37/40 | 15.88 | +0.147 | 68.6% | -2.531 |
| 3 | Zar / Fifths | 34/40 | 15.81 | +0.145 | 68.5% | -2.523 |
| 4 | Zar / BUM-RAP | 37/40 | 15.80 | +0.137 | 68.6% | -2.526 |
| 5 | Kaplan-Rubens / Fifths | 33/40 | 15.59 | +0.140 | 68.6% | -2.528 |
| 6 | Kaplan-Rubens / HCP | 37/40 | 15.56 | +0.133 | 68.5% | -2.532 |
| 7 | HCP+len / Fifths | 31/40 | 15.54 | +0.138 | 68.3% | -2.538 |
| 8 | HCP+len / HCP | 33/40 | 15.42 | +0.147 | 68.2% | -2.542 |
| 9 | HCP+len / BUM-RAP | 35/40 | 14.90 | +0.124 | 68.3% | -2.540 |
| 10 | HCP+len+short / BUM-RAP | 31/40 | 14.75 | +0.114 | 68.0% | -2.553 |
| … | … | | | | | |
| 39 | HCP / Kaplan-Rubens | 2/40 | 1.82 | -0.203 | 64.5% | -2.836 |
| 40 | Classic LTC / Kaplan-Rubens | 1/40 | 1.04 | -0.274 | 63.6% | -2.874 |
| 41 | Classic (HCP 25/33/37) | 0/40 | 0.00 | -0.762 | 64.2% | -3.224 |

## Precision — round robin (41 teams, 820 matches of 5000 boards)

| # | Team (fit lens / NT lens) | Won | VP/match | IMPs/board vs field | % optimal vs DD | IMPs lost vs DD/board |
|---|---|---|---|---|---|---|
| 1 | Zar / Fifths | 40/40 | 16.69 | +0.163 | 69.6% | -2.461 |
| 2 | Zar / HCP | 37/40 | 16.33 | +0.160 | 69.3% | -2.474 |
| 3 | Kaplan-Rubens / HCP | 37/40 | 16.09 | +0.150 | 69.1% | -2.471 |
| 4 | Kaplan-Rubens / Fifths | 38/40 | 16.06 | +0.146 | 69.4% | -2.458 |
| 5 | HCP+len / Fifths | 36/40 | 15.11 | +0.125 | 69.2% | -2.484 |
| 6 | Kaplan-Rubens / BUM-RAP | 34/40 | 15.07 | +0.127 | 69.1% | -2.483 |
| 7 | Zar / BUM-RAP | 33/40 | 15.04 | +0.117 | 69.3% | -2.486 |
| 8 | HCP+len+short / Fifths | 32/40 | 14.67 | +0.115 | 69.0% | -2.491 |
| 9 | NLTC / Fifths | 32/40 | 14.36 | +0.110 | 69.0% | -2.497 |
| 10 | HCP+len / HCP | 32/40 | 14.31 | +0.116 | 68.9% | -2.497 |
| … | … | | | | | |
| 39 | HCP / Kaplan-Rubens | 3/40 | 2.11 | -0.190 | 65.0% | -2.771 |
| 40 | Classic LTC / Kaplan-Rubens | 1/40 | 0.87 | -0.326 | 64.3% | -2.848 |
| 41 | Classic (HCP 25/33/37) | 0/40 | 0.00 | -0.686 | 65.1% | -3.130 |

## Acol — round robin (41 teams, 820 matches of 5000 boards)

| # | Team (fit lens / NT lens) | Won | VP/match | IMPs/board vs field | % optimal vs DD | IMPs lost vs DD/board |
|---|---|---|---|---|---|---|
| 1 | Zar / Fifths | 36/40 | 15.99 | +0.146 | 68.4% | -2.529 |
| 2 | Kaplan-Rubens / Fifths | 35/40 | 15.95 | +0.143 | 68.4% | -2.536 |
| 3 | Kaplan-Rubens / BUM-RAP | 40/40 | 15.93 | +0.144 | 68.4% | -2.541 |
| 4 | Zar / HCP | 37/40 | 15.89 | +0.146 | 68.2% | -2.534 |
| 5 | HCP+len / Fifths | 31/40 | 15.62 | +0.139 | 68.2% | -2.542 |
| 6 | HCP+len / HCP | 32/40 | 15.46 | +0.144 | 68.0% | -2.547 |
| 7 | Zar / BUM-RAP | 34/40 | 15.44 | +0.127 | 68.5% | -2.534 |
| 8 | Kaplan-Rubens / HCP | 33/40 | 15.43 | +0.131 | 68.2% | -2.541 |
| 9 | HCP+len / BUM-RAP | 36/40 | 14.95 | +0.124 | 68.2% | -2.547 |
| 10 | HCP+len+short / Fifths | 32/40 | 14.79 | +0.120 | 67.9% | -2.555 |
| … | … | | | | | |
| 39 | HCP / Kaplan-Rubens | 2/40 | 2.05 | -0.191 | 64.5% | -2.830 |
| 40 | Classic LTC / Kaplan-Rubens | 1/40 | 1.07 | -0.274 | 63.5% | -2.876 |
| 41 | Classic (HCP 25/33/37) | 0/40 | 0.00 | -0.708 | 64.4% | -3.188 |

## 2/1 GF — round robin (41 teams, 820 matches of 5000 boards)

| # | Team (fit lens / NT lens) | Won | VP/match | IMPs/board vs field | % optimal vs DD | IMPs lost vs DD/board |
|---|---|---|---|---|---|---|
| 1 | Zar / Fifths | 38/40 | 16.46 | +0.162 | 68.5% | -2.516 |
| 2 | Kaplan-Rubens / Fifths | 37/40 | 16.25 | +0.155 | 68.6% | -2.521 |
| 3 | Zar / HCP | 36/40 | 16.02 | +0.156 | 68.3% | -2.528 |
| 4 | Kaplan-Rubens / BUM-RAP | 36/40 | 15.79 | +0.147 | 68.4% | -2.531 |
| 5 | Zar / BUM-RAP | 35/40 | 15.50 | +0.132 | 68.4% | -2.526 |
| 6 | Kaplan-Rubens / HCP | 35/40 | 15.24 | +0.133 | 68.3% | -2.533 |
| 7 | HCP+len / Fifths | 33/40 | 15.18 | +0.132 | 68.3% | -2.536 |
| 8 | HCP+len+short / Fifths | 33/40 | 15.04 | +0.130 | 68.0% | -2.546 |
| 9 | HCP+len / HCP | 32/40 | 14.99 | +0.131 | 68.0% | -2.548 |
| 10 | HCP+len+short / BUM-RAP | 31/40 | 14.53 | +0.113 | 67.8% | -2.556 |
| … | … | | | | | |
| 39 | HCP / Kaplan-Rubens | 2/40 | 2.14 | -0.180 | 64.4% | -2.829 |
| 40 | Classic LTC / Kaplan-Rubens | 1/40 | 0.91 | -0.282 | 63.6% | -2.889 |
| 41 | Classic (HCP 25/33/37) | 0/40 | 0.00 | -0.841 | 63.6% | -3.288 |

## All Fives — round robin (41 teams, 820 matches of 5000 boards)

| # | Team (fit lens / NT lens) | Won | VP/match | IMPs/board vs field | % optimal vs DD | IMPs lost vs DD/board |
|---|---|---|---|---|---|---|
| 1 | Zar / HCP | 37/40 | 16.44 | +0.163 | 68.3% | -2.519 |
| 2 | Zar / Fifths | 37/40 | 16.44 | +0.158 | 68.5% | -2.513 |
| 3 | Kaplan-Rubens / Fifths | 37/40 | 16.08 | +0.147 | 68.7% | -2.516 |
| 4 | Kaplan-Rubens / BUM-RAP | 37/40 | 15.97 | +0.145 | 68.5% | -2.528 |
| 5 | Zar / BUM-RAP | 35/40 | 15.51 | +0.131 | 68.3% | -2.525 |
| 6 | Kaplan-Rubens / HCP | 36/40 | 15.43 | +0.131 | 68.5% | -2.522 |
| 7 | HCP+len / Fifths | 34/40 | 15.37 | +0.129 | 68.1% | -2.542 |
| 8 | HCP+len+short / Fifths | 32/40 | 14.63 | +0.116 | 67.9% | -2.550 |
| 9 | HCP+len / HCP | 33/40 | 14.54 | +0.117 | 67.9% | -2.547 |
| 10 | NLTC / Fifths | 28/40 | 14.21 | +0.113 | 68.0% | -2.560 |
| … | … | | | | | |
| 39 | BUM-RAP / Kaplan-Rubens | 4/40 | 2.06 | -0.208 | 64.1% | -2.821 |
| 40 | Classic LTC / Kaplan-Rubens | 1/40 | 0.91 | -0.283 | 63.9% | -2.869 |
| 41 | Classic (HCP 25/33/37) | 0/40 | 0.00 | -0.772 | 64.1% | -3.221 |

## System shoot-out — same evaluation, different system

IMPs lost per board against the double-dummy-best level, all 70000 test boards (0 = perfect):

| Team | Perfect exchange | SAYC | Precision | Acol | 2/1 GF | All Fives |
|---|---|---|---|---|---|---|
| Zar / Fifths | -2.324 | -2.523 | -2.461 | -2.529 | -2.516 | -2.513 |
| Zar / HCP | -2.348 | -2.527 | -2.474 | -2.534 | -2.528 | -2.519 |
| Kaplan-Rubens / Fifths | -2.312 | -2.528 | -2.458 | -2.536 | -2.521 | -2.516 |
| NLTC / Fifths | -2.361 | -2.566 | -2.497 | -2.575 | -2.565 | -2.560 |
| HCP / HCP | -2.520 | -2.630 | -2.564 | -2.629 | -2.618 | -2.621 |
| Classic LTC / HCP | -2.598 | -2.668 | -2.640 | -2.674 | -2.679 | -2.662 |
| Classic (HCP 25/33/37) | -2.936 | -3.224 | -3.130 | -3.188 | -3.288 | -3.221 |

Head-to-head of the systems with the same evaluation (Zar / Fifths), IMPs per board (row vs column):

| | Perfect exchange | SAYC | Precision | Acol | 2/1 GF | All Fives |
|---|---|---|---|---|---|---|
| Perfect exchange | — | +0.248 | +0.158 | +0.249 | +0.231 | +0.222 |
| SAYC | -0.248 | — | -0.091 | +0.001 | -0.018 | -0.025 |
| Precision | -0.158 | +0.091 | — | +0.093 | +0.074 | +0.064 |
| Acol | -0.249 | -0.001 | -0.093 | — | -0.018 | -0.029 |
| 2/1 GF | -0.231 | +0.018 | -0.074 | +0.018 | — | -0.009 |
| All Fives | -0.222 | +0.025 | -0.064 | +0.029 | +0.009 | — |
