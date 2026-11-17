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
