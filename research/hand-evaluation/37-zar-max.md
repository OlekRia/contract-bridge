# Zar Max — the full Zar Points method in a SAYC team league (H-49)

Source: Zar Petkov, *Zar Points — Aggressive Bidding Hand Evaluation* (October 2003, 38 pp.),
www.bridgewebs.com/ocala/ZarPoints.pdf. Our earlier "Zar" lens uses only the book's initial
count. **Zar Max** adds every re-evaluation rule in the book that a program can apply.

Runner: `eval_lab sayc20 data/train30k-s1000-plus-fresh30k-s6000.jsonl 0.5 5000 <out> [competitive]`.
Evaluators: `sim::hand_eval::{zar_max, zar_max_fit, suit_footprint_losers, footprint_losers, zar_max_slam_ok}`
(14 unit tests, written first; the book's worked hands are the fixtures). R-TOOL-016, H-49.
Outputs: `confirm/sayc20-s6000-silent.md`, `confirm/sayc20-s6000-competitive.md` (`.err` holds the tuned lines).

## 1. The method

**One-hand count, as you pick up the cards (`zar_max`)**

| Component | Rule | Book |
|---|---|---|
| High-card Zars | HCP + controls (A=2, K=1), i.e. 6-4-2-1 | p.3 |
| Distribution Zars | (a + b) + (a − d): the two longest suits plus longest minus shortest | p.4-5 |
| Short honours | −1 for each singleton K, Q or J and each doubleton KQ, KJ, QJ, Qx or Jx | p.6, p.13 |
| Concentration | +1 when every HCP is in two suits (11-14 HCP) or three suits (15+ HCP); never below 11 HCP | p.6 |

**Fit re-evaluation, shown by the raise (`zar_max_fit`)**

| Component | Rule | Book |
|---|---|---|
| Honours in partner's suit | the raiser (shorter trump holding) gets +1 per A, K, Q, J, T in trumps, max 2. With equal lengths, the two hands' bonuses are averaged | p.12-13 |
| Super-fit | +3 per trump beyond eight (9th +3, 10th +6, 11th +9) | p.13-14 |
| Secondary fit | the longest side suit with 8+ cards: +3 per card beyond eight, plus the shorter holder's honours there, +1 each, max 2 | p.12, p.14 |

**Slam gate: footprints and controls (`zar_max_slam_ok`)**

For each side suit, FP is the shorter holding and CT is the combined controls. Immediate losers
are 0 if FP < CT, min(1, FP) if FP = CT, and min(2, FP) if FP > CT (p.37).
- **Small slam** needs at most one immediate loser and three aces (SAYC Blackwood).
- **Grand slam** needs no immediate loser and all four aces plus the trump king.

**Lines and notrump.** The book says to bid game at 52, the five-level at 57, a small slam at 62 and,
at 5 per level, a grand at 67. It says that balanced hands and notrump go by HCP ("the only thing
that matters is brute HCP power", p.35), so Zar Max's notrump lens is HCP.

**Not exercised here.** The 26-point opening, the spade tiebreak at 25 and the 22-25 weak two all
change *which* hands open. In this league the openings are fixed by SAYC.

**Book inconsistencies (we follow the stated rules).**
- The AQ doubleton is discounted in the 6-card-suit example (p.15) but not in the Meckstroth hand on the same page. We do not discount AQ.
- Rodwell's 11-count gets the three-suit concentration upgrade (p.6), but the rule reserves that upgrade for 15+ HCP.
- Two pieces of arithmetic are off by one (p.5 Chagas, 27 vs 26).

## 2. The league

- **Field:** 22 teams of four, and every team bids **SAYC**. All four players in a team use one evaluation method.
- **Auction model:** `exchange_seated`, `System::Sayc`. The captain knows their own 13 cards; partner's count is only known as the average over partner's SAYC bidding range.
- **Roster:** the 20 teams of `35-auction20.md`, plus **Zar Max (H-49)** with tuned lines and **Zar Max, book lines** (52/62/67, NT 25/33/37).
- **Deals:** lines tuned on the 30 000 seed-1000 deals; tested on the 30 000 brand-new seed-6000 deals.
- **Format:** round robin of 231 matches × 5 000 boards, WBF 20-VP scale, double-dummy results.

### Silent opponents (top 12 of 22)

| # | Team | Won | VP/match | IMPs lost vs DD/board |
|---|---|---|---|---|
| 1 | Fit-Quality L2 | 21/21 | 18.80 | −2.329 |
| 2 | Fermat + partnership shortness (H-46) | 19/21 | 18.56 | −2.349 |
| 3 | Fermat Complete | 20/21 | 18.21 | −2.353 |
| 4 | Fermat + wasted honours (H-47) | 18/21 | 17.13 | −2.377 |
| **5** | **Zar Max (H-49)** | **17/21** | **15.13** | **−2.454** |
| 6 | Fermat / Fifths | 14/21 | 12.82 | −2.513 |
| 7 | Fermat / Fermat NT | 15/21 | 12.76 | −2.507 |
| 8 | Zar / Fifths | 15/21 | 12.25 | −2.521 |
| 9 | Kaplan-Rubens / Fifths | 14/21 | 12.05 | −2.517 |
| 10 | Zar / HCP | 12/21 | 11.74 | −2.515 |
| … | | | | |
| 19 | Zar Max, book lines | 4/21 | 4.92 | −2.658 |
| 22 | SAYC classic (HCP 25/33/37) | 0/21 | 0.00 | −3.226 |

### Opponents compete (top 12 of 22)

| # | Team | Won | VP/match | IMPs lost vs DD/board |
|---|---|---|---|---|
| 1 | Fermat + partnership shortness (H-46) | 20/21 | 18.46 | −2.586 |
| 2 | Fit-Quality L2 | 20/21 | 17.56 | −2.612 |
| 3 | Fermat Complete | 20/21 | 17.51 | −2.605 |
| 4 | Fermat + wasted honours (H-47) | 18/21 | 16.29 | −2.623 |
| **5** | **Zar Max (H-49)** | **17/21** | **14.51** | **−2.694** |
| 6 | Fermat / Fifths | 14/21 | 13.30 | −2.729 |
| 7 | Zar / Fifths | 14/21 | 12.27 | −2.735 |
| … | | | | |
| 10 | Zar / HCP | 11/21 | 11.15 | −2.756 |
| 12 | Zar Max, book lines | 9/21 | 9.69 | −2.764 |
| 22 | SAYC classic (HCP 25/33/37) | 0/21 | 0.00 | −3.286 |

Full tables, including % optimal and fit/NT splits, are in `confirm/sayc20-s6000-*.md`.

### Head-to-head (IMPs per board, 95% CI)

| Match | silent | competing |
|---|---|---|
| **Zar Max vs Zar / HCP** (H-49 primary) | **+0.064 [+0.036, +0.092]** | **+0.067 [+0.040, +0.094]** |
| … on fit boards | +0.132 [+0.075, +0.189] | +0.138 [+0.082, +0.194] |
| Zar Max vs Fermat Complete | −0.091 [−0.129, −0.053] | −0.078 [−0.114, −0.042] |
| … fit boards only | −0.082 [−0.151, −0.012] | −0.037 [−0.101, +0.027] |
| Zar Max vs SAYC classic | +0.904 | +0.718 |
| Zar Max tuned vs book lines | +0.215 [+0.183, +0.247] | +0.078 [+0.049, +0.106] |
| Book lines: Zar Max vs plain Zar (52/62/67) | −0.004 [−0.039, +0.030] | +0.095 [+0.062, +0.128] |

### Ablation: each variant vs Zar / HCP (tuned lines)

| Variant | silent | competing |
|---|---|---|
| One-hand count only (discounts + concentration) | −0.001 [−0.014, +0.013] | +0.011 [−0.002, +0.024] |
| + fit re-evaluation, no slam gate | +0.018 [−0.009, +0.045] | +0.040 [+0.016, +0.065] |
| + slam gate, no fit re-evaluation | +0.000 [−0.022, +0.022] | +0.014 [−0.009, +0.036] |
| **Full Zar Max** | **+0.064** | **+0.067** |

### Tuned lines (SAYC model) vs the book

| Line | Book | Tuned, silent (non-vul / vul) | Tuned, competing |
|---|---|---|---|
| Game in a fit | 52 | 55.2 / 54.2 | 53.7 |
| Small slam | 62 | 63.2 | 61.7 |
| Grand slam | 67 | 67.7 | 66.5 |
| NT game / slam / grand (HCP) | 25 / 33 / 37 | 24.4 (23.8 vul) / 31.4 / 35.0 | 23.8 / 31.7 / 34.9 |

## 3. Reading

1. **H-49 is corroborated by its pre-stated criterion.** Zar Max beats Zar / HCP by +0.064 IMP/board, with a CI lower bound of +0.036, above the +0.03 bar. The competitive model gives the same result (+0.067). All of the gain is on fit boards (+0.13), because notrump is HCP in both teams. The margin over the bar is small.
2. **Zar Max is the best method outside the Fermat family** in both conditions (5th of 22, 17/21 matches won). It finishes ahead of every other one-hand method we have tested, including Zar / Fifths, Kaplan-Rubens / Fifths and NLTC.
3. **It does not reach Fermat Complete** (−0.09 silent, −0.08 competing). About half of that gap is in notrump: Fermat NT with the stopper check beats plain HCP by about 0.10. On fit boards the gap is −0.08 silent and −0.04 competing, and the competing figure is not significant.
4. **The parts only work together.** Neither the fit re-evaluation alone (+0.02/+0.04) nor the footprint gate alone (0.00/+0.01) delivers much; together they give +0.064/+0.067. The fit terms push super-fit hands into the slam zone, and the footprint gate stops the ones with two fast losers in a side suit. The one-hand refinements (short-honour discount, concentration) are worth ≈ 0.
5. **The book's slam and grand lines are right; its game line is about 2-3 points too low** with silent opponents (52 vs 54-55). With competing opponents it is closer (53.7). Playing the book's lines costs 0.2 IMP/board silent (all of it from game bidding plus the 25/33/37 NT lines) and 0.08 competing.

## 4. Caveats

- **Range-information auction model, not bid-by-bid.** The fit terms are treated as shown by the raise and the footprint gate as shown by cue bids/Blackwood. This favours Zar Max over the plain lenses, which have no slam gate, in the same way it favours the Fermat family.
- **Double-dummy play; one test seed (6000);** competitive model as in `20-competitive.md`.
- **The Fermat teams' control bids** (stoppers, RKCB) are not SAYC. They are kept from `35-auction20.md` so the rosters stay comparable.

## 5. Zar Max at the table (memory card)

1. **Count:** HCP + controls (A=2, K=1) + two longest suits + (longest − shortest).
2. **Short honours:** −1 for each singleton K/Q/J or doubleton KQ/KJ/QJ/Qx/Jx.
3. **Concentration:** +1 if all your HCP are in 2 suits (11-14 HCP) or 3 suits (15+).
4. **Raising partner:** +1 per trump honour (A-K-Q-J-10), max 2.
5. **Extra length, once the fit is known:** +3 per trump beyond eight; +3 per card beyond eight in a second fit, plus your honours there (max 2).
6. **Lines:** about **54** for game, **62-63** for a small slam, **67** for a grand.
7. **Before slam, count fast losers in each side suit.** Compare the shorter holding with the combined controls (A=2, K=1). If the holding is shorter, there are 0 losers; if they are equal, 1; if the holding is longer, 2. A suit never has more losers than the length of the shorter holding.
   - **Small slam:** at most 1 fast loser in total, and no more than one ace missing.
   - **Grand slam:** no fast losers, all four aces and the trump king.
8. **Notrump:** count HCP only.
