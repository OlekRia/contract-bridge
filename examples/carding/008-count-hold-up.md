# Parity Count Signal — Hold-Up Timing Against a Running Suit

## The Rule

Highest EVEN spot = EVEN count. Highest ODD spot = ODD count.
Second card same parity: high→low = TRUE, low→high = FALSE.

When dummy has a threatening long suit and partner holds a stopper,
give COUNT so partner knows WHEN to win. Duck too long = the suit
runs. Win too early = declarer re-establishes it.

---

## Deal

```
            North (Dummy)
            ♠ 7 5
            ♥ 8 3
            ♦ A K Q 8 7 3
            ♣ J 6 4

West (You)                  East (Partner)
♠ Q 10 8 3                  ♠ K J 9 2
♥ A K 10 6 2                ♥ J 7 4
♦ 10 4                      ♦ J 9 5
♣ 9 3                       ♣ Q 10 2

            South (Declarer)
            ♠ A 6 4
            ♥ Q 9 5
            ♦ 6 2
            ♣ A K 8 7 5
```

Contract: **3NT by South**

Auction: 1♣ – Pass – 1♦ – 1♥ – 1NT – Pass – 3NT (all pass)

---

## The Situation

You (West) lead ♥6 (4th best). Partner plays ♥J, declarer wins ♥Q.

Declarer leads ♦2 toward dummy's ♦AKQ873.

Partner holds ♦J95 — the stopper. But when should partner win?

- If partner wins ♦J on the first round: declarer has ♦6 left.
  Declarer re-enters dummy (♣J?) and runs ♦AKQ87 = 5 diamond tricks.
  Contract makes easily.

- If partner ducks twice and wins on the third round: declarer has
  no diamonds left. Dummy's ♦Q873 are stranded if dummy has no side
  entry. But does dummy have an entry?

Dummy's only possible entry is ♣J. If partner uses up declarer's
♦62 (a doubleton), dummy's diamonds are stranded.

**The key question**: how many diamonds does declarer have?

---

## The Count Signal

**Trick 2**: Declarer leads ♦2. Dummy plays ♦A. Partner plays ♦5
(highest odd spot = odd count = 3 diamonds).

You hold ♦104. Play highest even spot.

**Your play: ♦10** (even spot = even count = doubleton).

Partner now knows: you have 2 diamonds (even), dummy has 6, declarer
has (13 − 6 − 2 − 3) = 2 diamonds. Declarer has a doubleton!

**Trick 3**: Declarer leads ♦6. Dummy plays ♦K. Partner plays ♦9
(high→low: ♦5 then... wait, ♦9 is odd like ♦5 — and 9 > 5, so
this is low→high same parity. FALSE signal?).

No — partner played ♦5 first, then ♦9. That's low→high, both odd
= FALSE. But partner has 3 diamonds (odd) and wants to tell the
truth. So partner plays ♦9 then ♦5 — high→low — wait, partner
can't choose the order freely; they play as dummy calls the suit.

**Clarification**: In follow-suit situations, partner plays what
they can. The truth check mainly applies when partner has free
choice over two tricks. Here, partner's ♦5 at trick 2 (odd = odd
count) is the primary signal. The ♦9 at trick 3 is just following
suit. You show out (confirming your doubleton — you discard ♠3).

**Trick 4**: Dummy leads ♦Q. Partner wins ♦J!

Partner timed the hold-up perfectly: duck round 1, duck round 2,
win round 3. Declarer has no more diamonds. Dummy's ♦873 are
stranded — dummy has no side entry (♣J is blocked under ♣AK).

Partner returns ♥7. Your ♥AK106 cash. Down 2.

---

## What If You Had 3 Diamonds?

If you held ♦1064 (3 cards), you'd play ♦ highest odd spot. But
your odd cards are... none visible. ♦10 is even, ♦6 is even, ♦4
is even. All your diamonds are even-ranked!

Play ♦4 (lowest even — an even card from an odd holding is itself
a signal: you couldn't find an odd card, so partner infers from
context). Or use the truth check: ♦4 then ♦10 (low→high, both
even = FALSE even → actually odd). Partner calculates: you have 3
diamonds, dummy has 6, partner has 3, declarer has 1. Duck ONCE,
win round 2.

---

## The Mathematics

Partner's hold-up formula against a dummy suit:
- Count declarer's diamonds = 13 − dummy's − yours − partner's
- Duck exactly (declarer's count − 1) rounds
- Win on the round that exhausts declarer's last diamond

Your parity count signal makes this calculation possible.

---

## Key Takeaway

Against a threatening long suit in dummy, count is far more valuable
than attitude. Partner already knows they need to hold up — what
they don't know is HOW LONG to duck. Your parity count signal tells
them declarer's length, which determines the exact hold-up timing.
The even/odd spot card gives the initial read; the truth check on
the second card confirms or denies when partner has free choice.
