---
title: "Hand Evaluation Methods — The Full Scorecard"
subtitle: "Ten methods, five All Fives decisions, about 240 000 simulated deals"
author: "Fermat Institute — Principal Investigator for the Director"
date: "17 November 2026"
mainfont: "DejaVu Serif"
sansfont: "DejaVu Sans"
monofont: "DejaVu Sans Mono"
fontsize: 11pt
geometry: "margin=1.7cm"
colorlinks: true
---

# 1. The answer in one table

Average IMPs gained **per decision** compared with plain HCP, vulnerable. All rows
are human-usable except "SHV", a computer-fitted table shown only to reveal how much
room is left.

| Method | NT response to 1NT (EC-1) | Opener after 1M–2M (EC-2) | Responder raise, 4+ trumps (EC-4) | Opener after 1m–3m: 3NT? (EC-5) | Slam with major fit (EC-6) |
|---|---|---|---|---|---|
| HCP | 0 | 0 | 0 | 0 | 0 |
| Fifths | **+0.04** | +0.06 | +0.00 | −0.00 | +0.17 |
| BUM-RAP | +0.01 | +0.08 | +0.00 | +0.00 | +0.49 |
| HCP + length | +0.03 | +0.21 | +0.00 | −0.25 | +0.33 |
| HCP + length + shortness | — | +0.35 | **+0.05** | −1.07 | +0.47 |
| **Zar** | −0.10 | **+0.43** | +0.04 | −0.99 | **+1.05** |
| Kaplan–Rubens | −0.04 | +0.41 | +0.04 | −0.77 | +0.96 |
| **NLTC** | −0.15 | +0.37 | +0.04 | −1.11 | **+1.05** |
| Classic LTC | −0.64 | +0.04 | +0.00 | **−1.47** | **−0.17** |
| *SHV (computer)* | *+0.04* | *+0.50* | *+0.05* | *+0.05* | *+1.49* |

Fifths: A 4, K 2.8, Q 1.8, J 1, T 0.4. BUM-RAP: A 4.5, K 3, Q 1.5, J 0.75, T 0.25.
NLTC: missing A 1.5 losers, K 1, Q 0.5. Zar: A6 K4 Q2 J1 + (2 × longest + second −
shortest). Non-vulnerable: same ordering in every column, effects about 20-40%
smaller (§5).

# 2. What it means

**Rule 1 — Two lenses.** For a notrump decision use the NT lens (**HCP, or Fifths**).
For a trump decision use the fit lens (**Zar, or NLTC**). Using the wrong lens is
expensive: deciding 3NT after a minor raise with Zar or LTC costs about **1 to 1.5 IMPs
per decision** (EC-5), and the NT lens gains almost nothing in trump decisions.

**Rule 2 — Classic LTC never wins.** It ties HCP for game decisions after a fit, is worse
than HCP for slam, and is the worst method in every NT decision. Its flaw is structural:
it counts K-Q-x as one loser and A-x-x as two, so it cannot see aces. **NLTC** (missing
ace 1.5 losers, king 1, queen 0.5) repairs this and performs like Zar.

**Rule 3 — Zar is the best human fit count.** First or tied-first in both trump
decisions where the choice matters (EC-2 game, EC-6 slam). Kaplan–Rubens and NLTC are
equal or close, but Kaplan–Rubens takes 26 steps.

**Rule 4 — Aces matter most at the slam level.** Every ace-heavy method (Zar, NLTC,
KnR, BUM-RAP) gains most in EC-6. Plain HCP undervalues aces; Fifths, which
weights aces slightly more than HCP does, helps a little.

**Rule 5 — With a 4-card raise the decision is usually easy.** In EC-4 double-dummy
play says bid game with almost any 4-card raise when vulnerable, so no method can gain
much. Treat this column cautiously: it is the case most likely to be inflated by
double-dummy optimism.

# 3. Verdict on each method

| Method | Use it for | Avoid it for | Mental cost |
|---|---|---|---|
| HCP | NT decisions; the shared language of the auction | trump-fit game and slam judgments | lowest |
| Fifths | NT game decisions (tens count; Q/J a bit less) | — (harmless elsewhere, little gain) | low |
| BUM-RAP | nothing in particular (middling everywhere) | NT | low |
| HCP + length (+ shortness) | quick fit estimate when Zar is too slow | NT decisions (shortness is negative there) | low |
| **Zar** | game and slam with a trump fit (**DEC-15 after 1M–2M**) | any NT decision | moderate |
| Kaplan–Rubens | computer analysis; as good as Zar with a fit | table use (26 steps); NT | very high |
| **NLTC** | an alternative fit count, especially for slam | NT decisions | moderate |
| Classic LTC | nothing — retire it as a decision rule | everything measured | low |
| SHV table | measuring headroom in research | table use | computer |

