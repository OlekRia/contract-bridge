# 20-team evaluation tournament (round robin, 5 000-board matches)

Runner `eval_lab tournament20 <data> <train frac> 5000` (perfect exchange of both hands, double-dummy results, silent opponents, dealer and vulnerability rotating as at the table; lines tuned on the 30 000 seed-1000 training deals). Two events: **A** = 70 000 fresh seed-2000 boards (14 match blocks), **B** = 30 000 fresh seed-5000 boards. 20 teams, 190 matches of 5 000 boards per event, WBF 20-VP scale. Outputs: `confirm/tournament20-s2000.md`, `confirm/tournament20-s5000.md`.

| # | Team | Won A | VP A | Won B | VP B | IMPs lost vs DD/board (A) | vs Fermat Complete (A) | fit boards (A) | NT boards (A) |
|---|---|---|---|---|---|---|---|---|---|
| 1 | Fermat Fit-Quality (H-48) | 19/19 | 18.77 | 19/19 | 19.02 | -2.072 | +0.071 [+0.052, +0.090] | +0.146 [+0.107, +0.184] | +0.000 [+0.000, +0.000] |
| 2 | Fermat + wasted honours (H-47) | 18/19 | 18.28 | 17/19 | 18.01 | -2.116 | +0.025 [+0.012, +0.038] | +0.051 [+0.025, +0.078] | +0.000 [+0.000, +0.000] |
| 3 | Fermat Complete | 17/19 | 17.32 | 18/19 | 17.98 | -2.140 | — | — | — |
| 4 | Fermat + partnership shortness (H-46) | 16/19 | 16.85 | 16/19 | 16.98 | -2.169 | -0.036 [-0.048, -0.024] | -0.075 [-0.100, -0.050] | +0.000 [+0.000, +0.000] |
| 5 | Fermat WL two-lens | 14/19 | 15.04 | 14/19 | 14.77 | -2.231 | -0.094 [-0.111, -0.076] | -0.192 [-0.228, -0.157] | +0.000 [+0.000, +0.000] |
| 6 | Fermat two-lens | 14/19 | 13.88 | 14/19 | 14.06 | -2.263 | -0.132 [-0.147, -0.117] | -0.179 [-0.208, -0.150] | -0.088 [-0.100, -0.075] |
| 7 | Kaplan-Rubens / Fifths | 13/19 | 13.69 | 14/19 | 13.44 | -2.271 | -0.138 [-0.160, -0.115] | -0.230 [-0.270, -0.190] | -0.051 [-0.073, -0.028] |
| 8 | Zar / Fifths | 13/19 | 13.24 | 12/19 | 12.83 | -2.278 | -0.144 [-0.164, -0.124] | -0.243 [-0.276, -0.210] | -0.051 [-0.073, -0.028] |
| 9 | All Fives today (Zar / HCP) | 11/19 | 12.21 | 11/19 | 12.70 | -2.300 | -0.169 [-0.189, -0.149] | -0.243 [-0.276, -0.210] | -0.098 [-0.122, -0.075] |
| 10 | NLTC / Fifths | 10/19 | 11.62 | 10/19 | 10.68 | -2.317 | -0.185 [-0.209, -0.161] | -0.327 [-0.370, -0.283] | -0.051 [-0.073, -0.028] |
| 11 | Zar published (52/62/67) / HCP 25 | 8/19 | 8.84 | 9/19 | 8.76 | -2.390 | -0.271 [-0.292, -0.250] | -0.331 [-0.364, -0.298] | -0.214 [-0.241, -0.187] |
| 12 | BUM-RAP | 9/19 | 8.73 | 8/19 | 8.24 | -2.414 | -0.298 [-0.325, -0.271] | -0.505 [-0.553, -0.457] | -0.102 [-0.126, -0.077] |
| 13 | Fifths | 7/19 | 7.55 | 7/19 | 8.17 | -2.436 | -0.322 [-0.348, -0.295] | -0.608 [-0.657, -0.559] | -0.051 [-0.073, -0.028] |
| 14 | HCP (tuned lines) | 6/19 | 6.17 | 5/19 | 5.62 | -2.493 | -0.386 [-0.413, -0.359] | -0.691 [-0.740, -0.641] | -0.098 [-0.122, -0.075] |
| 15 | HCP + length | 5/19 | 5.29 | 6/19 | 6.48 | -2.511 | -0.393 [-0.420, -0.366] | -0.483 [-0.526, -0.440] | -0.307 [-0.340, -0.274] |
| 16 | Kaplan-Rubens | 4/19 | 4.95 | 4/19 | 4.49 | -2.544 | -0.422 [-0.450, -0.395] | -0.230 [-0.270, -0.190] | -0.605 [-0.642, -0.568] |
| 17 | Classic LTC / HCP | 3/19 | 4.18 | 3/19 | 4.05 | -2.559 | -0.442 [-0.469, -0.415] | -0.805 [-0.854, -0.756] | -0.098 [-0.122, -0.075] |
| 18 | HCP + length + shortness | 2/19 | 1.86 | 2/19 | 2.04 | -2.745 | -0.647 [-0.675, -0.618] | -0.471 [-0.512, -0.429] | -0.813 [-0.851, -0.775] |
| 19 | OHE full (Darricades) | 1/19 | 1.52 | 1/19 | 1.67 | -2.774 | -0.655 [-0.686, -0.623] | -0.520 [-0.571, -0.469] | -0.782 [-0.821, -0.743] |
| 20 | SAYC classic (HCP 25/33/37) | 0/19 | 0.00 | 0/19 | 0.00 | -2.907 | -0.871 [-0.904, -0.838] | -1.565 [-1.625, -1.504] | -0.214 [-0.241, -0.187] |

## Reading

- **Fermat Fit-Quality (H-48) wins both events, 19 of 19 matches each** (18.8 and 19.0 VP/match); +0.071 IMP/board over Fermat Complete (event A), all of it on major-fit boards.
- The four Fermat-family teams take places 1-4 in both events; today's All Fives evaluation (Zar / HCP) is 9th; the published-line methods (SAYC 25/33/37, Zar 52/62/67) and OHE finish near or at the bottom.
- Notrump boards: every team using Fermat NT with the stopper check gains about +0.05 to +0.10 there over Fifths/HCP-based NT lenses; the Fit-Quality gain is fit-only (NT unchanged).
- Caveats: perfect exchange (no auction), double dummy, silent opponents. Event A is a region already seen in exploratory runs (H-45, H-46, H-48); event B was the H-48 confirmation set. The H-48 step-2 auction models showed the Fit-Quality gain survives silent auctions (+0.03 to +0.05) but not the competitive model (`33-h48-systems.md`).

## Event C — 30 000 brand-new random deals (seed 6000, never used before)

Generated for this event (`data/battle-fresh30k-s6000.jsonl`, sha256 prefix 399aba8a…, 462 s);
same 20 teams, same training deals, 190 matches of 5 000 boards. Output:
`confirm/tournament20-s6000.md`.

- **Fermat Fit-Quality (H-48) wins again, 19/19, 19.49 VP/match** — the largest margin of the three
  events: **+0.101 IMP/board over Fermat Complete [+0.073, +0.129]**, +0.207 on major-fit boards.
- Places 1-4 again the Fermat family; All Fives today 9th again (−0.170 vs Fermat Complete);
  SAYC classic 25/33/37 last again (0.03 VP/match).
- H-47 wasted honours: +0.008 [−0.012, +0.028] — the same "≈ 0" as its own confirmation.
- Three independent events, three identical podiums.

