# Hypothesis Cards — Beyond Points (packet 1, 2026-11-17)

Local research IDs (`HE-nn`), **not** official H-IDs. Nothing here changes a table
agreement. Information states: **S1** own hand pre-auction; **S2** own hand + legal
auction; **S3** both partnership hands (diagnosis only); **S4** all four hands (oracle).
Every table-usable evaluator below is S1/S2; anything using S3/S4 is labelled DIAG.

Outcome measure, unless stated: held-out paired IMPs/deal against the HCP-threshold
policy, DD-proxy outcomes (clairvoyant play — not single-dummy making probability).

Status vocabulary: PROPOSED · RUNNING · MEASURED · REFUTED · INCONCLUSIVE · DEAD END.

---

## Track A — practical (improve today's judgment)

### HE-01 Context-specific honour weights (NT)
- **Mechanism**: in NT, aces/kings (tempo + stoppers) are under-valued and Q/J
  over-valued by 4-3-2-1; tens add.
- **Info**: S2 (responder after 1NT 12-14).
- **Definition**: Fifths (4/2.8/1.8/1/0.4), BUM-RAP (4.5/3/1.5/.75/.25), learned rank weights.
- **Prior art**: Andrews "Fifths"; BUM-RAP; Kaplan–Rubens — **established**.
- **Prediction**: Fifths ≥ HCP by ≥ +0.02 IMP/deal on held-out EC-1.
- **Refutation**: CI excludes +0.02 from above (gain < +0.02 with precision).
- **Null**: integer HCP rounding already near-optimal at a narrow decision boundary.
- **Mental cost**: low (tens as half, Q/J slight discount).

### HE-02 Suit-holding value table (SHV)
- **Mechanism**: a suit's value is not the sum of its cards — honours interact with
  length and each other (AK-x-x-x-x ≠ A + K + length; bare Q in a doubleton ≈ nothing).
- **Info**: S2. **Definition**: additive table over (min(len,7), A/K/Q/J/T mask),
  ridge-fitted to DD tricks; trump suit gets its own block when strain is auction-known.
- **Prior art**: Kaplan–Rubens hand-crafts this idea — **new operational measure**.
- **Prediction**: beats learned rank weights (LW) by ≥ +0.02 IMP/deal held-out.
- **Refutation**: SHV ≤ LW with CI upper < +0.02.
- **Confounders**: overfitting (~256-512 params); sample size.
- **Mental cost**: high as a table; needs distillation (HE-11).

### HE-03 Trump-quality-aware loser count (H-8 / T16 B14 anomaly)
- **Mechanism**: classic LTC treats trump Q-less 8-card fits like side suits;
  missing trump honours cost more than side-suit honours (trump loser + control).
- **Info**: S2 (opener after 1M–2M, single raise 6-9, 3+ support).
- **Definition**: SHV with trump block; ablation: LTC + trump-top-honour count.
- **Prior art**: NLTC weights, Klinger trump-quality adjustments — **new combination**.
- **Prediction**: LTC loses to HCP+length+shortness on EC-2 (contradicting F-LTC-004)
  OR a trump-aware variant beats both by ≥ +0.03 IMP/deal.
- **Refutation**: classic LTC ≥ every alternative on held-out EC-2.

### HE-04 Single-number LTC vs HCP for the 1M–2M game decision (F-LTC-004 test)
- **Mechanism**: F-LTC-004 claims LTC > HCP with a fit.
- **Info**: S2. **Definition**: faithful classic LTC (Courtenay; repo `deal::ltc`).
- **Prediction (stated by F-LTC-004)**: −LTC threshold ≥ HCP threshold on EC-2.
- **Refutation**: −LTC < HCP with CI upper < 0.

## Track B — mechanistic (discover what evaluation misses)

### HE-05 The interaction gap (partnership-holding table)
- **Mechanism**: most decision value lives in interaction (stoppers, duplication,
  combined honour structure), not in either hand's total.
- **Info**: S3 DIAG — tells us *what the auction should transmit*, not a table evaluator.
- **Definition**: decompose DD-perfect gain into combined HCP → combined per-suit
  holding classes (combined length × combined top honours) → residual.
- **Prediction**: combined-holding model recovers ≥ 40% of the HCP→DD-perfect gap
  that combined HCP alone leaves (smoke test: combined HCP recovers ~5%).
- **Refutation**: < 20%.

### HE-06 Partner dependence (autonomy)
- **Mechanism**: equal-mean hands differ in how much of their value needs partner's
  specific cards; autonomous hands (long solid suits, aces) have lower variance.
- **Info**: S2 (MC over partner completions consistent with auction).
- **Definition**: per-hand mean and SD of DD tricks over K completions.
- **Prediction**: at fixed mean, high-SD hands have lower P(game makes) when mean
  is above the game line and higher when below — decision by P(make) beats mean.
- **Refutation**: P(make) policy ≤ mean-tricks policy on held-out hands.

### HE-07 Duplication / mirror detection after auction information
- **Mechanism**: shortness opposite partner's known length is wasted; mirror shape
  loses ~1 trick.
- **Info**: S2 where the auction reveals partner's shape.
- **Prior art**: "mirror distribution", wasted values — **established**; operational test new.

### HE-08 Tempo racing in NT
- **Mechanism**: slow winners (Q/J-led suits) lose to the defence's faster suit;
  quick tricks (A/K) win races.
- **Definition**: SHV coefficient pattern by honour type; ablation of A/K vs Q/J weights.
- **Status**: absorbed into HE-01/HE-02 measurement (cannot isolate tempo from DD alone).

## Track C — high-risk conceptual (is a hand one number?)

### HE-09 Strain-relative value (no single scalar)
- **Mechanism**: the same holding is worth different amounts in NT and as a side
  suit/trump suit; a single scalar mis-ranks hands across contexts.
- **Definition**: train SHV in NT context and in suit context; apply each to the other
  context's decision ("transfer cost").
- **Prediction**: transfer costs ≥ 0.05 IMP/deal vs the in-context table.
- **Refutation**: transfer cost CI within ±0.02 (one scalar suffices).

### HE-10 Simulation evaluator (computational ceiling)
- **Mechanism**: own hand + auction → sample partner hands → DD → choose action.
- **Info**: S2 legal. **Prior art**: GIB-style Monte Carlo — **established**.
- **Role**: performance reference, not a human mechanism.

## Track D — ideas outside the seed invitations

### HE-11 Distillation-loss accounting
- A short human rule does not inherit its parent model's gain; measure the loss
  separately on held-out deals.

### HE-12 Decision-boundary locality
- **Mechanism**: an evaluator only matters near the action threshold; global trick
  correlation (r) is a poor proxy for decision value.
- **Prediction**: rank of evaluators by r ≠ rank by held-out IMPs.

### HE-13 Scoring-relative thresholds
- **Mechanism**: the optimal threshold shifts with vulnerability (IMPs) and is
  different again at matchpoints; a "point count for game" is not a hand property.
- **Definition**: fitted thresholds V vs NV for each evaluator.

### HE-14 Invitation as information purchase (NT)
- **Mechanism**: the 2NT invite buys opener's evaluation; its value depends on which
  opener features predict tricks given an invite.
- **Info**: S2 for each player. **Status**: PROPOSED (needs a two-sided policy).

## Rejected / dead-end ideas (recorded)
- "Entropy of the hand" as strength — no operational link to tricks found; dropped.
- Physical metaphors (hand "mass", "momentum") — no measurable definition; dropped.
