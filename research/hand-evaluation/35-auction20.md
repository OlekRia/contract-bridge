# 20-team tournament without seeing partner's cards (All Fives auction model)

Runner `eval_lab auction20 <data> <train frac> 5000 <out> [competitive]`. Each team bids **without seeing partner's hand**: the captain knows his own thirteen cards and partner's count only as the average of partner's bidding range in the All Fives auction model (`exchange_seated`, the H-42 information model); stoppers and keycards are shown by control bids. The Fermat two-hand terms (shortness, wasted honours) are known only in the game/slam zone (Fermat estimate ≥ 50), where splinters and shortness tries show them; the trump-quality term of Fit-Quality is known from the raise structure. Methods needing both hands at once were replaced by their one-hand versions (OHE per hand; Fermat weakest-link two-lens → Fermat / Fifths). Deals: the 30 000 brand-new seed-6000 boards; lines tuned on the 30 000 seed-1000 training deals under the same model. 190 matches of 5 000 boards, WBF VPs.

Outputs: `confirm/auction20-s6000-silent.md`, `confirm/auction20-s6000-competitive.md` (each also shows the perfect-exchange reference).

## Silent opponents

| # | Team | Won | VP/match | IMPs lost vs DD/board | vs Fermat Complete | fit boards | NT boards |
|---|---|---|---|---|---|---|---|
| 1 | Fit-Quality L2 (+ shortness in game zone) | 19/19 | 18.85 | -2.333 | +0.037 [+0.009, +0.065] | +0.076 [+0.018, +0.134] | +0.000 [+0.000, +0.000] |
| 2 | Fermat + partnership shortness (H-46, shown in game zone) | 18/19 | 18.19 | -2.355 | +0.008 [-0.012, +0.027] | +0.016 [-0.024, +0.056] | +0.000 [+0.000, +0.000] |
| 3 | Fermat Complete | 17/19 | 17.75 | -2.363 | — | — | — |
| 4 | Fermat + wasted honours (H-47, shortness shown in game zone) | 16/19 | 17.30 | -2.377 | -0.018 [-0.038, +0.003] | -0.036 [-0.078, +0.006] | +0.000 [+0.000, +0.000] |
| 5 | Kaplan-Rubens / Fifths | 14/19 | 13.73 | -2.492 | -0.148 [-0.180, -0.115] | -0.209 [-0.266, -0.152] | -0.089 [-0.123, -0.056] |
| 6 | Fermat / Fermat NT | 13/19 | 13.25 | -2.503 | -0.156 [-0.182, -0.129] | -0.206 [-0.252, -0.160] | -0.108 [-0.136, -0.080] |
| 7 | Fermat / Fifths | 14/19 | 13.02 | -2.495 | -0.146 [-0.174, -0.118] | -0.206 [-0.252, -0.160] | -0.089 [-0.123, -0.056] |
| 8 | Zar / HCP | 12/19 | 13.01 | -2.496 | -0.159 [-0.191, -0.127] | -0.218 [-0.272, -0.165] | -0.102 [-0.138, -0.066] |
| 9 | Zar / Fifths | 12/19 | 12.66 | -2.492 | -0.152 [-0.183, -0.121] | -0.218 [-0.272, -0.165] | -0.089 [-0.123, -0.056] |
| 10 | NLTC / Fifths | 10/19 | 10.75 | -2.551 | -0.204 [-0.239, -0.169] | -0.325 [-0.389, -0.262] | -0.089 [-0.123, -0.056] |
| 11 | HCP+len / HCP+len | 9/19 | 9.13 | -2.573 | -0.230 [-0.269, -0.191] | -0.293 [-0.354, -0.232] | -0.171 [-0.220, -0.122] |
| 12 | HCP / HCP | 7/19 | 6.93 | -2.618 | -0.295 [-0.335, -0.256] | -0.500 [-0.571, -0.428] | -0.102 [-0.138, -0.066] |
| 13 | BUM-RAP / BUM-RAP | 6/19 | 6.56 | -2.620 | -0.288 [-0.327, -0.249] | -0.489 [-0.558, -0.420] | -0.097 [-0.135, -0.058] |
| 14 | Zar published (52/62/67) / HCP 25 | 5/19 | 6.03 | -2.651 | -0.317 [-0.353, -0.281] | -0.267 [-0.314, -0.219] | -0.365 [-0.418, -0.312] |
| 15 | OHE (Darricades, per hand) | 7/19 | 5.67 | -2.646 | -0.312 [-0.352, -0.272] | -0.362 [-0.426, -0.299] | -0.265 [-0.315, -0.214] |
| 16 | Fifths / Fifths | 4/19 | 5.62 | -2.628 | -0.299 [-0.337, -0.260] | -0.520 [-0.591, -0.449] | -0.089 [-0.123, -0.056] |
| 17 | Classic LTC / HCP | 3/19 | 4.98 | -2.665 | -0.333 [-0.373, -0.293] | -0.577 [-0.650, -0.504] | -0.102 [-0.138, -0.066] |
| 18 | Kaplan-Rubens / Kaplan-Rubens | 3/19 | 4.37 | -2.689 | -0.357 [-0.397, -0.317] | -0.209 [-0.266, -0.152] | -0.497 [-0.553, -0.441] |
| 19 | HCP+len+short / HCP+len+short | 1/19 | 2.20 | -2.768 | -0.450 [-0.492, -0.409] | -0.263 [-0.322, -0.204] | -0.627 [-0.685, -0.569] |
| 20 | SAYC classic (HCP 25/33/37) | 0/19 | 0.00 | -3.239 | -1.007 [-1.061, -0.953] | -1.685 [-1.779, -1.591] | -0.365 [-0.418, -0.312] |

