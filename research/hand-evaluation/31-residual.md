# What Fermat Complete misses — the residual diagnostic (exploratory)

Method (fixes the H-45/H-46 misreading): on major-fit sides of the 30 000 seed-1000 training
deals, (1) regress DD tricks on the Fermat Complete fit total (tricks = −1.825 + 0.2100 ×
Fermat; ≈ 4.8 Fermat points per trick), then (2) regress the **residual** on 18 named two-hand
features (ridge λ = 1). Every weight below is therefore *on top of* Fermat Complete. Code:
`sim::hand_eval::residual_features` (2 unit tests), `eval_lab residual`. Outputs:
`confirm/residual-s1000.md`, `-s2000`, `-s3000`, `-s4000`.

## Weights (tricks per unit, beyond Fermat Complete)

| Feature | train fit | test-region fit (stability) | tricks per SD | ≈ Fermat points per unit |
|---|---|---|---|---|
| **wasted HCP** (honours opposite partner's 0-1 cards, side suits, aces included) | −0.095 | −0.093 | **−0.324** | −½ per HCP |
| **shorter hand's trump length** | +0.302 | +0.320 | **+0.252** | +1½ per card |
| ruff units, long-trump hand (void 2 / singleton 1, partner 3+) | +0.240 | +0.270 | +0.162 | +1 |
| 9-card trump fit (vs 8) | +0.348 | +0.317 | +0.158 | +1½ |
| uncovered side suits | −0.280 | −0.276 | −0.137 | −1½ |
| side-suit aces | +0.134 | +0.184 | +0.100 | +½ |
| ruff units, short-trump hand | +0.182 | +0.159 | +0.092 | +1 |
| keycards | +0.108 | +0.071 | +0.091 | +½ |
| top trump honours (A/K/Q) | +0.130 | +0.146 | +0.088 | +½ |
| 10+ card fit (vs 8) | +0.275 | +0.225 | +0.085 | +1½ |
| fitting K/Q/J (opposite 4+) | +0.086 | +0.101 | +0.084 | +½ |
| mirror, second fit, side kings, HCP imbalance, tens/nines, long side suit, Q/J | |·| ≤ 0.05 | — | ≤ 0.03 | ≈ 0 |

The 18 features explain about 24% of Fermat Complete's residual variance (train 24.2%, own
test-region fits 23.2-23.9%).

## Value as a team (fitted once on train; IMPs per board vs Fermat Complete)

| Region | FC + residual model | on fit boards | Ceiling (DIAG) |
|---|---|---|---|
| seed 1000 test | **+0.108** [+0.090, +0.126] | +0.220 | +0.259 |
| seed 2000 | **+0.100** [+0.082, +0.118] | +0.206 | +0.269 |
| seed 3000 | **+0.087** [+0.060, +0.115] | +0.180 | +0.243 |
| seed 4000 | **+0.093** [+0.065, +0.121] | +0.190 | +0.265 |

Stable across four regions: an interpretable correction captures about 40% of the ceiling.

## Reading

1. **Wasted values are an HCP effect, aces included.** −½ point per HCP sitting opposite
   partner's singleton or void. H-47 counted −1 per K/Q/J and spared aces opposite a
   singleton — the wrong scale (jack = king) and the wrong scope; that is why it vanished.
2. **Trump quality is about the shorter hand.** Fermat pays length one hand at a time
   (2 × longest suit); tricks depend on the *shorter* trump holding (5-3 < 4-4 < 5-4) and on
   a ninth trump. 
3. **Ruffing value counts — in either hand** when partner has three or more cards there,
   still about +1 point per unit beyond Fermat's shape number.
4. Coverage and controls (uncovered side suits, side aces, keycards) matter at game level
   too, not only in the slam gate.

The residual model itself is a diagnostic (18 fitted weights). A human-usable distillation of
its three largest terms is the candidate for a frozen test on new deals.
