---
title: "Beyond Points — Hand Evaluation Research"
subtitle: "Fermat Institute · Packet 1 · Evidence, ratified change and teaching material"
author: "Principal Investigator (Claude) for the Director"
date: "17 November 2026"
mainfont: "DejaVu Serif"
sansfont: "DejaVu Sans"
monofont: "DejaVu Sans Mono"
fontsize: 10pt
geometry: "margin=1.6cm"
colorlinks: true
toc: true
toc-depth: 2
header-includes:
  - \usepackage{longtable}
  - \renewcommand{\arraystretch}{1.15}
---

# Executive summary

**Question.** Can a new way of perceiving a bridge hand beat established evaluation
methods — measurably, on deals it was not tuned on?

**Answer, in defined situations: yes; globally: no.** Everything below is double-dummy
(DD) simulation on 40 000 held-out deals per scenario, with endpoints frozen before the
test data were read. Opponents are silent; play is clairvoyant.

| Finding (vulnerable IMPs per qualifying deal) | Result | 95% CI |
|---|---|---|
| Zar beats classic LTC after 1M–2M (now ratified: DEC-15) | **+0.394** | +0.352 to +0.436 |
| Classic LTC vs HCP (F-LTC-004's claim) | +0.035 | −0.009 to +0.079 |
| 3M invitation with Zar on both sides vs Zar alone | **+0.100** | +0.072 to +0.128 |
| Learned holding table vs Zar (not human-usable) | +0.069 | +0.043 to +0.096 |
| Fifths vs HCP, responder after 1NT (replicates Andrews) | +0.043 | +0.026 to +0.060 |
| Same hand valued for NT, used for a suit decision | −0.436 | −0.474 to −0.398 |

**What we learned.** A hand has no single value: it is *strain-relative* (NT vs trump
values correlate only 0.69), *partner-relative* (knowing partner's holdings is worth
about twice the best single-hand count), and therefore *auction-relative* (invitations
pay only when both partners count well). Classic LTC fails because it cannot see aces:
it scores K-Q-x as one loser and A-x-x as two.

**Ratified.** AMD-34 / DEC-15 / H-36: after 1M–2M, opener counts Zar — vulnerable 33+
game, 31-32 invite with 3M; non-vulnerable 34+ game, 32-33 invite; responder accepts
with Zar 20+ (vul) / 19+ (non-vul). Table validation is tracked under H-36.

**Limits.** DD proxy, no opening-lead model, silent opponents, game decisions only, no
human usability test yet. Per random board the gains are small (≈ +0.015 IMP for the
LTC→Zar change); per qualifying decision they are large.

\newpage
# Beyond Points — packet 1 (opening memo)

Brief: `../../hand-evaluation-discovery.yaml`. PI: Claude. Codex: not connected this session.
Status of every claim: see the labels in each file. Nothing here changes an agreement.

## Discovery thesis

A bridge hand does not have one value. Its worth is (1) **strain-relative** — the same
thirteen cards rank differently for notrump and for trump play; (2) **partner-relative** —
most of the decision value lives in how the two hands interlock, which no single-hand
count can see; and therefore (3) **auction-relative** — the practical gains come less
from a cleverer count than from using a good count on *both* sides of an exchange.
Within one strain, well-chosen published counts are already close to what any
own-hand method can achieve.

## Five ways to perceive a hand (evidence status, packet 1)

| # | Perception | Evidence | Status |
|---|---|---|---|
| 1 | **Strain lenses** — value a hand separately for NT and for a trump fit; a scalar tuned for one strain is no better than HCP in the other | Same opener hands: NT-fitted vs trump-fitted values r = 0.69; transfer loses 0.44 IMP/deal (held-out, 40k) | COMPUTATIONAL RESULT, confirmed (S7) |
| 2 | **Aces are tempo; loser counts that ignore them fail** — classic LTC scores KQx better than Axx | Classic LTC ≈ HCP for the 1M–2M game decision; Zar +0.39 IMP/deal over LTC (held-out, 40k) | confirmed (P2 inconclusive vs HCP, P3 confirmed) |
| 3 | **Holdings, not cards** — a suit's value depends on length and companions (singleton honours half-wasted; tens/nines make 11-counts into games) | Holding table beats Zar by +0.07 (held-out); matched pairs F2, F4 | confirmed in EC-2 (S1); rediscovered (Andrews' Binky points) |
| 4 | **Invitations are worth what both evaluations are worth** | Zar invite +0.10 over Zar direct; SHV invite +0.23 over Zar direct; HCP-evaluated invites gain ≈ 0 | confirmed (P4); new operational measure |
| 5 | **The headroom is in the partnership, not the hand** | S2 Monte-Carlo ceiling ≈ Zar for the direct decision; S3 partnership table +0.96 vs HCP | DIAG / exploratory |

