# H-42 systems check — Fermat Complete inside real auction models

Fermat Complete added as a team to the systems study (`eval_lab systems ... [competitive]`,
`bin/eval_lab.rs::systems`, team "Fermat Complete"): within every auction model the captain
knows its own counts exactly and its partner's only as the auction's range (expected value
given the bucket); suit coverage for the stopper / control terms is assumed known (stopper
and control bids); thresholds tuned on the training deals for each model; tournaments on the
**untouched fresh deals** (seed 2000), 48 teams, 5 000-board matches.

## Head-to-head (IMPs per board, 95% CI)

### Opponents silent
| Match | Perfect exchange | SAYC | Precision | Acol | 2/1 GF | All Fives |
|---|---|---|---|---|---|---|
| Fermat Complete vs Zar / HCP | +0.169 [+0.149,+0.189] | +0.150 [+0.128,+0.172] | +0.127 [+0.106,+0.148] | +0.168 [+0.146,+0.190] | +0.157 [+0.136,+0.178] | +0.146 [+0.125,+0.167] |
| Fermat Complete vs Zar / Fifths | +0.144 [+0.124,+0.164] | +0.142 [+0.121,+0.164] | +0.121 [+0.100,+0.142] | +0.162 [+0.140,+0.184] | +0.132 [+0.111,+0.153] | +0.129 [+0.109,+0.150] |
| Fermat Complete vs Fermat / Fermat NT | +0.132 [+0.117,+0.147] | +0.146 [+0.127,+0.165] | +0.123 [+0.105,+0.141] | +0.170 [+0.151,+0.190] | +0.154 [+0.136,+0.172] | +0.125 [+0.108,+0.142] |

### Opponents bid
| Match | Perfect exchange | SAYC | Precision | Acol | 2/1 GF | All Fives |
|---|---|---|---|---|---|---|
| Fermat Complete vs Zar / HCP | +0.160 [+0.140,+0.180] | +0.152 [+0.130,+0.173] | +0.129 [+0.110,+0.148] | +0.135 [+0.114,+0.157] | +0.157 [+0.136,+0.178] | +0.164 [+0.142,+0.186] |
| Fermat Complete vs Zar / Fifths | +0.133 [+0.113,+0.153] | +0.134 [+0.113,+0.156] | +0.128 [+0.109,+0.147] | +0.129 [+0.108,+0.150] | +0.148 [+0.128,+0.169] | +0.157 [+0.135,+0.179] |
| Fermat Complete vs Fermat / Fermat NT | +0.134 [+0.119,+0.149] | +0.142 [+0.123,+0.161] | +0.133 [+0.116,+0.151] | +0.148 [+0.128,+0.168] | +0.147 [+0.128,+0.166] | +0.153 [+0.132,+0.174] |

## Result

Fermat Complete won every round robin (12 of 12; 48/48 matches each). Against today's All
Fives evaluation (Zar with a fit, HCP in notrump) it gains +0.13 to +0.17 IMP per board in
every system model, +0.164 [+0.142, +0.186] in the All Fives model with opponents bidding.
Full standings: `confirm/h42-systems.md`, `confirm/h42-systems-competitive.md`.

**Limits:** coverage assumed known; double-dummy play; mechanical opponents; the five-level
stop modelled as staying in game. Usability and table pilot still to come.
