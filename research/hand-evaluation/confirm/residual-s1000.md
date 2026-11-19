Residual of Fermat Complete on major-fit sides: tricks = -1.825 + 0.2100 × Fermat fit total (train).
Share of the residual variance the 18 features explain: train 24.2%, test region (own fit) 23.9%.

| Feature | tricks per unit (train) | tricks per unit (test-region fit) | SD (test) | tricks per SD |
|---|---|---|---|---|
| trump fit 9 | +0.348 | +0.317 | 0.46 | +0.158 |
| trump fit 10+ | +0.275 | +0.225 | 0.31 | +0.085 |
| top trump honours (A/K/Q) | +0.130 | +0.146 | 0.68 | +0.088 |
| ruff units, short-trump hand | +0.182 | +0.159 | 0.51 | +0.092 |
| ruff units, long-trump hand | +0.240 | +0.270 | 0.67 | +0.162 |
| mirror side suits (shorter hand 3+) | +0.044 | +0.032 | 0.67 | +0.030 |
| wasted HCP (opposite 0-1 cards) | -0.095 | -0.093 | 3.39 | -0.324 |
| fitting K/Q/J (opposite 4+) | +0.086 | +0.101 | 0.98 | +0.084 |
| second 8-card fit | -0.001 | -0.010 | 0.48 | -0.001 |
| side-suit aces | +0.134 | +0.184 | 0.75 | +0.100 |
| side-suit kings | -0.015 | -0.008 | 0.81 | -0.012 |
| HCP imbalance |a-b| | -0.006 | -0.007 | 4.30 | -0.025 |
| tens and nines in 7+ card suits | +0.026 | +0.014 | 1.07 | +0.028 |
| longest side suit - 6 | -0.036 | -0.022 | 0.87 | -0.031 |
| uncovered side suits | -0.280 | -0.276 | 0.49 | -0.137 |
| keycards (RKCB) | +0.108 | +0.071 | 0.84 | +0.091 |
| queens and jacks | +0.021 | +0.020 | 1.28 | +0.027 |
| shorter hand's trump length | +0.302 | +0.320 | 0.83 | +0.252 |

Residual round robin on 70000 held-out random boards (32-board matches, 10 matches, perfect exchange):

| # | Team | Won | VP/match | % optimal vs DD | IMPs lost vs DD/board | team vs Fermat Complete (IMPs/board, 95% CI) | on fit boards | on NT boards |
|---|---|---|---|---|---|---|---|---|
| 1 | FC + residual model (18 features, DIAG) | 2/4 | 12.26 | 74.6% | -2.066 | +0.108 [+0.090, +0.126] | +0.220 [+0.183, +0.257] | +0.000 [+0.000, +0.000] |
| 2 | FERMAT COMPLETE | 3/4 | 11.31 | 73.1% | -2.171 | — | — | — |
| 3 | CEILING two-hand table (DIAG) | 3/4 | 10.75 | 76.0% | -1.939 | +0.259 [+0.233, +0.285] | +0.444 [+0.400, +0.489] | +0.081 [+0.053, +0.109] |
| 4 | All Fives today (Zar / HCP) | 1/4 | 8.98 | 70.4% | -2.347 | -0.184 [-0.204, -0.164] | -0.257 [-0.290, -0.225] | -0.114 [-0.137, -0.090] |
| 5 | Zar / Fifths | 0/4 | 6.71 | 70.9% | -2.320 | -0.155 [-0.174, -0.135] | -0.257 [-0.290, -0.225] | -0.057 [-0.080, -0.034] |
