---
title: "Zar Points — How We Play Them"
subtitle: "All Fives · DEC-15 / R-BID-049 · Opener after 1♥/1♠ – 2♥/2♠"
author: "Fermat Institute — Principal Investigator for Ori and Urs"
date: "17 November 2026"
mainfont: "DejaVu Serif"
sansfont: "DejaVu Sans"
monofont: "DejaVu Sans Mono"
fontsize: 11pt
geometry: "margin=1.8cm"
colorlinks: true
---

# 1. Why Zar, and only here

After partner raises our major (1♥–2♥ or 1♠–2♠, 6-9 HCP, 3+ trumps), we used to
count losers (LTC). On 40 000 simulated deals the old LTC rule did no better than plain
HCP, and **Zar beat it by about 0.4 IMP on average each time this decision came up** (DD
simulation, held-out deals). LTC's flaw: it counts K-Q-x as one loser and A-x-x as two,
so it cannot see aces. Zar counts aces heavily and rewards shape.

**Scope (ratified, DEC-15):** opener's decision after an *uncontested* 1M–2M, and
responder's answer to 1M–2M–3M. Everywhere else we play as before.

# 2. Counting Zar in two steps

**Zar = honour points + shape number.**

**Step 1 — honour points.** Count **A = 6, K = 4, Q = 2, J = 1.**
(That is HCP plus controls: an ace is 4 + 2, a king 3 + 1.)

