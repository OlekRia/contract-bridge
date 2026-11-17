# Results — packet 1

Labels: **COMPUTATIONAL RESULT** (DDS proxy; clairvoyant play, no lead model) unless
stated. IMPs are vulnerable unless marked NV. "vs HCP" = mean paired IMPs/deal on held-out
deals against the HCP-threshold policy, 95% CI (deals i.i.d., one record per deal).

## 1. Development results (exploration; dev seeds 101 / 201, 10k train / 10k test halves)

### EC-1 — responder after 1NT 12-14 (balanced, no 4M): Pass vs 3NT

| Evaluator | V vs HCP | NV vs HCP | r(tricks) |
|---|---|---|---|
| Fifths (Andrews) | **+0.043 [+0.009,+0.078]** | −0.002 [−0.025,+0.021] | 0.665 |
| LW learned rank weights | +0.036 [+0.004,+0.069] | +0.003 | 0.666 |
| RxL rank × length | +0.038 [−0.001,+0.077] | −0.002 | 0.667 |
| HCP+length | +0.014 | −0.050 | 0.644 |
| SHV holding table | −0.010 | −0.020 | 0.662 |
| BUM-RAP | −0.028 | −0.028 | 0.654 |
| Kaplan–Rubens (Pavlicek) | −0.075 [−0.125,−0.025] | — | 0.634 |
| Zar (out of purpose) | −0.135 | −0.105 | 0.602 |
| NLTC (out of purpose) | −0.183 | −0.138 | 0.591 |
| Classic LTC (out of purpose) | −0.602 | −0.313 | 0.450 |
| DIAG combined HCP (S3) | +0.119 | +0.086 | 0.707 |
| DIAG partnership holding table (S3) | +0.301 | +0.195 | 0.771 |
| DIAG DD-perfect (S4) | +1.656 | +1.099 | — |

Learned NT weights (A..9 count, tricks per card): A 1.98, K 1.43, Q 0.85, J 0.47, T 0.15,
9 0.04 → scaled A=4: **K 2.89, Q 1.72, J 0.94, T 0.31** — an independent re-derivation of
Andrews' Fifths (4 / 2.8 / 1.8 / 1.0 / 0.4) and Mossakowski–Mańdziuk's network weights.

### EC-2 — opener after 1M–2M (single raise 6-9, 3+): Pass vs 4M

| Evaluator | V vs HCP | NV vs HCP | r(tricks) |
|---|---|---|---|
| SHV holding table (Binky-style, λ=5) | **+0.513 [+0.434,+0.593]** | +0.351 | 0.592 |
| ZX Zar + placement (fitted) | +0.472 | +0.334 | 0.577 |
| Zar | **+0.459 [+0.395,+0.522]** | +0.324 | 0.569 |
| DST 6-feature fitted | +0.434 | — | 0.581 |
| VEC HCP/LTC/controls/length/shortness | +0.419 | +0.336 | 0.570 |
| RxL rank × length (+trump bucket) | +0.405 | +0.311 | 0.577 |
| LW rank weights | +0.387 | +0.316 | 0.567 |
| Kaplan–Rubens (Pavlicek) | +0.358 | +0.267 | 0.556 |
| HCP+length+shortness | +0.339 | +0.252 | 0.542 |
| NLTC | +0.304 | +0.295 | 0.551 |
| HCP+length | +0.148 | +0.126 | 0.494 |
| Fifths | +0.039 | +0.012 | 0.437 |
| BUM-RAP | +0.019 | +0.021 | 0.447 |
| **Classic LTC** | **−0.017 [−0.105,+0.071]** | −0.010 | 0.443 |
| DIAG combined HCP (S3) | +0.095 | +0.150 | 0.506 |
| DIAG partnership table (S3) | +0.925 | +0.621 | 0.714 |
| DIAG DD-perfect (S4) | +2.659 | +1.868 | — |

Cross-fold check (train/test halves swapped): Zar +0.453, ZX +0.471, SHV +0.507 (V).

### EC-3 — integrated invite policies (dev, V; tuned on train half)

| Policy (opener eval / responder eval) | vs HCP direct |
|---|---|
| HCP / HCP, direct | 0 |
| HCP / HCP, **invite (3M)** | **−0.063 [−0.132,+0.007]** |
| Zar / Zar, direct | +0.459 |
| Zar / Zar, invite | +0.490 |
| SHV / SHV, direct | +0.511 |
| **SHV / SHV, invite** | **+0.589 [+0.509,+0.668]** |
| SHV / SHV-help-suit, invite | +0.580 |

