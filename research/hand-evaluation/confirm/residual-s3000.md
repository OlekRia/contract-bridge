Residual of Fermat Complete on major-fit sides: tricks = -1.825 + 0.2100 × Fermat fit total (train).
Share of the residual variance the 18 features explain: train 24.2%, test region (own fit) 23.2%.

| Feature | tricks per unit (train) | tricks per unit (test-region fit) | SD (test) | tricks per SD |
|---|---|---|---|---|
| trump fit 9 | +0.348 | +0.296 | 0.46 | +0.160 |
| trump fit 10+ | +0.275 | +0.270 | 0.31 | +0.084 |
| top trump honours (A/K/Q) | +0.130 | +0.113 | 0.68 | +0.088 |
| ruff units, short-trump hand | +0.182 | +0.188 | 0.52 | +0.094 |
| ruff units, long-trump hand | +0.240 | +0.262 | 0.68 | +0.163 |
| mirror side suits (shorter hand 3+) | +0.044 | +0.040 | 0.67 | +0.030 |
| wasted HCP (opposite 0-1 cards) | -0.095 | -0.093 | 3.42 | -0.327 |
| fitting K/Q/J (opposite 4+) | +0.086 | +0.089 | 0.98 | +0.084 |
| second 8-card fit | -0.001 | +0.015 | 0.49 | -0.001 |
| side-suit aces | +0.134 | +0.138 | 0.76 | +0.102 |
| side-suit kings | -0.015 | -0.019 | 0.81 | -0.012 |
| HCP imbalance |a-b| | -0.006 | -0.008 | 4.33 | -0.025 |
| tens and nines in 7+ card suits | +0.026 | +0.015 | 1.07 | +0.027 |
| longest side suit - 6 | -0.036 | -0.050 | 0.88 | -0.031 |
| uncovered side suits | -0.280 | -0.276 | 0.49 | -0.137 |
| keycards (RKCB) | +0.108 | +0.101 | 0.85 | +0.092 |
| queens and jacks | +0.021 | +0.020 | 1.29 | +0.027 |
| shorter hand's trump length | +0.302 | +0.307 | 0.83 | +0.250 |

Residual round robin on 30000 held-out random boards (32-board matches, 10 matches, perfect exchange):

| # | Team | Won | VP/match | % optimal vs DD | IMPs lost vs DD/board | team vs Fermat Complete (IMPs/board, 95% CI) | on fit boards | on NT boards |
|---|---|---|---|---|---|---|---|---|
| 1 | All Fives today (Zar / HCP) | 2/4 | 10.69 | 70.2% | -2.367 | -0.192 [-0.223, -0.160] | -0.292 [-0.344, -0.241] | -0.096 [-0.133, -0.060] |
| 2 | FC + residual model (18 features, DIAG) | 2/4 | 10.55 | 74.4% | -2.092 | +0.087 [+0.060, +0.115] | +0.180 [+0.124, +0.236] | +0.000 [+0.000, +0.000] |
| 3 | CEILING two-hand table (DIAG) | 2/4 | 10.49 | 75.9% | -1.962 | +0.243 [+0.204, +0.283] | +0.393 [+0.326, +0.461] | +0.101 [+0.059, +0.143] |
| 4 | FERMAT COMPLETE | 2/4 | 10.15 | 72.9% | -2.185 | — | — | — |
| 5 | Zar / Fifths | 1/4 | 8.13 | 70.6% | -2.349 | -0.170 [-0.200, -0.140] | -0.292 [-0.344, -0.241] | -0.054 [-0.088, -0.021] |