## Opponents compete (interference, LoTT partscore battles, sacrifices)

| # | Team | Won | VP/match | IMPs lost vs DD/board | vs Fermat Complete | fit boards | NT boards |
|---|---|---|---|---|---|---|---|
| 1 | Fermat + partnership shortness (H-46, shown in game zone) | 18/19 | 18.46 | -2.565 | +0.014 [-0.003, +0.032] | +0.030 [-0.007, +0.066] | +0.000 [+0.000, +0.000] |
| 2 | Fermat Complete | 17/19 | 18.19 | -2.580 | — | — | — |
| 3 | Fermat + wasted honours (H-47, shortness shown in game zone) | 17/19 | 17.89 | -2.595 | -0.015 [-0.033, +0.003] | -0.030 [-0.067, +0.006] | +0.000 [+0.000, +0.000] |
| 4 | Fit-Quality L2 (+ shortness in game zone) | 18/19 | 17.39 | -2.604 | -0.026 [-0.052, -0.000] | -0.053 [-0.106, -0.000] | +0.000 [+0.000, +0.000] |
| 5 | Fermat / Fermat NT | 13/19 | 12.01 | -2.739 | -0.187 [-0.220, -0.155] | -0.264 [-0.317, -0.212] | -0.114 [-0.153, -0.075] |
| 6 | Fermat / Fifths | 13/19 | 11.88 | -2.740 | -0.187 [-0.219, -0.154] | -0.264 [-0.317, -0.212] | -0.113 [-0.152, -0.075] |
| 7 | Zar / HCP | 11/19 | 11.88 | -2.738 | -0.185 [-0.218, -0.151] | -0.269 [-0.326, -0.213] | -0.105 [-0.142, -0.068] |
| 8 | Zar / Fifths | 10/19 | 11.58 | -2.741 | -0.189 [-0.223, -0.155] | -0.269 [-0.326, -0.213] | -0.113 [-0.152, -0.075] |
| 9 | NLTC / Fifths | 13/19 | 11.40 | -2.755 | -0.189 [-0.225, -0.152] | -0.268 [-0.330, -0.206] | -0.113 [-0.152, -0.075] |
| 10 | Kaplan-Rubens / Fifths | 14/19 | 11.39 | -2.744 | -0.192 [-0.227, -0.156] | -0.274 [-0.334, -0.214] | -0.113 [-0.152, -0.075] |
| 11 | HCP+len / HCP+len | 9/19 | 9.33 | -2.791 | -0.233 [-0.272, -0.195] | -0.286 [-0.346, -0.226] | -0.183 [-0.232, -0.134] |
| 12 | Classic LTC / HCP | 9/19 | 7.88 | -2.819 | -0.254 [-0.293, -0.215] | -0.411 [-0.481, -0.341] | -0.105 [-0.142, -0.068] |
| 13 | HCP / HCP | 7/19 | 7.49 | -2.802 | -0.243 [-0.281, -0.206] | -0.389 [-0.456, -0.323] | -0.105 [-0.142, -0.068] |
| 14 | BUM-RAP / BUM-RAP | 5/19 | 6.81 | -2.818 | -0.267 [-0.305, -0.229] | -0.447 [-0.512, -0.381] | -0.097 [-0.138, -0.056] |
| 15 | Fifths / Fifths | 5/19 | 6.61 | -2.822 | -0.274 [-0.312, -0.236] | -0.443 [-0.510, -0.376] | -0.113 [-0.152, -0.075] |
| 16 | OHE (Darricades, per hand) | 4/19 | 6.41 | -2.840 | -0.271 [-0.311, -0.232] | -0.292 [-0.354, -0.231] | -0.251 [-0.302, -0.200] |
| 17 | Zar published (52/62/67) / HCP 25 | 4/19 | 6.27 | -2.850 | -0.300 [-0.335, -0.264] | -0.300 [-0.349, -0.250] | -0.300 [-0.351, -0.249] |
| 18 | Kaplan-Rubens / Kaplan-Rubens | 1/19 | 4.06 | -2.883 | -0.334 [-0.375, -0.294] | -0.274 [-0.334, -0.214] | -0.392 [-0.447, -0.337] |
| 19 | HCP+len+short / HCP+len+short | 2/19 | 3.08 | -2.931 | -0.378 [-0.419, -0.337] | -0.257 [-0.316, -0.198] | -0.492 [-0.549, -0.435] |
| 20 | SAYC classic (HCP 25/33/37) | 0/19 | 0.00 | -3.335 | -0.879 [-0.933, -0.825] | -1.490 [-1.585, -1.395] | -0.300 [-0.351, -0.249] |

## Reading

- **Silent opponents: Fermat Fit-Quality wins 19/19** (18.85 VP/match), +0.037 IMP/board over Fermat Complete [+0.009, +0.065] — smaller than with both hands seen (+0.10), but real.
- **Opponents compete: the Fermat family still takes places 1-4, but Fit-Quality drops to 4th** (−0.026 [−0.052, −0.000] vs Fermat Complete); Fermat + shortness (H-46) edges first on VPs (+0.014, CI crosses 0). Fermat Complete is the robust choice: 2nd and 3rd, never behind a non-Fermat team.
- **Fermat Complete vs today's All Fives evaluation (Zar / HCP): +0.159 [+0.127, +0.191] silent, +0.185 [+0.151, +0.218] competitive** — the gain survives bidding without seeing partner's cards and grows when opponents compete.
- SAYC classic (HCP 25/33/37) loses all 19 matches under both conditions.
- Limits: double-dummy play; the auction model is a range-information model, not a bid-by-bid simulation; competitive model as in `20-competitive.md` (known biases, H-37).