NT (EC-1 with 2NT invite allowed): the tuned policy almost never invites (HCP 0%,
Fifths 5%, SHV 8% of deals); invite gains ≤ +0.006 over direct.

### HE-09 — strain transfer (same opener hands; table fitted to NT tricks vs trump tricks)

r between the two fitted value functions on the same hands = **0.69**. Using the NT-fitted
value for the 4M decision: **−0.561 [−0.640,−0.481] IMP/deal** vs the in-context table
(V), −0.336 (NV) — no better than HCP. (First attempt, NT responder table → opener hands,
was confounded by hand population and is superseded.)

## 2. Confirmation (held-out seeds 112 / 212, 40 000 deals each; frozen plan `05`)

Training = full 20k dev set; one run per vulnerability; raw output in `confirm/`
(`compare-*.txt`, `invite-*.txt`, `transfer-*.txt`, per-deal `paired-*.csv`).
Head-to-head CIs computed from the paired CSVs (exact per-deal IMP differences).

### Primary endpoints

| ID | Comparison (vul) | Result | 95% CI | Predeclared verdict |
|---|---|---|---|---|
| P1 | Fifths vs HCP (EC-1) | **+0.043** | [+0.026, +0.060] | **CONFIRMED** (≥ +0.02) |
| P2 | Classic LTC vs HCP (EC-2; F-LTC-004) | +0.035 | [−0.009, +0.079] | **INCONCLUSIVE** — F-LTC-004's advantage not demonstrated; at most +0.08 |
| P3 | Zar vs classic LTC (EC-2) | **+0.394** | [+0.352, +0.436] | **CONFIRMED** (≥ +0.20) |
| P4 | SHV/SHV invite vs Zar/Zar direct (EC-2) | **+0.228** | [+0.198, +0.258] | **CONFIRMED** (≥ +0.05) |

### Secondary endpoints (Bonferroni ×6 noted: 99.2% ≈ ±2.64 SE)

| ID | Comparison | Vul | NV | Note |
|---|---|---|---|---|
| S1 | SHV table vs Zar (EC-2) | **+0.069 [+0.043, +0.096]** | +0.058 [+0.040, +0.077] | survives Bonferroni (V lower ≈ +0.033) |
| S2 | ZX (Zar + placement) vs Zar | +0.012 [−0.005, +0.029] | +0.028 [+0.015, +0.040] | inconclusive V; small NV |
| S3 | KnR vs HCP (EC-1, NT) | −0.041 [−0.067, −0.015] | −0.055 [−0.071, −0.039] | prediction (≤ 0) confirmed; replicates Andrews |
| S4 | HCP invite vs HCP direct (EC-2) | 0.000 (tuner chose never to invite) | **+0.025 [+0.012, +0.037]** | prediction (≤ 0) **refuted NV** (small gain) |
| S5 | SHV help-suit invite vs SHV invite | −0.001 | +0.010 | no value from naming the try suit (with this model) |
| S6 | NV replication: P1 | +0.006 [−0.003, +0.015] | — | Fifths gain is vulnerable-only |
| S6 | NV replication: P3 Zar vs LTC | — | +0.315 [+0.285, +0.346] | replicated |
| S6 | NV replication: P4 | — | +0.147 (vs Zar direct) | replicated |
| S7 | HE-09 transferred vs in-context table | **−0.436 [−0.474, −0.398]** | −0.317 [−0.343, −0.292] | prediction (loss ≥ 0.20) confirmed |

### Other held-out facts worth recording

- EC-2 (vul) vs HCP: SHV +0.499, DST 6-feature +0.464, RxL +0.463, VEC +0.438,
  ZX +0.442, Zar +0.429, LW +0.423, KnR +0.414, NLTC +0.366, HCP+len+short +0.345,
  HCP+len +0.211, BUM-RAP +0.077, Fifths +0.064, **classic LTC +0.035**.
  NLTC vs classic LTC +0.331 [+0.300, +0.363]; KnR vs Zar −0.015 [−0.043, +0.014].
- Human-usable integrated policy: **Zar/Zar invite vs Zar direct +0.100 [+0.072, +0.128]**
  (V), +0.081 [+0.070, +0.092] (NV). Tuned thresholds — V: opener 31-32 invites, 33+ game,
  responder accepts with Zar ≥ 20; NV: opener 32-33 invites, 34+ game, accepts ≥ 19.