# 4. How to read and trust these numbers

- **Decisions** (all All Fives, opponents silent): EC-1 responder with a balanced hand
  and no 4-card major after 1NT 12-14 (pass or 3NT); EC-2 opener after 1M–2M (pass or
  4M); EC-4 responder with 4+ trumps and 6-15 HCP after 1M (2M or 4M); EC-5 opener after
  1m–3m with 5+ support and 10-12 (pass or 3NT); EC-6 opener + responder with a major fit
  and a game-forcing raise (4M or 6M, decided on the **sum** of both players' counts).
- **Method**: every method gets one decision threshold tuned on separate training deals,
  then is scored on held-out deals (EC-1/EC-2: 20 000 train, 40 000 test; EC-4..EC-6:
  15 000 train, 25 000 test). Endpoints were written down before the test data existed
  (`05-confirmation-plan.md`).
- **Precision**: 95% confidence intervals are about ±0.02 to ±0.08 IMP per decision
  (full intervals in `confirm/`). Differences smaller than about 0.05 are not reliable.
- **Limitations**: double-dummy play (perfect information, no opening-lead model);
  opponents silent; slam without keycard checking; minor raise without the 5m option;
  responder raise without a limit raise. **These are simulations, not table results.**

# 5. Non-vulnerable results

| Method | EC-1 | EC-2 | EC-4 | EC-5 | EC-6 |
|---|---|---|---|---|---|
| Fifths | +0.01 | +0.03 | +0.00 | −0.02 | +0.14 |
| BUM-RAP | −0.03 | +0.05 | +0.01 | −0.04 | +0.41 |
| HCP + length | −0.02 | +0.12 | +0.05 | −0.26 | +0.28 |
| HCP + length + shortness | — | +0.25 | +0.07 | −0.81 | +0.40 |
| Zar | −0.12 | +0.30 | **+0.09** | −0.75 | +0.88 |
| Kaplan–Rubens | −0.05 | +0.29 | +0.07 | −0.64 | +0.81 |
| NLTC | −0.11 | +0.29 | +0.07 | −0.90 | **+0.89** |
| Classic LTC | −0.34 | −0.01 | +0.03 | −1.23 | −0.14 |
| *SHV (computer)* | *−0.00* | *+0.36* | *+0.09* | *+0.03* | *+1.26* |

# 6. Predeclared tests (Addendum B)

| ID | Claim tested | Result (vul) | Verdict |
|---|---|---|---|
| P5 | Classic LTC beats HCP for responder's raise | +0.003 [−0.001, +0.008] | **Refuted** (upper bound < +0.02) |
| P6 | NT lens (Fifths) beats fit lens (Zar) for the 3NT decision after 1m–3m | +0.99 [+0.93, +1.04] | **Confirmed** |
| P7 | Zar total beats HCP total for slam | +1.05 [+0.97, +1.12] | **Confirmed** |

Also measured: classic LTC vs HCP for slam −0.17 [−0.27, −0.07] (LTC worse); NLTC vs
classic LTC for slam +1.22 [+1.17, +1.28]; Kaplan–Rubens vs Zar for slam −0.08
[−0.14, −0.02].

# 7. Tuned lines (double-dummy; check before using at the table)

| Decision | Method | Line found |
|---|---|---|
| Opener after 1M–2M | Zar | game 32+ (no invite) — **ratified as DEC-15 with the 3M invite** |
| Slam with a major fit, GF raise | Zar total | slam at **61+** (Petkov publishes 62) |
| Slam with a major fit | NLTC total | slam at **13 or fewer** combined NLTC losers |
| Slam with a major fit | HCP total | 28+ — implausibly low: shows double-dummy optimism without keycards |
| Opener after 1m–3m | HCP | 3NT with **15+** (vul and non-vul) |
| Responder after 1NT | HCP / Fifths | 3NT with 12+ HCP, or 11.1+ Fifths (vul) |

# 8. Proposals arising (for the Director; not agreements)

- **C — Minor-fit 3NT judgment.** After 1m–3m, decide 3NT with HCP (15+), never with
  losers or Zar. Evidence: P6, about 1 IMP per decision.
- **D — Slam investigation trigger with a major fit.** Use Zar total 61+ (or NLTC 13 or
  fewer) to start slam investigation, instead of "33 HCP" (F-HCP-003) or "12 combined
  losers" (F-LTC-002). Evidence: P7; classic LTC worse than HCP. Must be re-tested
  *with* keycard checking (H-34) before adoption.
- **E — Retire classic LTC as a decision rule** everywhere it is still cited (raise
  ranges, competitive re-evaluation, H-8), replacing it with NLTC where a loser count is
  wanted. Competitive use is still untested.