## Files

| File | Content |
|---|---|
| `00-ledger.md` | Shared ledger, reproducibility header, datasets |
| `01-prior-art.md` | Literature audit, exact definitions, comparison matrix (subagent; sources dated) |
| `02-anomaly-audit.md` | H-8 / H-31 / H-34, T16, T12 B16 recomputed (subagent; verified B16 DD) |
| `03-hypothesis-cards.md` | 14 hypothesis cards (HE-01..HE-14) incl. 4 outside the seed invitations |
| `04-experiment-contracts.md` | EC-1..EC-3 contracts and 7 Codex challenge packets |
| `05-confirmation-plan.md` | Frozen endpoints, hashes, addendum |
| `06-results.md` | Dev and confirmation results, failure atlas |
| `07-teaching-positions.md` | 24 positions, 6 matched pairs, Monte-Carlo solutions |
| `08-human-procedure.md` | Draft memory card, drills, usability plan (not run) |
| `09-director-packet.md` | Scoped superiority statement, adoption proposal, cascade, pilot |
| `10-next.md` | Packet 2 plan and Codex handoff |
| `data/`, `confirm/`, `teaching/` | Seeded datasets (JSONL), paired per-deal CSVs, MC outputs |

Tooling: `src/bridge-tools/src/sim/hand_eval.rs`, `src/sim/eval_lab.rs`,
`src/bin/eval_lab.rs` (R-TOOL-015, TDD).


\newpage

# Director Review Packet — hand evaluation, packet 1

For: Ori (Director). From: Claude (PI). Date: 2026-11-17.
Nothing here changes a table agreement. Each item below needs your decision.

## 1. Scoped superiority statement

> **Task**: opener's game decision after an uncontested 1♥/1♠ – 2♥/2♠ in All Fives
> (opener 12-19, not 12-14 balanced; single raise 6-9 HCP with 3+ trumps).
> **Information**: opener's own 13 cards + the auction (responder's own cards for the
> acceptance decision of an invitation). **Outcome**: double-dummy tricks (clairvoyant
> play, no opening-lead model). **Scoring**: IMPs, head-to-head against the named rule
> on the same deal. **Sample**: 40 000 held-out uniformly generated deals (seed 212),
> rules tuned on 20 000 separate deals (seed 201), endpoints frozen in advance
> (`05-confirmation-plan.md`).
>
> | Comparison (vulnerable) | IMP/qualifying deal | 95% CI |
> |---|---|---|
> | Zar vs classic LTC (our current raise note) | **+0.394** | [+0.352, +0.436] |
> | Zar vs HCP | +0.429 | [+0.397, +0.461] |
> | Classic LTC vs HCP (tests F-LTC-004) | +0.035 | [−0.009, +0.079] |
> | Holding table (SHV) vs Zar | +0.069 | [+0.043, +0.096] |
> | Zar invite (3M) vs Zar direct | +0.100 | [+0.072, +0.128] |
> | SHV invite vs Zar direct | +0.228 | [+0.198, +0.258] |
>
> Non-vulnerable: same ordering, about 25-35% smaller (`06-results.md` §2).
> **Limitations**: DD proxy (single-dummy play and opening leads not modelled);
> opponents assumed silent; game decision only (no slam, no partscore competition);
> 1st/2nd seat filter; simulated deals, not table evidence. The auction occurs on
> ≈ 3.7% of boards for our pair, so **+0.39 IMP/qualifying deal ≈ +0.015 IMP per random
> board** (≈ 0.35 IMP per 24-board session) for the LTC→Zar change alone.

What is **not** claimed: global superiority; anything about NT, minors, competition or
slam; any human-performance gain (no usability test has been run).

## 2. Proposed adoption (minimal) — needs your approval

