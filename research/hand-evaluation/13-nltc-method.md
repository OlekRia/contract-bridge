---
title: "New Losing Trick Count (NLTC) — How to Use It"
subtitle: "All Fives · a loser count that sees aces · cross-check to Zar (DEC-15)"
author: "Fermat Institute — Principal Investigator for Ori and Urs"
date: "17 November 2026"
mainfont: "DejaVu Serif"
sansfont: "DejaVu Sans"
monofont: "DejaVu Sans Mono"
fontsize: 11pt
geometry: "margin=1.8cm"
colorlinks: true
---

# 1. What NLTC is, and where it fits

The **New Losing Trick Count** repairs the old Losing Trick Count. The old count charged
one loser for each missing ace, king or queen. That made K-Q-x look better than A-x-x,
so it could not see aces. In our simulations the old count did no better than plain HCP
for game decisions, and was *worse* than HCP for slam.

NLTC charges a missing ace more than a missing king, and a missing king more than a
missing queen. With that one change it performs almost as well as Zar:

| Decision (vulnerable, IMPs per decision vs HCP) | Classic LTC | **NLTC** | Zar |
|---|---|---|---|
| Opener after 1M–2M: pass or 4M | +0.04 | **+0.37** | +0.43 |
| Responder with 4+ trumps: 2M or 4M | +0.00 | **+0.04** | +0.04 |
| Slam with a major fit: 4M or 6M | −0.17 | **+1.05** | +1.05 |
| Any notrump decision | −0.64 to −1.47 | **−0.15 to −1.11** | −0.10 to −0.99 |

**Status.** Our agreement after 1M–2M is **Zar** (DEC-15). NLTC is offered as a second
lens for players who think in losers, as a cross-check, and as a candidate slam count
(Proposal D, not yet adopted). **Never use NLTC (or any loser count) for notrump.**

# 2. Counting NLTC

For each suit, look only at the **top cards**: one card in a singleton, two in a
doubleton, three in any longer suit. Charge losers for the honours missing among them:

| Missing | Losers | Counts only when the suit has |
|---|---|---|
| Ace | **1.5** | 1 or more cards |
| King | **1.0** | 2 or more cards |
| Queen | **0.5** | 3 or more cards |

A void is 0. Add the four suits. A hand ranges from 0 (very strong) to 12 (worthless).

**Common holdings — learn these:**

| Holding | NLTC | Holding | NLTC | Holding | NLTC |
|---|---|---|---|---|---|
| void | 0 | AK | 0 | AKQ (+ any) | 0 |
| A | 0 | AQ | 1.0 | AKx | 0.5 |
| K | 1.5 | Ax | 1.0 | AQx | 1.0 |
| x | 1.5 | KQ | 1.5 | Axx | 1.5 |
| | | Kx | 1.5 | KQx | 1.5 |
| | | Qx | 2.5 | Kxx / KJx | 2.0 |
| | | xx | 2.5 | Qxx | 2.5 |
| | | | | Jxx / xxx | 3.0 |

(Long suits count only their top three cards: AKxxx = AKx = 0.5.)

**Example A.** ♠AJ953 ♥A42 ♦A732 ♣6
♠ A-J-9 → missing K, Q → 1.5 · ♥ A-x-x → 1.5 · ♦ A-x-x → 1.5 · ♣ singleton x → 1.5.
**NLTC 6.0.**

**Example B.** ♠KQ953 ♥KQ2 ♦QJ32 ♣6
♠ K-Q-x → 1.5 · ♥ K-Q-x → 1.5 · ♦ Q-J-x → 2.5 · ♣ x → 1.5. **NLTC 7.0.**

Same 13 HCP and same shape. The old count called B the better hand (5 losers against
7). NLTC rightly prefers A: with A, game made in 41% of simulated layouts after
1♠–2♠; with B, in 16%.

