# H-39 — Fermat Points test

Fermat Points (`sim::hand_eval::{fermat_base, fermat_fit}`, unit-tested):
BASE = HCP + ½ per ten − ½ per K, Q or J in a singleton or doubleton (notrump lens);
FIT = BASE + controls (A=2, K=1) + shape number (2 × longest + second − shortest).
Ablations: without the ten bonus (noT), without the short-honour discount (noS).
Same data and protocol as `19-systems.md` / `20-competitive.md`, now with 46 teams.
Commands: `eval_lab systems data/battle-100k-s1000.jsonl 0.3 5000 out.md [competitive]`.

## Silent opponents


| Fermat / Fermat NT vs | Perfect exchange | SAYC | Precision | Acol | 2/1 GF | All Fives |
|---|---|---|---|---|---|---|
| Zar / Fifths | +0.013 [-0.001,+0.028] | +0.001 [-0.011,+0.014] | +0.002 [-0.013,+0.017] | -0.002 [-0.015,+0.012] | -0.004 [-0.017,+0.009] | +0.014 [+0.000,+0.027] |
| Zar / HCP | +0.039 [+0.024,+0.054] | +0.003 [-0.011,+0.017] | +0.015 [-0.000,+0.031] | +0.005 [-0.009,+0.020] | +0.015 [+0.001,+0.029] | +0.023 [+0.009,+0.038] |
| Kaplan-Rubens / Fifths | +0.006 [-0.014,+0.025] | -0.005 [-0.021,+0.011] | +0.011 [-0.005,+0.027] | -0.008 [-0.026,+0.010] | -0.012 [-0.029,+0.005] | +0.014 [-0.002,+0.030] |
| HCP / HCP | +0.253 [+0.226,+0.279] | +0.130 [+0.107,+0.153] | +0.129 [+0.106,+0.153] | +0.116 [+0.093,+0.139] | +0.122 [+0.099,+0.145] | +0.152 [+0.128,+0.175] |
| Fermat-noT / Fermat NT-noT | +0.019 [+0.006,+0.032] | +0.007 [-0.004,+0.019] | +0.003 [-0.009,+0.016] | +0.004 [-0.008,+0.017] | +0.026 [+0.014,+0.038] | +0.019 [+0.007,+0.031] |
| Fermat-noS / Fermat NT-noS | +0.027 [+0.014,+0.040] | -0.010 [-0.021,+0.001] | +0.004 [-0.009,+0.016] | -0.005 [-0.017,+0.007] | -0.009 [-0.021,+0.003] | +0.010 [-0.002,+0.022] |

## Opponents bid (competitive model)

| Fermat / Fermat NT vs | Perfect exchange | SAYC | Precision | Acol | 2/1 GF | All Fives |
|---|---|---|---|---|---|---|
| Zar / Fifths | +0.002 [-0.013,+0.017] | -0.015 [-0.029,-0.001] | -0.013 [-0.026,+0.000] | -0.023 [-0.036,-0.011] | -0.010 [-0.023,+0.003] | +0.013 [+0.001,+0.025] |
| Zar / HCP | +0.027 [+0.012,+0.042] | -0.011 [-0.025,+0.004] | -0.006 [-0.020,+0.008] | -0.026 [-0.039,-0.013] | -0.009 [-0.024,+0.005] | +0.010 [-0.004,+0.024] |
| Kaplan-Rubens / Fifths | +0.009 [-0.010,+0.029] | +0.025 [+0.010,+0.040] | +0.004 [-0.012,+0.020] | -0.009 [-0.023,+0.005] | +0.027 [+0.013,+0.042] | +0.040 [+0.026,+0.055] |
| HCP / HCP | +0.209 [+0.182,+0.236] | +0.091 [+0.069,+0.113] | +0.078 [+0.056,+0.101] | +0.075 [+0.055,+0.096] | +0.087 [+0.065,+0.108] | +0.069 [+0.048,+0.090] |
| Fermat-noT / Fermat NT-noT | +0.006 [-0.007,+0.020] | -0.010 [-0.023,+0.003] | -0.017 [-0.031,-0.004] | -0.002 [-0.014,+0.010] | -0.007 [-0.020,+0.007] | +0.009 [-0.004,+0.021] |
| Fermat-noS / Fermat NT-noS | +0.015 [+0.002,+0.029] | -0.001 [-0.011,+0.009] | +0.012 [-0.001,+0.026] | -0.033 [-0.045,-0.021] | -0.022 [-0.035,-0.009] | +0.010 [-0.001,+0.021] |

Full standings: `confirm/h39-systems.md`, `confirm/h39-systems-competitive.md`.

## Fermat fit lens vs Zar (Director request)

Fermat fit = Zar + ½ per ten − ½ per K, Q or J in a singleton or doubleton.

**Fixed decisions** (held-out deals; IMPs per decision; `eval_lab compare`):

| Decision | Vulnerable | Non-vulnerable |
|---|---|---|
| Slam with a major fit (partnership totals) | **+0.135 [+0.093, +0.176]** | **+0.113 [+0.079, +0.148]** |
| Opener after 1M–2M | +0.012 [−0.007, +0.031] | +0.013 [−0.003, +0.028] |
| Responder with 4+ trumps (2M or 4M) | +0.016 [+0.005, +0.027] | −0.013 [−0.025, −0.001] |

**Tournaments, fit lens only** (both teams use Fifths in notrump; IMPs per board):

| | Perfect | SAYC | Precision | Acol | 2/1 GF | All Fives |
|---|---|---|---|---|---|---|
| Silent opponents | +0.019* | −0.003 | +0.005 | −0.021* | −0.003 | +0.011* |
| Opponents bid | +0.029* | +0.011* | +0.012 | +0.009 | +0.015* | +0.022* |

\* 95% CI excludes zero. Full intervals in `confirm/h39-systems*.md`.
