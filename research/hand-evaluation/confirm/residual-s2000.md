Residual of Fermat Complete on major-fit sides: tricks = -1.825 + 0.2100 × Fermat fit total (train).
Share of the residual variance the 18 features explain: train 24.2%, test region (own fit) 23.5%.

| Feature | tricks per unit (train) | tricks per unit (test-region fit) | SD (test) | tricks per SD |
|---|---|---|---|---|
| trump fit 9 | +0.348 | +0.328 | 0.46 | +0.159 |
| trump fit 10+ | +0.275 | +0.263 | 0.31 | +0.084 |
| top trump honours (A/K/Q) | +0.130 | +0.116 | 0.69 | +0.089 |
| ruff units, short-trump hand | +0.182 | +0.160 | 0.51 | +0.093 |
| ruff units, long-trump hand | +0.240 | +0.254 | 0.67 | +0.161 |
| mirror side suits (shorter hand 3+) | +0.044 | +0.035 | 0.67 | +0.030 |
| wasted HCP (opposite 0-1 cards) | -0.095 | -0.096 | 3.41 | -0.326 |
| fitting K/Q/J (opposite 4+) | +0.086 | +0.081 | 0.98 | +0.085 |
| second 8-card fit | -0.001 | -0.014 | 0.48 | -0.001 |
| side-suit aces | +0.134 | +0.150 | 0.75 | +0.101 |
| side-suit kings | -0.015 | -0.008 | 0.81 | -0.012 |
| HCP imbalance |a-b| | -0.006 | -0.008 | 4.32 | -0.025 |
| tens and nines in 7+ card suits | +0.026 | +0.025 | 1.07 | +0.028 |
| longest side suit - 6 | -0.036 | -0.024 | 0.87 | -0.031 |
| uncovered side suits | -0.280 | -0.284 | 0.49 | -0.138 |
| keycards (RKCB) | +0.108 | +0.100 | 0.84 | +0.091 |
| queens and jacks | +0.021 | +0.030 | 1.28 | +0.027 |
| shorter hand's trump length | +0.302 | +0.301 | 0.83 | +0.250 |

Residual round robin on 70000 held-out random boards (32-board matches, 10 matches, perfect exchange):

| # | Team | Won | VP/match | % optimal vs DD | IMPs lost vs DD/board | team vs Fermat Complete (IMPs/board, 95% CI) | on fit boards | on NT boards |
|---|---|---|---|---|---|---|---|---|
| 1 | CEILING two-hand table (DIAG) | 4/4 | 12.55 | 76.6% | -1.896 | +0.269 [+0.243, +0.295] | +0.421 [+0.377, +0.466] | +0.124 [+0.097, +0.152] |
| 2 | FERMAT COMPLETE | 3/4 | 10.37 | 73.6% | -2.141 | — | — | — |
| 3 | All Fives today (Zar / HCP) | 2/4 | 10.36 | 71.1% | -2.295 | -0.164 [-0.184, -0.143] | -0.237 [-0.270, -0.204] | -0.094 [-0.118, -0.070] |
| 4 | FC + residual model (18 features, DIAG) | 1/4 | 9.31 | 75.0% | -2.041 | +0.100 [+0.082, +0.118] | +0.206 [+0.168, +0.244] | +0.000 [+0.000, +0.000] |
| 5 | Zar / Fifths | 0/4 | 7.42 | 71.5% | -2.273 | -0.138 [-0.158, -0.119] | -0.237 [-0.270, -0.204] | -0.045 [-0.068, -0.023] |