**Proposal A — replace the classic-LTC raise note with Zar for game decisions after a
single major raise.** → **RATIFIED 2026-11-17** by the Director ("ratify now, track at
table"): AMD-34, DEC-15, H-36, R-BID-049, F-ZAR-001, F-SIM-007, S-JUDGE-EVAL-4.
- CURRENT AGREEMENT: `contract-bridge/all-fives.yaml: openings.1_suit.raise_note` =
  "After fit, use LTC: 24 - combined losers = tricks (H-8)".
- PROPOSED: "After 1M–2M, opener counts Zar (HCP + controls (A=2, K=1) + lengths of the
  two longest suits + (longest − shortest)). Vulnerable: 33+ bid game, 31-32 invite with
  3M, 30 or less pass; responder accepts with Zar 20+. Non-vulnerable: 34+ game, 32-33
  invite, responder accepts with Zar 19+. Without an invitation agreement: 32+ game."
  _These integer thresholds are the tuned DD values (V: invite 30.5-32.5, accept 19.5;
  NV: invite 31.5-33.5, accept 18.5; direct 31.5 both) and should be re-checked in the
  prospective pilot before any rounding change._
  _How responder counts Zar for acceptance is the same formula on responder's hand
  (the tuned policy used exactly that)._
- Cascade (if approved):
  - `gov/hypotheses.yaml`: H-8 → record the measured refutation of LTC's advantage in
    this scope; file a new H-ID for "Zar fit lens" with the pilot as its protocol.
  - `gov/facts.yaml`: F-LTC-004 — weaken to "not supported for the 1M–2M game decision
    (DD, 40k deals): LTC ≈ HCP"; add a model fact for Zar in this scope (class: model).
  - `gov/requirements.yaml`: new R-BID for the raise-continuation evaluation rule.
  - `all-fives.yaml` + `all-fives-verbose.yaml` (+ generated md/pdf): replace raise_note.
  - Constitution: no change.

