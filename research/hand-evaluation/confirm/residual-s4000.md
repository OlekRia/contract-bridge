Residual of Fermat Complete on major-fit sides: tricks = -1.825 + 0.2100 × Fermat fit total (train).
Share of the residual variance the 18 features explain: train 24.2%, test region (own fit) 23.9%.

| Feature | tricks per unit (train) | tricks per unit (test-region fit) | SD (test) | tricks per SD |
|---|---|---|---|---|
| trump fit 9 | +0.348 | +0.328 | 0.46 | +0.159 |
| trump fit 10+ | +0.275 | +0.229 | 0.31 | +0.086 |
| top trump honours (A/K/Q) | +0.130 | +0.116 | 0.69 | +0.089 |
| ruff units, short-trump hand | +0.182 | +0.142 | 0.51 | +0.093 |
| ruff units, long-trump hand | +0.240 | +0.250 | 0.67 | +0.161 |
| mirror side suits (shorter hand 3+) | +0.044 | +0.037 | 0.67 | +0.029 |
| wasted HCP (opposite 0-1 cards) | -0.095 | -0.092 | 3.42 | -0.326 |
| fitting K/Q/J (opposite 4+) | +0.086 | +0.094 | 0.97 | +0.084 |
| second 8-card fit | -0.001 | -0.011 | 0.48 | -0.001 |
| side-suit aces | +0.134 | +0.130 | 0.75 | +0.100 |
| side-suit kings | -0.015 | -0.001 | 0.81 | -0.012 |
| HCP imbalance |a-b| | -0.006 | -0.005 | 4.29 | -0.025 |
| tens and nines in 7+ card suits | +0.026 | +0.024 | 1.08 | +0.028 |
| longest side suit - 6 | -0.036 | -0.007 | 0.87 | -0.031 |
| uncovered side suits | -0.280 | -0.286 | 0.49 | -0.137 |
| keycards (RKCB) | +0.108 | +0.109 | 0.85 | +0.092 |
| queens and jacks | +0.021 | +0.024 | 1.28 | +0.027 |
| shorter hand's trump length | +0.302 | +0.319 | 0.84 | +0.253 |

Residual round robin on 30000 held-out random boards (32-board matches, 10 matches, perfect exchange):

| # | Team | Won | VP/match | % optimal vs DD | IMPs lost vs DD/board | team vs Fermat Complete (IMPs/board, 95% CI) | on fit boards | on NT boards |
|---|---|---|---|---|---|---|---|---|
| 1 | CEILING two-hand table (DIAG) | 3/4 | 11.90 | 76.4% | -1.915 | +0.265 [+0.225, +0.304] | +0.432 [+0.364, +0.500] | +0.104 [+0.062, +0.146] |
| 2 | Zar / Fifths | 3/4 | 11.79 | 70.8% | -2.325 | -0.176 [-0.207, -0.145] | -0.318 [-0.370, -0.267] | -0.039 [-0.073, -0.005] |
| 3 | FC + residual model (18 features, DIAG) | 2/4 | 9.14 | 74.5% | -2.062 | +0.093 [+0.065, +0.121] | +0.190 [+0.133, +0.247] | +0.000 [+0.000, +0.000] |
| 4 | FERMAT COMPLETE | 2/4 | 9.01 | 73.2% | -2.152 | — | — | — |
| 5 | All Fives today (Zar / HCP) | 0/4 | 8.17 | 70.5% | -2.348 | -0.201 [-0.232, -0.170] | -0.318 [-0.370, -0.267] | -0.089 [-0.125, -0.053] |
