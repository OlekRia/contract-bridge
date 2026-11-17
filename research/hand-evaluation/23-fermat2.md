# H-40 — Fermat-2 test (refuted)

Fermat-2 (`sim::hand_eval::{fermat2_nt, fermat2_fit}`, unit-tested): NT = HCP + ½ per ten −
½ per Q/J in a singleton or doubleton; FIT = HCP + ½ per ten − 1 per K/Q/J in a short side
suit + ½ per K/Q/J in a 4+ side suit + controls + shape number. Ablations: F2-noLong (no
long-side bonus), F2-halfShort (short discount ½).

Exploratory: test deals of seed 1000. **Confirmation: the untouched fresh deals of seed 2000**
(`data/rematch-train30k-plus-fresh70k.jsonl`: thresholds tuned on the first 30 000 deals of
seed 1000, tournaments on the 70 000 fresh deals).

## 85-team SAYC round robin (perfect exchange)

### Exploratory
Head-to-head over all 70000 boards (IMPs per board, 95% CI):

| Match | IMPs/board | 95% CI |
|---|---|---|
| Fermat-2 / Fermat-2 NT vs Fermat / Fermat NT | -0.043 | [-0.057, -0.028] |
| Fermat-2 / Fifths vs Fermat / Fifths | -0.045 | [-0.057, -0.033] |
| Fermat-2 / Fifths vs Zar / Fifths | -0.016 | [-0.031, -0.001] |
| Fermat-2 / Fermat-2 NT vs Zar / Fifths | -0.029 | [-0.046, -0.012] |
| Fermat / Fermat-2 NT vs Fermat / Fermat NT | +0.002 | [-0.005, +0.010] |
| Fermat-2 / Fermat-2 NT vs F2-noLong / Fermat-2 NT | -0.034 | [-0.046, -0.022] |
| Fermat-2 / Fermat-2 NT vs F2-halfShort / Fermat-2 NT | -0.001 | [-0.013, +0.010] |

### Confirmation (fresh deals)
Head-to-head over all 70000 boards (IMPs per board, 95% CI):

| Match | IMPs/board | 95% CI |
|---|---|---|
| Fermat-2 / Fermat-2 NT vs Fermat / Fermat NT | -0.042 | [-0.056, -0.028] |
| Fermat-2 / Fifths vs Fermat / Fifths | -0.048 | [-0.060, -0.036] |
| Fermat-2 / Fifths vs Zar / Fifths | -0.017 | [-0.032, -0.002] |
| Fermat-2 / Fermat-2 NT vs Zar / Fifths | -0.030 | [-0.047, -0.013] |
| Fermat / Fermat-2 NT vs Fermat / Fermat NT | +0.006 | [-0.002, +0.014] |
| Fermat-2 / Fermat-2 NT vs F2-noLong / Fermat-2 NT | -0.042 | [-0.053, -0.030] |
| Fermat-2 / Fermat-2 NT vs F2-halfShort / Fermat-2 NT | -0.001 | [-0.013, +0.011] |

Top 5 on the fresh deals:

| # | Team (fit lens / NT lens) | W-D-L | VP/match | IMPs/board vs field | % optimal vs DD | IMPs lost vs DD/board |
|---|---|---|---|---|---|---|
| 1 | F2-noLong / Fifths | 80-0-4 | 16.90 | +0.184 | 71.9% | -2.253 |
| 2 | Fermat / Fifths | 82-0-2 | 16.81 | +0.183 | 72.0% | -2.249 |
| 3 | F2-noLong / Fermat-2 NT | 77-0-7 | 16.59 | +0.176 | 72.0% | -2.261 |
| 4 | Fermat / Fermat-2 NT | 79-0-5 | 16.51 | +0.175 | 72.1% | -2.258 |
| 5 | Fermat / HCP | 75-0-9 | 16.19 | +0.167 | 71.6% | -2.271 |

## Slam decisions (EC-6 test deals, seed 512)

