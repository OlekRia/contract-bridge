# Predeclared Confirmation Plan (frozen 2026-11-17, before any test-set analysis)

Written after dev exploration on `nt-dev-s101` / `major-dev-s201` and **before** the
held-out sets `nt-test-s112` / `major-test-s212` (40 000 deals each) were analysed.

## Frozen artefacts

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

## Primary endpoints (one per experiment; vulnerable IMPs, DD proxy)

| ID | Comparison | Prediction (from dev) | Refutation condition |
|---|---|---|---|
| P1 (EC-1, HE-01) | Fifths vs HCP, responder pass/3NT | ≥ +0.02 IMP/deal | 95% CI upper bound < +0.02 |
| P2 (EC-2, HE-04 / F-LTC-004) | −LTC vs HCP, opener pass/4M | F-LTC-004 predicts LTC > HCP; dev predicts ≈ 0 | F-LTC-004 refuted in this scope if CI upper bound < +0.02 |
| P3 (EC-2, H-8 practical) | Zar vs −LTC | ≥ +0.20 IMP/deal | CI upper bound < +0.20 |
| P4 (EC-3, HE-14) | SHV/SHV invite vs Zar/Zar direct | ≥ +0.05 IMP/deal | CI upper bound < +0.05 |

## Secondary endpoints (reported; Bonferroni ×6 noted, 99.2% intervals approximated by ±2.64 SE)

- S1 SHV vs Zar direct (EC-2) — HE-02 / Binky-style table vs best published fixed formula.
- S2 ZX (Zar + placement) vs Zar (EC-2) — distilled placement rule.
- S3 KnR vs HCP (EC-1) — replication of Andrews' NT finding (prediction: ≤ 0).
- S4 HCP invite vs HCP direct (EC-2) — prediction: ≤ 0 (invites without good evaluation lose).
- S5 SHV-helpsuit invite vs SHV invite (EC-2) — dev: no gain; prediction ≈ 0.
- S6 Non-vulnerable replication of P1–P4.

## Stopping rule and reporting

Single run; no re-tuning after viewing test output. All finalists reported including losses.
If a primary endpoint is inconclusive (CI spans both the prediction and the refutation
threshold), it is reported as INCONCLUSIVE, not re-run with more data in this packet.
A further confirmation seed (s113 / s213) is reserved for packet 2.

## Addendum A (written before any test-set analysis, same day)

Added after dev exploration of HE-09 (strain transfer) and an unrelated mid-session
toolchain change (a `rustup update` not run by the PI moved rustc 1.99.1 → 1.100.0;
binaries were rebuilt; `compare`/`invite` source unchanged — new subcommands `mc`,
`transfer` and the `majornt` loader variant were added to `src/bin/eval_lab.rs`).

- **S7 (HE-09)**: `eval_lab transfer majornt data/major-dev-s201.jsonl major
  data/major-dev-s201.jsonl data/major-test-s212.jsonl v` — same opener hands, table
  fitted to NT tricks vs to trump tricks. Prediction: transferred table loses ≥ 0.20
  IMP/deal vs the in-context table (vul). Refutation: CI lower bound > −0.20.
  Confounder removed vs the first dev attempt: both tables see the same hand population.

## Post-run record

Runs executed once each as specified (outputs in `confirm/`). After the runs, the ridge
solver was refactored for clippy (`needless_range_loop`) and the crate was `cargo fmt`ed;
the rebuilt binary reproduced the dev numbers to 4 decimals (Zar +0.4588, LW +0.3870,
SHV +0.5130). Post-format hashes: `hand_eval.rs` f6508756…, `eval_lab.rs` 14c93bd2…,
`bin/eval_lab.rs` 88f77a9b…. One reporting column ("vs Zar/Zar direct") was added to
`invite` output after the first P4 run; thresholds and data were unchanged.

## Addendum B — scorecard extension (written before any EC-4..EC-6 data existed)

Requested by the Director ("evaluate them all"). Three new decisions, single-stage
confirmatory design (no exploration): train 15 000 deals, test 25 000 deals, separate
seeds; every method gets one threshold tuned on train (learned models also fit on train).
All methods in `compare` are reported; DD proxy; opponents silent.

| Exp | Auction | Decision | Seeds (train/test) |
|---|---|---|---|
| EC-4 | 1M – ?, responder 4+ trumps, 6-15 HCP | responder 2M vs 4M (opener declares) | 301 / 312 |
| EC-5 | 1m – 3m (5+ support, 10-12, no 4M) | opener pass 3m vs 3NT | 401 / 412 |
| EC-6 | 1M – GF raise (4+ trumps, 13+) | partnership 4M vs 6M, on the **sum** of each player's own count | 501 / 512 |

Primary endpoints (vulnerable; NV secondary):
- **P5 (EC-4)**: classic LTC vs HCP for responder. Prediction (F-LTC-004 / H-8's
  raise ranges): LTC > HCP. Refuted if CI upper bound < +0.02.
