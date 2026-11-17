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