**Trick estimate (published rule):** tricks ≈ **25 − (your NLTC + partner's NLTC)**.
The old count used 24.

# 3. Decision lines (from our simulations)

| Situation | Bid the higher contract when | Vulnerable | Not vulnerable |
|---|---|---|---|
| Opener after 1M–2M (cross-check to Zar) | your NLTC is at most | **6.5** → 4M | **6.0** → 4M |
| Responder with 4+ trumps after 1M | your NLTC is at most | **10** → 4M | **9.5** → 4M |
| Slam with a major fit and a GF raise | combined NLTC is at most | **13** → 6M | **13** → 6M |
| Any notrump decision | — | do not use | do not use |

The slam line is a trick count only: it does **not** replace keycard checking, and it is
a proposal (D), not an agreement.

# 4. Worked examples

**Opener after 1♠–2♠ (vulnerable unless stated).** "P(game)" = share of 200 simulated
layouts consistent with the auction where 4♠ makes double-dummy.

| Hand (♠ ♥ ♦ ♣) | HCP | NLTC | NLTC says | Zar says | P(game) | Who is right |
|---|---|---|---|---|---|---|
| AKJ84 A9653 K2 6 | 15 | 5.0 | 4♠ | 4♠ (35) | 0.73 | both |
| KQJ853 AQ872 4 2 | 12 | 5.5 | 4♠ | 3♠ (31) | 0.76 | **NLTC** |
| AJ953 A42 A732 6 | 13 | 6.0 | 4♠ | 3♠ (32) | 0.41 | both reasonable |
| QJ9632 AK4 A7 J5 | 15 | 6.5 | 4♠ | 4♠ (33) | 0.50 | both |
| AKQ74 A632 84 52 | 13 | 6.5 | 4♠ | pass (30) | 0.17 | **Zar** |
| AQ852 7 KJ43 K72 | 13 | 6.5 | 4♠ | pass (30) | 0.32 | **Zar** |
| KQ953 KQ2 QJ32 6 | 13 | 7.0 | pass | pass (28) | 0.16 | both |
| AQ852 KJ4 Q73 K2 | 15 | 7.0 | pass | pass (30) | 0.26 | both |
| KJ852 AQ43 Q2 72 | 12 | 8.0 | pass | pass (27) | 0.09 | both |

Non-vulnerable (line 6.0): QJ9632 AK4 A7 J5 (6.5) passes, but game made in 58% of
layouts (Zar invites); AJ953 A42 A732 6 (6.0) bids game, which is marginally wrong
(−0.4 IMP on average).

**Responder with four trumps after 1♠ (2♠ or 4♠?)**

| Hand (♠ ♥ ♦ ♣) | HCP | NLTC | Vul | Non-vul | P(game) | Avg IMPs for 4♠ vul / NV |
|---|---|---|---|---|---|---|
| K965 A 7543 8632 | 7 | 8.0 | 4♠ | 4♠ | 0.67 | +4.7 / +2.4 |
| Q743 4 A8652 J73 | 7 | 8.5 | 4♠ | 4♠ | 0.69 | +5.0 / +2.6 |
| K962 Q84 J73 Q52 | 8 | 10.0 | 4♠ | 2♠ | 0.41 | +0.4 / −0.4 |
| Q743 J52 Q63 J52 | 6 | 11.0 | 2♠ | 2♠ | 0.18 | −3.2 / −2.6 |

Notice: the 8-count with no shortness is a *weaker* raise than the 7-counts with a
singleton ace or a singleton and a 5-card side suit.

**Slam check with a major fit (illustration of counting; no simulation shown):**

- Opener ♠AKJ84 ♥A9653 ♦K2 ♣6 (NLTC 5.0) + responder ♠Q762 ♥K4 ♦AQ54 ♣A93
  (NLTC 6.5) = **11.5** → slam zone: check keycards.
- Opener ♠AQ852 ♥KJ4 ♦Q73 ♣K2 (NLTC 7.0) + responder ♠KT73 ♥Q6 ♦KJ54 ♣A93
  (NLTC 8.0) = **15.0** → stay in game, although the pair holds 28 HCP.

# 5. Common mistakes

1. **Counting beyond the top three cards.** A-K-x-x-x is 0.5, not more.
2. **Charging a king in a singleton.** Singleton K = 1.5: in a one-card suit only the
   missing ace counts.
3. **Charging a queen in a doubleton.** Q-x = 2.5 (missing A and K), not 3.0.
4. **Using 24 instead of 25** for the trick estimate.
5. **Using it for notrump.** NLTC rewards shortness; in notrump shortness is a liability.
6. **Adding it to Zar.** Use one count per decision. If they disagree, see §6.

# 6. When NLTC and Zar disagree

Over 40 000 simulated 1M–2M decisions Zar was slightly better (+0.06 IMP per decision),
and Zar is our agreement. From the examples above:

- NLTC is **too optimistic** on hands of exactly 6.5 built from small doubletons and
  side-suit honours without extra shape (AKQ74 A632 84 52; AQ852 7 KJ43 K72).
- NLTC is **better** on wild two-suiters with few HCP (KQJ853 AQ872 4 2).
- For slam the two are tied (+1.05 each over HCP).

# 7. Drill (answers on the last page)

Opener after 1♠–2♠ — NLTC? Action vulnerable? Not vulnerable?

1. ♠AKJ84 ♥A9653 ♦K2 ♣6
2. ♠KQ953 ♥KQ2 ♦QJ32 ♣6
3. ♠QJ9632 ♥AK4 ♦A7 ♣J5
4. ♠AJ953 ♥A42 ♦A732 ♣6
5. ♠KJ852 ♥AQ43 ♦Q2 ♣72
6. ♠KQJ853 ♥AQ872 ♦4 ♣2

Responder with four spades after 1♠ — NLTC? 2♠ or 4♠ (vul / non-vul)?

7. ♠K965 ♥A ♦7543 ♣8632
8. ♠K962 ♥Q84 ♦J73 ♣Q52
9. ♠Q743 ♥J52 ♦Q63 ♣J52

Slam: combined NLTC and verdict?

10. ♠AKJ84 ♥A9653 ♦K2 ♣6 opposite ♠Q762 ♥K4 ♦AQ54 ♣A93

\newpage

# Memory card

```
NLTC per suit (top 1/2/3 cards only):
    missing A = 1.5   (any length)
    missing K = 1.0   (2+ cards)
    missing Q = 0.5   (3+ cards)           void = 0
Tricks ~ 25 - (my NLTC + partner's NLTC)

                                   VUL      NONVUL
Opener after 1M-2M (cross-check)   <= 6.5   <= 6.0   -> 4M
Responder, 4+ trumps, after 1M     <= 10    <= 9.5   -> 4M
Slam, major fit, GF raise          combined <= 13    -> 6M (then keycards)

NEVER for notrump.  Never added to Zar.  Our 1M-2M agreement is Zar.
```

\newpage

# Drill answers

1. 5.0 — 4♠ / 4♠. 2. 7.0 — pass / pass. 3. 6.5 — 4♠ / pass (Zar would invite
non-vul; game made in 58% of layouts). 4. 6.0 — 4♠ / 4♠ (non-vul marginal).
5. 8.0 — pass / pass. 6. 5.5 — 4♠ / 4♠ (here NLTC beats Zar).
7. 8.0 — 4♠ / 4♠. 8. 10.0 — 4♠ / 2♠. 9. 11.0 — 2♠ / 2♠.
10. 5.0 + 6.5 = 11.5 — slam zone; check keycards.

All NLTC values computed by `eval_lab eval`; game frequencies by `eval_lab mc`
(200 layouts each, double-dummy, seeds 7001-7006); decision lines tuned on 15 000-20 000
training deals and tested on 25 000-40 000 held-out deals (`12-scorecard.md`).