**Step 2 — shape number = 2 × longest suit + second-longest − shortest.**
(Same as Petkov's "two longest suits + longest minus shortest".) Learn the common ones:

| Shape | Shape no. | Shape | Shape no. | Shape | Shape no. |
|---|---|---|---|---|---|
| 4-3-3-3 | 8 | 5-5-2-1 | 14 | 6-5-1-1 | 16 |
| 4-4-3-2 | 10 | 5-5-3-0 | 15 | 6-5-2-0 | 17 |
| 4-4-4-1 | 11 | 6-3-2-2 | 13 | 7-2-2-2 | 14 |
| 5-3-3-2 | 11 | 6-3-3-1 | 14 | 7-3-2-1 | 16 |
| 5-4-2-2 | 12 | 6-4-2-1 | 15 | 7-3-3-0 | 17 |
| 5-4-3-1 | 13 | 6-4-3-0 | 16 | 7-4-1-1 | 17 |
| 5-4-4-0 | 14 | 6-6-1-0 | 18 | | |

**Example.** ♠AJ953 ♥A42 ♦A732 ♣6
Honour points: A A A = 18, J = 1 → 19. Shape 5-4-3-1 → 13. **Zar 32.**

**Speed tip:** count the shape number first (it never changes during the auction), then
add the honours suit by suit. Aim for under 15 seconds (not yet timed at the table).

# 3. The decision after 1M – 2M (opener)

| Opener's Zar | Vulnerable | Not vulnerable |
|---|---|---|
| 34 or more | 4M | 4M |
| 33 | 4M | 3M (invite) |
| 32 | 3M (invite) | 3M (invite) |
| 31 | 3M (invite) | Pass |
| 30 or less | Pass | Pass |

Non-vulnerable needs one point more because the game bonus is smaller.

# 4. Responder after 1M – 2M – 3M

Responder counts Zar on **their own** hand the same way:

- **Vulnerable:** 20 or more → 4M; 19 or less → pass.
- **Not vulnerable:** 19 or more → 4M; 18 or less → pass.

A typical flat 8-count is about Zar 18 (decline). Aces and a singleton push a hand over
20 (accept). A maximum built from queens and jacks usually declines. This is deliberate:
Q-J values opposite a minimum opener are the ones that fail to produce tricks.

# 5. Worked examples (you are South; 1♠ – 2♠, opponents silent)

Evidence column: share of 200 random layouts consistent with the auction where 4♠
makes double-dummy ("P(game)"). This is a simulation, not a guarantee.

| Hand (♠ ♥ ♦ ♣) | HCP | Zar | Vul action | NV action | P(game) vul / NV |
|---|---|---|---|---|---|
| AKJ84 A9653 K2 6 | 15 | 35 | 4♠ | 4♠ | 0.73 / 0.78 |
| QJ9632 AK4 A7 J5 | 15 | 33 | 4♠ | 3♠ | 0.50 / 0.58 |
| AJ953 A42 A732 6 | 13 | 32 | 3♠ | 3♠ | 0.41 / 0.41 |
| AQ852 KJ4 Q73 K2 | 15 | 30 | Pass | Pass | 0.26 / 0.29 |
| KQ953 KQ2 QJ32 6 | 13 | 28 | Pass | Pass | 0.16 (vul) |
| KJ852 AQ43 Q2 72 | 12 | 27 | Pass | Pass | 0.09 (vul) |
| KT7642 AKQ A KT3 | 19 | 40 | 4♠ (or more) | 4♠ (or more) | 0.99 (vul) |

Notice the second and fourth rows: **15 HCP can be a game or a pass** depending on aces
and shape. And the third and fifth: same 13 HCP, same shape, but **aces versus
king-queen-jack** is the difference between inviting and passing.

**Responder examples** (partner opened 1♠, you raised to 2♠, partner bids 3♠):

| Hand (♠ ♥ ♦ ♣) | HCP | Zar | Vulnerable | Not vulnerable |
|---|---|---|---|---|
| A73 9 Q8642 K753 | 9 | 25 | 4♠ | 4♠ |
| Q743 4 A8652 J73 | 7 | 22 | 4♠ | 4♠ |
| K965 A 7543 8632 | 7 | 21 | 4♠ | 4♠ |
| K74 A52 8632 943 | 7 | 18 | Pass | Pass |
| J84 KQ5 QJ63 T52 | 9 | 18 | Pass | Pass |
| Q86 J83 KQ54 J72 | 9 | 18 | Pass | Pass |

# 6. Common mistakes

1. **Counting controls twice.** A = 6 already includes the control. Don't add 2 more.
2. **Shape number with a void.** Shortest = 0: 5-5-3-0 is 10 + 5 − 0 = 15.
3. **Stacking methods.** Do not add LTC, "length points" or shortness points on top of
   Zar; its shape number already contains them.
4. **Mixing up vulnerability.** Non-vulnerable is *stricter* (34 for game), not looser.
5. **Using it outside its scope.** Competition, slam bidding, minor-suit fits, NT
   decisions: Zar is not our tool there (see §8).

# 7. Known weak spots (where Zar can mislead)

- **Big two-suiters with few points.** ♠KQJ853 ♥AQ872 ♦4 ♣2 is only Zar 31, but game
  made in about 80% of layouts. Vulnerable we invite (partner usually accepts); non-vul
  Zar says pass, which is a known loss case. Judgment to upgrade 6-5 hands is allowed
  but untested.
- **An honour in a singleton** counts in full in Zar but is worth about half.
  ♠AQ852 ♥K ♦KJ43 ♣872 and ♠AQ852 ♥7 ♦KJ43 ♣K72 both count 30; the second is the
  better hand (game 32% vs 13% of layouts). Both are passes, but be cautious with
  singleton kings and queens near the line.
- **Tens and nines** are invisible to Zar. They matter more in NT than in suit play.
- **Solid trump honours** earn no extra opposite known 3-card support. Don't upgrade
  for "AKQ in trumps".

# 8. Petkov's wider Zar system (reference only — not our agreement, untested by us)

Zar Petkov also uses Zar points for opening (26+), responding (16+) and the general
game level (52 combined), with fit adjustments (extra trumps, trump honours). His
sources disagree on the fit bonuses, and we have **not** tested any of this. Our
agreement uses only the formula and the thresholds in §3 and §4.

# 9. Drill (answers at the bottom — cover them)

For each hand: Zar? Action vulnerable? Action not vulnerable? (You opened 1♠ or 1♥,
partner raised once, opponents silent.)

1. ♠AKJ84 ♥A9653 ♦K2 ♣6
2. ♠KQ953 ♥KQ2 ♦QJ32 ♣6
3. ♠QJ9632 ♥AK4 ♦A7 ♣J5
4. ♠AJ953 ♥A42 ♦A732 ♣6
5. ♠AQ852 ♥KJ4 ♦Q73 ♣K2
6. ♠KJ852 ♥AQ43 ♦Q2 ♣72
7. ♠KQJ853 ♥AQ872 ♦4 ♣2

Responder (partner invited with 3♠):

8. ♠K74 ♥A52 ♦8632 ♣943
9. ♠Q743 ♥4 ♦A8652 ♣J73
10. ♠J84 ♥KQ5 ♦QJ63 ♣T52
11. ♠K965 ♥A ♦7543 ♣8632
12. ♠A73 ♥9 ♦Q8642 ♣K753

Re-test after a week with the suits permuted.

\newpage

# Memory card

```
ZAR  =  A6 K4 Q2 J1  +  (2 x longest + 2nd longest - shortest)

After 1M - 2M (uncontested):        VUL         NONVUL
   game (4M)                        33+         34+
   invite (3M)                      31-32       32-33
   pass                             30-         31-

After 1M - 2M - 3M, responder:      accept 20+  accept 19+

Never add LTC or extra shape points on top.  Only this auction.
```

\newpage

# Drill answers

**Answers:** 1. Zar 35: 4♠ / 4♠. 2. 28: pass / pass. 3. 33: 4♠ / 3♠. 4. 32: 3♠ / 3♠.
5. 30: pass / pass. 6. 27: pass / pass. 7. 31: 3♠ / pass (weak spot — see §7).
8. 18: pass. 9. 22: 4♠. 10. 18: pass. 11. 21: 4♠. 12. 25: 4♠.
(All Zar values computed by `eval_lab eval`; game frequencies by `eval_lab mc`, 200
layouts each, double-dummy.)
