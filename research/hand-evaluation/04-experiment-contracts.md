# Experiment Contracts & Codex Handoff Packets (packet 1, 2026-11-17)

**Pairing status — honest record.** Codex was *not* connected in this session. No
handoff, independent review or Codex challenge has happened yet. Claude (PI) wrote
both the contracts and the implementation below; that removes the separation of
duties the brief asks for. Each contract therefore ends with a **Codex challenge
packet**: specific things Codex must try to break before any result is promoted.

Shared tooling (implemented this session, TDD, R-TOOL-015):

| Artifact | Path | State |
|---|---|---|
| Evaluators (HCP-variants, Fifths, BUM-RAP, LTC, NLTC, Zar, controls, length/shortness) | `src/bridge-tools/src/sim/hand_eval.rs` | AVAILABLE |
| Feature extractors (holding classes, rank counts, partnership DIAG) | same | AVAILABLE |
| IMPs, scoring, threshold fit, paired summary, ridge, auction filters | `src/bridge-tools/src/sim/eval_lab.rs` | AVAILABLE |
| Runner `eval_lab gen` / `eval_lab compare` | `src/bridge-tools/src/bin/eval_lab.rs` | AVAILABLE |
| Kaplan–Rubens CCCC | — | PROPOSED (awaiting exact published algorithm + fixtures) |
| Single-dummy / information-constrained play | — | ABSENT (DDS proxy only) |

Common rules for all contracts:
- Generation: uniform 52-card shuffle (`StdRng::seed_from_u64(seed)`), rejection on
  the scenario auction filter; acceptance rate logged; one record per deal (no
  rotations / relabelings), so deal = cluster and records are i.i.d.
- Splits by seed: **dev** (exploration, free use) · **train** + **test** (frozen
  definitions, one run) · **confirm** (untouched until the primary comparison is fixed).
- Outcome: DD tricks for the auction's declarer (DDS proxy, clairvoyant play).
- Fitting: every evaluator gets one fitted threshold on train (maximise summed IMP
  gain); learned evaluators additionally get ridge weights on train only.
- Primary statistic: mean paired IMPs/deal vs the HCP policy on test, 95% normal CI
  (i.i.d. deals). Head-to-head IMPs = difference of gains vs always-low (exact,
  because IMPs are odd).

---

## EC-1 — Responder's NT game decision (practical track: HE-01, HE-02, HE-12)

- **Auction**: S opens 1NT (All Fives 1st/2nd seat, 12-14 balanced incl. 5332 with
  5M); N balanced, no 4-card major, 7-14 HCP (so no Stayman/transfer route applies).
  `contract-bridge/all-fives.yaml: openings.1NT`.
- **Decision**: N chooses Pass (1NT) or 3NT. Simplification: no 2NT invite in EC-1
  (an invite policy is HE-14, separate).
- **Evaluator input**: N's 13 cards only (S2: plus "partner 12-14 balanced").
- **Outcome**: DD NT tricks, S declaring. Gain = IMPs(3NT − 1NT).
- **Comparators**: HCP; HCP+length; Fifths; BUM-RAP; Zar; −LTC; −NLTC; LW
  (learned rank weights A..9 + length/shortness, ridge λ=1); SHV (holding table, λ=5).
  Out-of-purpose comparators (LTC, Zar are suit/opening tools) are reported but
  **not** used for any "defeat" claim.
- **Diagnostics** (not achievable): combined HCP (S3), partnership table (S3),
  DD-perfect (S4).
- **Vulnerability**: primary = vulnerable IMPs (declared before confirmation);
  secondary = non-vulnerable.
- **Primary comparison (confirm set)**: best table-usable new/learned evaluator chosen
  on dev vs HCP. Smallest useful effect: +0.02 IMP/deal.
- **Cost**: ~64 deals/s all strains on 28 threads → 40k deals ≈ 11 min.

## EC-2 — Opener's game decision after 1M–2M (mechanistic track: HE-03, HE-04)

- **Auction**: S opens 1♥/1♠ (longest suit, ties to higher; 12-19 HCP; not 12-14
  balanced which opens 1NT); N single raise: 6-9 HCP, 3+ support, excluding 5+
  support with ≤7 losers (game raise). `all-fives.yaml: openings.1_suit.raises`.
- **Decision**: S chooses Pass (2M) or 4M. Game tries are excluded here (they need a
  responder evaluation policy — integrated policy is a separate experiment).
- **Evaluator input**: S's 13 cards + trump suit + "partner 6-9 with 3+ support".
- **Outcome**: DD tricks in M, S declaring. Gain = IMPs(4M − 2M).
- **Comparators**: as EC-1 plus HCP+length+shortness; LTC is **in purpose** here
  (fit known), so F-LTC-004 is directly tested.
- **Primary comparison**: −LTC vs HCP (tests F-LTC-004); secondary: best learned vs best fixed.

## EC-3 — Is a hand one number? (high-risk conceptual: HE-05, HE-09)

- **Part A (interaction gap, DIAG)**: on EC-1/EC-2 test sets, how much of the
  HCP→DD-perfect gain does the S3 partnership-holding table recover, vs combined HCP?
- **Part B (strain transfer)**: SHV fitted in NT context applied to the suit decision
  and vice versa; transfer cost vs in-context SHV.
- **Prediction / refutation**: see HE-05, HE-09 cards.

---

## Codex challenge packets (to run before promotion)

1. **Leakage audit**: confirm no table-usable evaluator reads `partner`, the lead or
   DD tables; `Case.partner` is used only by `DIAG:` rows.
2. **Baseline faithfulness**: check LTC/NLTC/Zar/Fifths/BUM-RAP against the prior-art
   definitions in `01-prior-art.md`; add published worked examples as unit tests.
3. **Scoring**: independent IMP table and contract score checks (doubled scores are
   not used in these contracts).
4. **Auction realism**: EC-1 excludes 4-card-major responders and invitations; EC-2
   excludes game tries. Estimate how much those simplifications change the ranking.
5. **Overfitting**: SHV has 256 (EC-1) / 512 (EC-2) parameters. Re-run with λ ∈ {1, 5,
   20, 50} on dev only; report train-test gap.
6. **Seed independence**: verify dev/train/test/confirm seeds produce disjoint deals
   (hash PBN strings).
7. **Counterexample search**: matched pairs where SHV and HCP disagree; inspect 20 by
   hand for bridge plausibility before believing the aggregate.
