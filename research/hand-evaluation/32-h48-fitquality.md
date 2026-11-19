# H-48 — Fermat Fit-Quality: corroborated on 30 000 new random deals

Plan: `05-confirmation-plan.md` Addendum H, frozen before the seed-5000 deals were generated and
before any H-48 code ran; one run. Code: `sim::hand_eval::fit_quality` (3 unit tests), runner
`eval_lab fitquality`. Confirmation data: 30 000 new uniformly random deals, seed 5000
(`data/battle-fresh30k-s5000.jsonl`, sha256 prefix c306e880…, 485 s with ddss), after the usual
30 000 seed-1000 training deals (`data/train30k-s1000-plus-fresh30k-s5000.jsonl`, fraction 0.5).
Output: `confirm/h48-fitquality-confirm.md` / `.err`; exploratory sanity run on seed 2000:
`confirm/h48-fitquality-explore-s2000.*` (definition already frozen).

## Verdict

| Endpoint | Criterion | Seed 5000 (30 000 new boards) | Result |
|---|---|---|---|
| **P-H48** Fit-Quality vs Fermat Complete | CI lower > +0.03 | **+0.077 [+0.048, +0.106]** IMP/board (fit boards **+0.156**) | **Corroborated** |

| Team (IMPs/board vs Fermat Complete) | seed 2000 (exploratory) | **seed 5000 (confirmation)** |
|---|---|---|
| **H-48 Fermat Fit-Quality** | +0.067 | **+0.077** |
| trump quality only | +0.058 | +0.051 |
| ruff units only | −0.022 | −0.025 |
| wasted HCP only | −0.051 | −0.053 |
| FC + 18-feature residual model (DIAG) | +0.100 | +0.076 |
| Ceiling, two-hand table (DIAG) | +0.269 | +0.267 |
| All Fives today (Zar / HCP) | −0.164 | −0.189 |

- The rounded three-part rule is **as good as the fitted 18-feature model** on new deals (+0.077 vs
  +0.076): nothing was lost by rounding the weights to halves.
- Against today's All Fives evaluation (Zar with a fit, HCP in notrump) the gain is about
  **+0.27 IMP per board** (Fermat Complete +0.19 plus Fit-Quality +0.08), all on paper — perfect
  exchange, double dummy.
- **Wasted HCP and ruff units only work together.** Alone each loses: a singleton creates both a
  ruff (more tricks) and wasted honours opposite it (fewer); counting one side of the coin
  misprices the hand.

## The method (fit lens, after an 8-card major fit is found)

Fermat Complete fit total of both hands, then add, over the side suits:

| Term | Points |
|---|---|
| Shorter hand's trump length | **+1½ per trump** (5-3 → +4½; 4-4 → +6; 5-4 → +6) |
| Ninth (or more) trump | **+1½** |
| Ruff units where partner holds 3+ cards in that suit (either hand) | **+1 per unit** (void = 2, singleton = 1) |
| Honours opposite partner's singleton or void (aces included) | **−½ per HCP** |

Lines (tuned on train): **game 57¾ vulnerable / 58¼ non-vulnerable; slam 66¼ (keycard gate and
−4 per uncovered side suit as before); grand 69¼ with all keycards.** Notrump unchanged
(Fermat NT − 3 per unstopped suit; 3NT 24¼ / 24¾).

Worked example: ♠AQ543 ♥KQ32 ♦76 ♣J2 opposite ♠K762 ♥J ♦AK854 ♣Q43 — shorter trumps 4 (+6),
ninth trump (+1½), ♥ singleton opposite four (+1), ♥KQ opposite the singleton (−2½): **+6**.

## Limits and next steps

Perfect exchange (partner's shortness and trump length known), double-dummy play, silent
opponents, five-level not modelled. Before any amendment: (1) the H-42-style auction-model check,
with shortness known only after shape-showing bids (splinters, shortness tries, second suits) and
3- vs 4-card support known from the raise structure; (2) a usability check (counting time,
partner agreement). The remaining ceiling gap (+0.19 beyond Fit-Quality) is still open.