- **P6 (EC-5)**: Fifths (NT lens) vs Zar (fit lens) for the 3NT decision. Prediction
  (strain lenses, HE-09): Fifths ≥ Zar. Refuted if CI upper bound < 0.
- **P7 (EC-6)**: Zar total vs HCP total for slam. Prediction: Zar > HCP by ≥ +0.05.
  Refuted if CI upper bound < +0.05.
Head-to-head CIs from the paired per-deal CSVs. Known simplifications: no keycard
check in EC-6 (pure trick count), no 5m option in EC-5, no limit raise in EC-4.

## Addendum C — the million-deal battle (written before the battle data existed)

Requested by the Director: Zar vs NLTC vs all other methods on 1 000 000 uniformly random
deals (seed 1000), DD outcomes, "who reaches the DD-optimal contract".

- Samples: every partnership (NS and EW) with 20+ combined HCP; opponents silent.
- Shared strain rule and declarer (`sim::battle::shared_strain`): 8+ major fit, else NT.
- Each method: partnership total (sum of both hands) chooses partscore / game / small slam
  / grand; three cuts per strain regime (suit / NT) tuned on deals 0-299 999 (30%).
- Test: deals 300 000-999 999. Pilot reads only the first 20 000 deals (inside train).
- Metrics: % optimal level (in the shared strain); mean IMP regret vs DD-best level;
  mean IMPs vs silent-opponents DD par (all strains); head-to-head IMP matrix.
  Uncertainty: cluster-robust by deal.
- **Primary (P8)**: Zar vs NLTC head-to-head, vulnerable. Prediction (from EC-2/EC-6):
  Zar ≥ NLTC (difference within ±0.05 IMP per sample would be called a tie).
- Secondary: full ranking by IMP regret, both vulnerabilities.
- **Amendment to Addendum C (before any test-region data was analysed):** after the pilot
  (first 20 000 deals, inside the training region) the Director asked to add Kaplan–Rubens
  (already present) and Darricades' OHE. Added contestants: "OHE start (HL+D)", "OHE full"
  (Timm codification, `14-ohe-codification.md`), and fixed published-line variants
  "HCP @25/33/37", "Zar @52/62/67", "OHE full @pub". P8 (Zar vs NLTC) unchanged.
- **Amendment 2 to Addendum C (Director, before any test-region analysis):** sample
  reduced from 1 000 000 to the first **100 000** deals of seed 1000 (compute time). Split
  unchanged: first 30% (30 000 deals) train, remaining 70 000 test. File:
  `data/battle-100k-s1000.jsonl` (first 100 000 complete lines of the generation run).

## Addendum E — H-45 Fermat Interlock (written 2026-11-18, before any interlock code ran)

Requested by the Director ("discover a method better than Fermat Complete"). Frozen now:

- **Definitions** exactly as filed in H-45 (`gov/hypotheses.yaml`): fit-lens terms fitting
  (+1 per K/Q/J in a side suit where partner has 4+), wasted (−1 per K/Q/J opposite a
  singleton; −2 K, −1 Q/J, −1 A opposite a void), double fit (+2 for a side suit with a combined
  8+); NT filler (+½ per J/T in a suit where partner holds A, K or Q). Gates (−3 per unstopped
  suit, −4 per uncovered side suit, keycards) as in Fermat Complete.
- **Ceiling (DIAG, step 0)**: ridge λ = 5 on DD tricks in the shared strain; features = combined
  per-suit holding class (length × A/K/Q/J/T mask) and short-hand length × combined-length
  bucket, separate trump block, plus raw fitting/wasted counts; fitted separately for NT and
  major-fit sides on the train region; three cuts on predicted tricks per vulnerability.
- **Split**: first 30% of deals train (seed 1000, deals 0-29 999) in both files.
  Exploratory = `data/battle-100k-s1000.jsonl` test region (seed 1000, already used to develop
  Fermat Complete); **confirmation = `data/rematch-train30k-plus-fresh70k.jsonl` fresh region
  (seed 2000)**. Definitions are not changed after the exploratory run.
- **Primary (P-H45)**: Fermat Interlock (all terms) vs Fermat Complete, IMPs per board on the
  fresh region, 32-board blocks, rotating vulnerability, perfect exchange. Corroborated if the
  95% CI lower bound > +0.03; refuted if the upper bound < 0.
- **Step 0 (S-H45)**: Ceiling vs Fermat Complete ≥ +0.10 → headroom exists; < +0.05 → the
  search moves to information transfer.
- Secondary: ablations (each term alone, all but one, fit lens only); % optimal; IMPs lost vs DD.

## Addendum F — H-46 Partnership Shortness (written 2026-11-18, before any H-46 code ran and before the seed-3000 deals were read)

- **Definition** exactly as filed in H-46: per side suit, s = shorter hand's length, c =
  combined length: +2 (s = 0, c ≤ 7), +1 (s = 1, c ≤ 7), −1 (s ≥ 3), else 0. Fit value =
  Fermat fit totals + w × term; w chosen from {1, 2, 3, 4, 5} by lowest total train regret
  after tuning the lines for each w. NT lens and all gates as in Fermat Complete.