- EC-1 (NT): no learned table beats Fifths — SHV vs Fifths +0.001 [−0.015, +0.016] (V),
  −0.010 [−0.020, +0.001] (NV): **practically equivalent** (CI inside ±0.02).
  2NT invites add ≤ +0.017 over direct (SHV), +0.004 (Fifths).
- DIAG: partnership-holding table (S3 information) +0.96 vs HCP in EC-2 and +0.32 in EC-1 —
  roughly double the best single-hand evaluator; DD-perfect +2.72 / +1.65.
- S2 Monte-Carlo ceiling (dev, 1 500 hands, k = 48, EC-2 vul): vs Zar +0.003
  [−0.140, +0.145]; vs HCP +0.381 [+0.195, +0.567]; vs classic LTC +0.521. Zar is
  already at (or within noise of) the own-hand ceiling for the direct decision.

### Frequency (per random board)

The EC-2 auction (either of us opens, partner raises, opponents silent) arises on
≈ 3.7% of boards; EC-1 (1NT 12-14 with a balanced no-major response) on ≈ 2.2%.
So P3 (+0.39) ≈ **+0.015 IMP/board**, P4 (+0.23 over Zar direct, ≈ +0.62 over LTC
direct) ≈ +0.023 IMP/board, P1 ≈ +0.001 IMP/board. These are small per board; they are
large per qualifying decision.

### Interpretation

- **Within one strain, published counts are near the ceiling**: Fifths in NT and Zar
  after a major raise are matched or beaten only by small margins (SHV +0.07 in EC-2,
  nothing in EC-1), and the S2 Monte-Carlo policy does not beat Zar.
- **Classic LTC is the weak link of today's agreements** in its own home scenario: it
  is indistinguishable from HCP and 0.39 IMP/decision worse than Zar. NLTC (which
  charges a missing ace 1.5 losers) recovers most of the gap — the mechanism is aces.
- **Value moves with the strain** (S7) and **with what partner holds** (DIAG), so the
  practical frontier is exchanging information with good evaluations on both sides (P4).

## 3. Failure atlas (packet 1)

| # | Case | Who is wrong | Mechanism | Evidence |
|---|---|---|---|---|
| F1 | `KQ953 KQ2 QJ32 6` vs `AJ953 A42 A732 6` after 1♠–2♠ | Classic LTC (ranks the K-Q-J hand better: 5 vs 7 losers) | LTC scores KQx = 1 loser, Axx = 2: blind to aces/tempo | P(make) 0.155 vs 0.41 (MC 200) |
| F2 | `AQ852 K KJ43 872` vs `AQ852 7 KJ43 K72` | HCP, Zar (identical values) | singleton honour half-wasted | 0.125 vs 0.32 |
| F3 | `AKQ74 A632 84 52` vs `97542 AKQ3 A4 52` | Kaplan–Rubens (+1.7 for trump texture) | no trump-texture premium opposite known 3+ support | 0.165 vs 0.20 |
| F4 | `QT9 KT8 KJT9 Q62` vs `Q72 K83 KJ54 Q62` after 1NT | HCP, Zar | intermediates | 0.595 vs 0.27 |
| F5 | `QJ2 QJ3 QJ54 QJ2` after 1NT | Kaplan–Rubens (7.40) | KnR's queen/jack discounts over-apply in NT | P 0.415, game right |
| F6 | `AQ852 KJ4 Q73 K2` (15 HCP 5332) after 1♠–2♠ | HCP (says game) | flat shape, no ruffing value | P 0.26 |
| F7 | `KQJ853 AQ872 4 2` (12 HCP 6-5) | HCP (says pass) | two-suiter trick sources | P 0.755 |
| F8 | Generic invitation evaluated by HCP | the *policy* | an invitation is only as good as both evaluations | dev −0.063; held-out V: tuner refuses to invite; NV +0.025 (vs +0.10 with Zar) |
| F9 | NT-fitted value used for a suit decision | any single scalar | strain-relative value | −0.56 IMP/deal |
| F10 | T12 B16 (H-31 premise) | the post-mortem, not the bidding | DD shows 9 tricks in ♠: no missed game | `02-anomaly-audit.md` |

Baseline wins recorded: Fifths ≥ every learned table in NT (dev); Zar ≈ every fitted
small model in EC-2; the help-suit try did not beat a generic invite.