**Proposal B — Fifths for responder's 1NT game decision, vulnerable only (optional).**
Confirmed held-out: Fifths vs HCP +0.043 [+0.026, +0.060] IMP/qualifying deal vulnerable,
+0.006 (null) non-vulnerable; ≈ +0.001 IMP per random board. This is prior art (Andrews)
replicated, not a discovery. Recommendation: teach it as judgment ("tens count, 11-counts
with intermediates bid 3NT vulnerable"), no agreement change. No learned table beat it.

## 3. Conflicts and source corrections found (separate from any new treatment)

From `02-anomaly-audit.md` (each is a SOURCE ERROR / GAP, not a system change):
- T12 B16 (H-31's founding board): DD is 9 tricks in ♠, not 10; ♠Q is West's; partner had
  2 HCP, not 9. H-31's premise ("missed game") does not hold.
- H-34's T16 B14 account: "two trump losers unless doubleton AK onside" is wrong.
- AMD-31 is cited three times but does not exist; `analysis/2026-09-08.md` was never committed.
- Two loser-count conventions coexist (tool: Qxx = 2 losers; 2NT LTC ask: Qxx = 3).
- `all-fives-verbose.yaml:435` example is 9 HCP, not 10.

## 4. Prospective pilot (for your review)

- Duration: 6 sessions (≈ 150 boards); expected qualifying 1M–2M auctions: ≈ 5-6.
  **This is far too few to confirm a 0.4 IMP effect at the table** (SD per qualifying
  board ≈ 4-5 IMPs). The pilot's purpose is usability (speed, recall, agreement), not
  proof; table confirmation needs ≈ 400+ qualifying boards.
- Record per qualifying board: both players' Zar, LTC and HCP counts written after the
  board, the call made, the result, and the S2 Monte-Carlo answer computed afterwards.
- Stop rule: abandon if either player reports the count takes > 20 s at the table or if
  the partnership disagrees on the count in > 1 of 5 hands.

## 5. Decisions requested

1. Approve, amend or reject Proposal A (and whether to pilot it first).
2. Approve the source corrections in §3 as a separate housekeeping amendment.
3. Approve packet 2 scope (`10-next.md`): Codex challenge packets, NT confirmation,
   single-dummy validation, competitive auctions, slam zone.


\newpage

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


\newpage

# Teaching Positions — packet 1 (24 positions, 6 matched pairs)

**How to use**: read the hand and auction, decide, *then* read the solution line.
Solutions are **COMPUTATIONAL RESULTS (DDS proxy)**: 200 auction-consistent random
completions per hand (`eval_lab mc`, seeds 7001 / 7002), DD tricks with the auction's
declarer, vulnerable IMPs, game vs stopping low. P(make) has a standard error of about
±0.03; E[IMP] is the average IMP result of bidding game instead of stopping low.
Card counts are asserted by the tool (13 cards each). Raw output:
`teaching/mc-major-v.csv`, `teaching/mc-nt-v.csv`.

Auctions follow CURRENT AGREEMENT (`contract-bridge/all-fives.yaml`): 1NT = 12-14 balanced
(1st/2nd seat); single raise = 6-9, 3+ support. Simplification (labelled): opener decides
Pass vs 4M directly (no game try); responder decides Pass vs 3NT directly (no 2NT invite).

The single-deal "reveal" format of the brief is replaced here by the 200-deal profile,
so no lesson rests on one lucky layout. Example full deals per position: packet 2.

---

## Part 1 — Opener after 1♠/1♥ – 2♠/2♥ (you are South, vulnerable)

| # | Hand (♠ ♥ ♦ ♣) | HCP | LTC | NLTC | Zar | KnR | P(make 10) | E[IMP] | Verdict |
|---|---|---|---|---|---|---|---|---|---|
| 1 (A1) | AJ852 KQ 8743 K2 | 13 | 7 | 7.5 | 29 | 12.85 | 0.19 | −3.1 | Pass |
| 2 (A2) | AJ852 73 KQ84 K2 | 13 | 6 | 7.0 | 29 | 14.35 | 0.23 | −2.4 | Pass |
| 3 (B1) | AKQ74 A632 84 52 | 13 | 6 | 6.5 | 30 | 16.10 | 0.165 | −3.6 | Pass |
| 4 (B2) | 97542 AKQ3 A4 52 | 13 | 6 | 6.5 | 30 | 14.40 | 0.20 | −2.9 | Pass |
| 5 (C1) | AJ953 A42 A732 6 | 13 | 7 | 6.0 | 32 | 15.80 | 0.41 | **+0.5** | **Game** |
| 6 (C2) | KQ953 KQ2 QJ32 6 | 13 | **5** | 7.0 | 28 | 13.45 | 0.155 | −3.7 | Pass |
| 7 (D1) | AQ852 K KJ43 872 | 13 | 7 | 7.5 | 30 | 12.65 | 0.125 | −4.1 | Pass |
| 8 (D2) | AQ852 7 KJ43 K72 | 13 | 6 | 6.5 | 30 | 14.75 | 0.32 | −1.0 | Pass (close) |
| 9 | KQJ853 AQ872 4 2 | **12** | 4 | 5.5 | 31 | 17.10 | 0.755 | **+6.0** | **Game** |
| 10 | AQ852 KJ4 Q73 K2 | **15** | 6 | 7.0 | 30 | 14.40 | 0.26 | −1.9 | Pass |
| 11 | KT7642 AKQ A KT3 | 19 | 4 | 4.0 | 40 | 20.25 | 0.985 | +9.8 | Game; slam try |
| 12 | Q9852 A KQ743 Q2 | 13 | 5 | 6.5 | 30 | 13.55 | 0.325 | −1.0 | Pass (close) |
| 13 | KJ852 AQ43 Q2 72 | 12 | 7 | 8.0 | 27 | 12.30 | 0.09 | −4.7 | Pass |
| 14 | J97642 AK KQ3 Q2 | **15** | 6 | 7.0 | 32 | 13.15 | 0.325 | −1.0 | Pass (close) |

Lessons (each a pair with identical HCP and suit lengths):

- **C1 vs C2 — aces are tempo; classic LTC is blind to them.** Same 13 HCP, same 5-3-4-1.
  Three aces make game a favourite (0.41, +0.5 IMP); K-Q-J hands do not (0.155, −3.7).
  Classic LTC calls C2 the *better* hand (5 losers vs 7) because it scores KQx as one
  loser and Axx as two. NLTC (missing ace = 1.5) and Zar (controls) get it right.
- **D1 vs D2 — an honour in a singleton is half-wasted.** Moving the ♥K into clubs
  (singleton ♥ stays) lifts P(make) from 0.125 to 0.32. HCP and Zar cannot see this.
- **A1 vs A2 — a doubleton KQ vs KQ in a 4-card side suit.** Small effect here
  (0.19 vs 0.23): within noise, recorded as a weak placement effect.
- **B1 vs B2 — "trump quality" did not show up.** Solid AKQ trumps (0.165) vs AKQ in a
  side suit (0.20): no premium for trump honours opposite known 3+ support. KnR
  scores B1 1.7 points higher — the wrong way here. (Bears on H-34's trump gate and the
  T16 B14 account: the issue there was missing *controls*, not trump texture as such.)
- **#9 vs #10 — shape beats HCP.** 12 HCP 6-5 is a game; 15 HCP 5-3-3-2 is not.
  HCP gets both wrong; LTC, NLTC, Zar and KnR get #9 right.

## Part 2 — Responder after 1NT (12-14) (you are North, balanced, no 4-card major, vulnerable)

| # | Hand (♠ ♥ ♦ ♣) | HCP | Fifths | KnR | P(make 9) | E[IMP] 3NT | Verdict |
|---|---|---|---|---|---|---|---|
| 15 (E1) | Q72 K83 KJ54 Q62 | 11 | 10.2 | 8.95 | 0.27 | −1.8 | Pass |
| 16 (E2) | QT9 KT8 KJT9 Q62 | 11 | 11.4 | 10.50 | **0.595** | **+3.5** | **3NT** |
| 17 (F1) | K72 Q83 KQJ54 62 | 11 | 10.2 | 10.75 | 0.32 | −0.9 | Pass |
| 18 (F2) | K72 KQ3 J7542 Q2 | 11 | 10.2 | 8.55 | 0.255 | −2.0 | Pass |
| 19 | A72 983 A754 K62 | 11 | 10.8 | 11.35 | 0.27 | −1.8 | Pass |
| 20 | Q72 J3 AQ954 Q62 | 11 | 10.4 | 10.15 | 0.365 | −0.3 | Pass (close) |
| 21 | QJ2 QJ3 QJ54 QJ2 | 12 | 11.2 | 7.40 | 0.415 | +0.5 | 3NT (close) |
| 22 | K72 83 AKT94 962 | 10 | 10.0 | 12.40 | 0.325 | −0.8 | Pass |
| 23 | K72 Q83 AJ54 K62 | 13 | 12.4 | 11.90 | 0.72 | +5.5 | 3NT |
| 24 | K72 Q83 A754 962 | 9 | 8.6 | 8.35 | 0.06 | −5.1 | Pass |

Lessons:

- **E1 vs E2 — intermediates decide 11-counts.** Same HCP and shape; tens and nines turn
  a clear pass (0.27) into a clear 3NT (0.595). HCP and Zar see nothing; Fifths (tens)
  and KnR (tens, nines) see it.
- **F1 vs F2 — concentration in the long suit helps a little** (0.32 vs 0.255), not
  enough to change the decision.
- **#21 — a hand of queens and jacks** still makes 3NT 41% of the time opposite 12-14
  balanced; KnR (7.40) badly undervalues it in NT, which matches its measured NT loss.
- **#19 — aces do not rescue a flat 11 in NT** the way they do in a suit contract
  (compare C1). Interpretation (one hand, plus the EC-1 learned weights A:K:Q:J ≈
  4 : 2.9 : 1.7 : 0.9): in NT aces carry no premium beyond Fifths; opposite a known
  suit fit they do (EC-2 weights A:K ≈ 4 : 2.4).

## What these positions cannot show

- They use DD play (clairvoyant), no opening-lead model, and stop-or-blast decisions.
- 200 samples per hand: a P(make) difference below ~0.08 within a pair is noise.
- They are illustrations chosen by the PI, not a validation set.


\newpage

# Human Procedure, Memory Card and Drills — DRAFT (packet 1)

Status: **PROPOSED — not usability-tested.** No partnership timing, recall or agreement
data exist yet. Simplified rules do not inherit their computational parents' results;
each rule below names the exact test it has (or has not) passed.

## The idea in one sentence

**A hand has no single value: count it with the lens that matches the strain the auction
has made likely — the NT lens before a fit, the fit lens once a trump suit is agreed.**

(HE-09: the same opener hands valued for NT vs for their trump suit correlate only
r ≈ 0.69, and the NT value used for a suit decision is no better than HCP.)

## Memory card

```
NT LENS (no fit; balanced auctions, e.g. 1NT – ?)
  Count HCP, then: tens +½, queens and jacks count a little less (Fifths: 4/2.8/1.8/1/0.4).
  Opposite 12-14: 12+ → 3NT.  Exactly 11 → 3NT only with two tens (or T-9 combinations).
  [Test status: Fifths itself — EC-1 P1. The "11 + two tens" shortcut — UNTESTED (packet 2).]

FIT LENS (partner has raised your major)
  Zar = HCP + controls (A=2, K=1) + (your two longest suits) + (longest − shortest).
  After 1M – 2M, no invitation agreement: Zar 32+ → bid game.
  With an invitation (3M):  vul   33+ game · 31-32 invite · responder accepts Zar 20+
                            nonvul 34+ game · 32-33 invite · responder accepts Zar 19+
  Do NOT count classic losers (LTC) here: it cannot see aces (KQx = 1 loser, Axx = 2).
  [Test status: Zar vs LTC +0.39 IMP/deal (P3, confirmed held-out); Zar invite vs Zar
   direct +0.10 (confirmed held-out). DD proxy, silent opponents. Human use untested.]

TWO WARNINGS (from matched pairs; illustrations, not aggregate tests)
  • An honour in a singleton is worth about half (F2).
  • Solid trump honours earn no extra opposite known 3-card support (F3).

INVITE ONLY IF BOTH OF YOU COUNT WELL
  With HCP on both sides the tuned policy never invites (vul) and an HCP invite gained
  only +0.02 (nonvul); with Zar on both sides the invite gains +0.10 (vul).
```

## When to ignore the card

- Slam zone (the card is calibrated for game decisions only).
- Competitive auctions, sacrifices and penalty doubles (no defensive model in packet 1).
- Minor-suit fits, 5-3 vs 4-4 strain choice, and NT with a known 8-card major fit.
- Matchpoints: thresholds were tuned for IMPs; matchpoint thresholds are not measured.

## Double-counting guard

Zar already counts shape twice (two longest + longest−shortest) and aces twice (HCP +
controls). Do **not** add Goren shortness points, LTC or "length points" on top.

## Drills (delayed solutions in `07-teaching-positions.md`)

1. Six matched pairs (positions 1-8, 15-18): for each pair, decide both hands before
   looking; score 1 point for each pair where you rank the two hands correctly.
2. Positions 9-14 and 19-24: game or not? Record time per decision.
3. Re-test after one week with the hands' suits permuted (same holdings, different suits).

## Usability evaluation plan (not run)

- Participants: Ori and Urs, independently.
- Measures: seconds per decision, recall of the card after 7 days, inter-partner
  agreement (Cohen's κ) on 40 fresh positions, decision quality vs the S2 Monte-Carlo
  answer, compared with today's procedure (HCP + LTC raise note).
- Held-out: the 40 positions are drawn from seed 213 (reserved, untouched).


\newpage

# Appendix A — Hypothesis portfolio


Local research IDs (`HE-nn`), **not** official H-IDs. Nothing here changes a table
agreement. Information states: **S1** own hand pre-auction; **S2** own hand + legal
auction; **S3** both partnership hands (diagnosis only); **S4** all four hands (oracle).
Every table-usable evaluator below is S1/S2; anything using S3/S4 is labelled DIAG.

Outcome measure, unless stated: held-out paired IMPs/deal against the HCP-threshold
policy, DD-proxy outcomes (clairvoyant play — not single-dummy making probability).

Status vocabulary: PROPOSED · RUNNING · MEASURED · REFUTED · INCONCLUSIVE · DEAD END.

---

### Track A — practical (improve today's judgment)

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

### Track B — mechanistic (discover what evaluation misses)

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

### Track C — high-risk conceptual (is a hand one number?)

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

### Track D — ideas outside the seed invitations

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

### Rejected / dead-end ideas (recorded)
- "Entropy of the hand" as strength — no operational link to tricks found; dropped.
- Physical metaphors (hand "mass", "momentum") — no measurable definition; dropped.


\newpage

# Appendix B — Frozen confirmation plan


Written after dev exploration on `nt-dev-s101` / `major-dev-s201` and **before** the
held-out sets `nt-test-s112` / `major-test-s212` (40 000 deals each) were analysed.

### Frozen artefacts

| File | sha256 (prefix) |
|---|---|
| `src/sim/hand_eval.rs` | `281348213c51344d` |
| `src/sim/eval_lab.rs` | `606c746f7cf88453` |
| `src/bin/eval_lab.rs` | `6662c2d7fbecddb5` |
| `data/nt-dev-s101.jsonl` (train) | `d4594193f4844a2c` |
| `data/major-dev-s201.jsonl` (train) | `46f1b3a50b780bfc` |

Training = the full 20 000-deal dev set of each scenario (thresholds and ridge weights,
λ = 5 for SHV, 1 for LW/RxL, 0.1 for VEC/DST/ZX, as in code). Test = the untouched
40 000-deal seeds. One run per command, both vulnerabilities; **vulnerable is primary**.
Commands:

```
eval_lab compare nt    data/nt-dev-s101.jsonl    data/nt-test-s112.jsonl    v|nv  paired.csv
eval_lab compare major data/major-dev-s201.jsonl data/major-test-s212.jsonl v|nv  paired.csv
eval_lab invite  major data/major-dev-s201.jsonl data/major-test-s212.jsonl v|nv
eval_lab invite  nt    data/nt-dev-s101.jsonl    data/nt-test-s112.jsonl    v|nv
```

### Primary endpoints (one per experiment; vulnerable IMPs, DD proxy)

| ID | Comparison | Prediction (from dev) | Refutation condition |
|---|---|---|---|
| P1 (EC-1, HE-01) | Fifths vs HCP, responder pass/3NT | ≥ +0.02 IMP/deal | 95% CI upper bound < +0.02 |
| P2 (EC-2, HE-04 / F-LTC-004) | −LTC vs HCP, opener pass/4M | F-LTC-004 predicts LTC > HCP; dev predicts ≈ 0 | F-LTC-004 refuted in this scope if CI upper bound < +0.02 |
| P3 (EC-2, H-8 practical) | Zar vs −LTC | ≥ +0.20 IMP/deal | CI upper bound < +0.20 |
| P4 (EC-3, HE-14) | SHV/SHV invite vs Zar/Zar direct | ≥ +0.05 IMP/deal | CI upper bound < +0.05 |

### Secondary endpoints (reported; Bonferroni ×6 noted, 99.2% intervals approximated by ±2.64 SE)

- S1 SHV vs Zar direct (EC-2) — HE-02 / Binky-style table vs best published fixed formula.
- S2 ZX (Zar + placement) vs Zar (EC-2) — distilled placement rule.
- S3 KnR vs HCP (EC-1) — replication of Andrews' NT finding (prediction: ≤ 0).
- S4 HCP invite vs HCP direct (EC-2) — prediction: ≤ 0 (invites without good evaluation lose).
- S5 SHV-helpsuit invite vs SHV invite (EC-2) — dev: no gain; prediction ≈ 0.
- S6 Non-vulnerable replication of P1–P4.

### Stopping rule and reporting

Single run; no re-tuning after viewing test output. All finalists reported including losses.
If a primary endpoint is inconclusive (CI spans both the prediction and the refutation
threshold), it is reported as INCONCLUSIVE, not re-run with more data in this packet.
A further confirmation seed (s113 / s213) is reserved for packet 2.

### Addendum A (written before any test-set analysis, same day)

Added after dev exploration of HE-09 (strain transfer) and an unrelated mid-session
toolchain change (a `rustup update` not run by the PI moved rustc 1.99.1 → 1.100.0;
binaries were rebuilt; `compare`/`invite` source unchanged — new subcommands `mc`,
`transfer` and the `majornt` loader variant were added to `src/bin/eval_lab.rs`).

- **S7 (HE-09)**: `eval_lab transfer majornt data/major-dev-s201.jsonl major
  data/major-dev-s201.jsonl data/major-test-s212.jsonl v` — same opener hands, table
  fitted to NT tricks vs to trump tricks. Prediction: transferred table loses ≥ 0.20
  IMP/deal vs the in-context table (vul). Refutation: CI lower bound > −0.20.
  Confounder removed vs the first dev attempt: both tables see the same hand population.

### Post-run record

Runs executed once each as specified (outputs in `confirm/`). After the runs, the ridge
solver was refactored for clippy (`needless_range_loop`) and the crate was `cargo fmt`ed;
the rebuilt binary reproduced the dev numbers to 4 decimals (Zar +0.4588, LW +0.3870,
SHV +0.5130). Post-format hashes: `hand_eval.rs` f6508756…, `eval_lab.rs` 14c93bd2…,
`bin/eval_lab.rs` 88f77a9b…. One reporting column ("vs Zar/Zar direct") was added to
`invite` output after the first P4 run; thresholds and data were unchanged.