- **Data**: train = deals 0-29 999 of `data/battle-100k-s1000.jsonl` (as before); confirmation
  = `data/battle-fresh30k-s3000.jsonl` (30 000 new uniformly random deals, seed 3000, generated
  with ddss 0.2 — byte-identical solver output to dds-bridge 0.18 on the benchmark), combined
  as `data/train30k-s1000-plus-fresh30k-s3000.jsonl`, train fraction 0.5.
- **Exploratory** runs only on the seed-1000 and seed-2000 regions; no definition changes
  after them.
- **Primary (P-H46)**: H-46 vs Fermat Complete on the seed-3000 deals, IMPs per board,
  32-board blocks, rotating vulnerability, perfect exchange. Corroborated if the 95% CI lower
  bound > +0.03; refuted if the upper bound < 0; otherwise inconclusive.
- Secondary: ruffing-bonus-only, mirror-only, H-46 + wasted honours (H-45 term), ceiling
  (DIAG, same ridge as Addendum E), All Fives today; fit/NT split; share of the ceiling.

## Addendum G — H-47 Fermat Complete + wasted honours (written 2026-11-18, before the seed-4000 deals were generated)

- **Definition** exactly as filed in H-47 (= `interlock_fit` with only `wasted` on, unchanged
  since H-45): side suits only; −1 per K/Q/J opposite partner's singleton; −2 per K and −1 per
  A/Q/J opposite partner's void. No tuned weight; lines re-tuned on train as for Fermat Complete.
- **Data**: train = deals 0-29 999 of `data/battle-100k-s1000.jsonl`; confirmation =
  `data/battle-fresh30k-s4000.jsonl` (30 000 new uniformly random deals, seed 4000, ddss),
  combined as `data/train30k-s1000-plus-fresh30k-s4000.jsonl`, train fraction 0.5.
- **Primary (P-H47)**: H-47 vs Fermat Complete, IMPs per board on the seed-4000 deals, 32-board
  blocks, rotating vulnerability, perfect exchange. Corroborated if mean ≥ +0.015 and 95% CI
  lower bound > 0; refuted if the upper bound < 0; otherwise inconclusive. One run.
- Secondary: fit/NT split, % optimal, IMPs lost vs DD, ceiling (DIAG) and All Fives today.

## Addendum H — H-48 Fermat Fit-Quality (written 2026-11-18, before the seed-5000 deals were generated and before any H-48 code ran)

- **Definition** exactly as filed in H-48. Fit value = Fermat fit totals + term, term over side
  suits: −½ × HCP held opposite partner's 0-1 cards (A=4, K=3, Q=2, J=1); +1½ × shorter hand's
  trump length; +1½ if combined trumps ≥ 9; +1 × ruff units (void 2, singleton 1) in side suits
  where partner holds 3+, both hands. No tuned weight; lines re-tuned on train; gates unchanged.
- **Data**: train = deals 0-29 999 of `data/battle-100k-s1000.jsonl`; confirmation =
  `data/battle-fresh30k-s5000.jsonl` (30 000 new uniformly random deals, seed 5000, ddss),
  combined as `data/train30k-s1000-plus-fresh30k-s5000.jsonl`, train fraction 0.5.
- **Primary (P-H48)**: H-48 vs Fermat Complete on the seed-5000 deals, IMPs per board, 32-board
  blocks, rotating vulnerability, perfect exchange. Corroborated if the 95% CI lower bound
  > +0.03; refuted if the upper bound < 0; otherwise inconclusive. One run.
- Secondary: each part alone (wasted HCP; trump quality; ruff units), the 18-feature residual
  model and the ceiling (DIAG), All Fives today; fit/NT split.

## Addendum H2 — H-48 step 2, auction-model check (written 2026-11-18, before any step-2 code ran)

- **Models**: the H-42 `eval_lab systems` information models (captain + partner's count bucket
  per system; SAYC, Precision, Acol, 2/1, All Fives; silent and competitive). Fermat Complete's
  coverage gates as in H-42 (shown by stopper/control bids).
- **Information levels for the Fit-Quality term** (added to the regime's Fermat fit value):
  L1 = trump term only (3- vs 4-card support shown by the raise structure), wasted/ruff terms 0;
  **L2 = trump term always + wasted/ruff terms only when the regime's Fermat fit estimate ≥ 50**
  (game-try / slam-try zone where splinters and shortness tries happen); L3 = full term known.
  Lines tuned on train per level.
- **Data**: `data/train30k-s1000-plus-fresh30k-s5000.jsonl`, train fraction 0.5, 5 000-board
  matches; secondary run on `data/rematch-train30k-plus-fresh70k.jsonl` (0.3).
- **Primary (P-H48b)**: Fit-Quality L2 vs Fermat Complete, All Fives model, **competitive**
  opponents, IMPs per board. Corroborated if the 95% CI lower bound > 0; refuted if the upper
  bound < 0. Secondary: L1 and L3, the other systems, silent opponents, L2 vs Zar / HCP.