| | Vulnerable | Non-vulnerable |
|---|---|---|
| Fermat-2 fit vs Fermat fit | −0.104 [−0.147, −0.062] | −0.087 [−0.123, −0.051] |
| Fermat-2 fit vs Zar | +0.030 [−0.021, +0.082] | +0.026 [−0.017, +0.069] |

## Bidding systems (fresh deals)

### Silent opponents
| Match | Perfect exchange | SAYC | Precision | Acol | 2/1 GF | All Fives |
|---|---|---|---|---|---|---|
| Fermat-2 / Fermat-2 NT vs Fermat / Fermat NT | -0.042 [-0.056,-0.028] | -0.007 [-0.019,+0.005] | -0.004 [-0.018,+0.009] | -0.011 [-0.023,+0.002] | +0.001 [-0.010,+0.012] | -0.010 [-0.022,+0.003] |
| Fermat-2 / Fifths vs Fermat / Fifths | -0.048 [-0.060,-0.036] | -0.015 [-0.025,-0.005] | -0.002 [-0.014,+0.009] | -0.014 [-0.024,-0.004] | -0.002 [-0.012,+0.007] | -0.013 [-0.024,-0.003] |
| Fermat-2 / Fermat-2 NT vs Zar / Fifths | -0.030 [-0.047,-0.013] | -0.007 [-0.021,+0.007] | -0.006 [-0.022,+0.010] | -0.015 [-0.029,-0.002] | -0.020 [-0.035,-0.005] | -0.005 [-0.020,+0.009] |
| Fermat-2 / Fifths vs Zar / Fifths | -0.017 [-0.032,-0.002] | -0.011 [-0.022,+0.001] | +0.005 [-0.009,+0.019] | -0.014 [-0.025,-0.003] | -0.002 [-0.014,+0.009] | +0.003 [-0.009,+0.014] |
| Fermat fit vs Zar | +0.031 [+0.021,+0.041] | +0.005 [-0.005,+0.014] | +0.007 [-0.005,+0.019] | -0.000 [-0.010,+0.009] | +0.000 [-0.009,+0.010] | +0.016 [+0.005,+0.027] |

### Opponents bid
| Match | Perfect exchange | SAYC | Precision | Acol | 2/1 GF | All Fives |
|---|---|---|---|---|---|---|
| Fermat-2 / Fermat-2 NT vs Fermat / Fermat NT | -0.032 [-0.045,-0.019] | +0.021 [+0.010,+0.032] | -0.014 [-0.025,-0.002] | +0.032 [+0.019,+0.045] | +0.010 [-0.000,+0.020] | -0.004 [-0.015,+0.008] |
| Fermat-2 / Fifths vs Fermat / Fifths | -0.043 [-0.055,-0.032] | +0.015 [+0.006,+0.025] | -0.011 [-0.021,-0.001] | +0.034 [+0.022,+0.045] | +0.007 [-0.002,+0.016] | +0.004 [-0.006,+0.013] |
| Fermat-2 / Fermat-2 NT vs Zar / Fifths | -0.033 [-0.050,-0.016] | +0.012 [-0.002,+0.027] | -0.019 [-0.033,-0.005] | +0.006 [-0.008,+0.020] | +0.012 [-0.002,+0.026] | +0.004 [-0.010,+0.018] |
| Fermat-2 / Fifths vs Zar / Fifths | -0.018 [-0.032,-0.004] | +0.024 [+0.012,+0.035] | -0.004 [-0.016,+0.007] | +0.015 [+0.004,+0.026] | +0.018 [+0.007,+0.029] | +0.013 [+0.002,+0.024] |
| Fermat fit vs Zar | +0.025 [+0.015,+0.036] | +0.009 [-0.002,+0.019] | +0.007 [-0.002,+0.016] | -0.018 [-0.028,-0.009] | +0.011 [+0.001,+0.020] | +0.009 [+0.001,+0.017] |
