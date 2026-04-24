# Parity Count to Time Your Ace — When to Win, When to Duck

## The Concept

Declarer leads a long suit to establish it. You hold the Ace — the
only stopper. Duck too early, and declarer re-enters to cash winners.
Win too late, and the suit runs before you can act.

Partner's parity count signal tells you how many rounds to duck.

---

## Deal

```
            North (Dummy)
            ♠ 8 5
            ♥ 9 6 3
            ♦ K Q J 10 6
            ♣ A 7 4

West (Partner)              East (You)
♠ K J 10 7 3                ♠ Q 9 6 2
♥ Q J 10 4                  ♥ A 8 7
♦ 7 3                       ♦ A 8 5
♣ 8 5                       ♣ J 9 3

            South (Declarer)
            ♠ A 4
            ♥ K 5 2
            ♦ 9 4 2
            ♣ K Q 10 6 2
```

Contract: **3NT by South**

Auction: 1♣ – Pass – 1♦ – 1♠ – 1NT – Pass – 3NT (all pass)

---

## The Play

**Trick 1**: Partner leads ♠7 (4th best from KJ1073). Dummy ♠5.
You play ♠Q (third hand high). Declarer wins ♠A.

**Trick 2**: Declarer leads ♦2 toward dummy's ♦KQJ106.

You hold ♦A85. Dummy has 5 diamonds. If declarer has 3 diamonds,
you should duck TWICE and win on the third round — declarer's
diamonds are exhausted, dummy's remaining diamonds are stranded
(dummy has no side entry after ♣A is used).

If declarer has only 2 diamonds, you should duck ONCE and win on
the second round.

**How do you know?** Partner's parity count signal.

Dummy plays ♦K. Partner holds ♦73 — 2 diamonds (even). Play
highest even spot.

**Partner plays ♦ ... wait, ♦7 is odd, ♦3 is odd.** Both of
partner's diamonds are odd-ranked! With no even spot card available,
partner plays ♦7 (highest card — context tells you it's from a
short holding, likely doubleton).

**You read: ♦7 is odd spot = odd count signal.** But is it true?
Context helps: dummy has 5, you have 3. If partner has 3 (odd),
declarer has 2. If partner has 2, declarer has 3. You need to
consider which is more consistent with the auction and play.

Declarer opened 1♣ (clubs) and responded 1NT — likely balanced
without long diamonds. Partner's ♦7 from a doubleton is signalling
with the only card available. The auction + ♦7 (high card from
short holding) suggests doubleton.

You calculate: dummy 5 + you 3 + partner 2 = 10. Declarer has 3
diamonds.

**You duck.** ♦K wins.

**Trick 3**: Dummy leads ♦Q. Partner plays ♦3 (last diamond —
♦7 then ♦3 = high→low, both odd = TRUE odd count).

But wait — partner has 2 diamonds (even), yet signalled odd. The
truth check reveals: ♦7→♦3 is high→low same parity = TRUE. So
the odd signal was "true odd." But partner actually has EVEN (2).

**This is the limitation**: when all your cards have the wrong
parity for your actual count, you signal what you can and partner
uses context (auction, hand pattern) to resolve the ambiguity.
Here, partner's show-out on round 3 would confirm the doubleton.

**You duck again.** ♦Q wins.

**Trick 4**: Dummy leads ♦J. **NOW you win ♦A.**

Declarer's last diamond (♦9) was played on this trick. Dummy still
has ♦106, but declarer has NO diamonds left. The only way to reach
dummy is ♣A — one entry, but the diamonds are already established.

Declarer plays ♣ to dummy's ♣A, cashes ♦106 = 4 diamond tricks
total. But defense has ♠KJ103 + ♥QJ10 ready to run.

**Trick 5**: You return ♠9. Partner runs spades: ♠K, ♠J, ♠10, ♠3.

**Defence: ♦A timed perfectly + 4 spade tricks = down 1.**

---

## The Wrong Timing

**If you win ♦A on the first round**: declarer enters via ♣A later,
runs ♦QJ106 = 4 diamond tricks. With ♣KQ and ♠A already cashed,
that's 9 tricks. Contract makes.

**If you win ♦A on the second round**: declarer still has ♦9.
Enters dummy via ♣A, plays ♦J (you played ♦A already). Then ♦106
cash. Same result — 4 diamond tricks.

**Only winning on the third round works**: declarer is out of
diamonds. Dummy's ♦106 cash only via ♣A (one entry = 2 tricks, not 4).

---

## The Count Arithmetic

Formula: Partner's count reveals declarer's length.

```
Declarer's diamonds = 13 − dummy(5) − you(3) − partner(2) = 3
Duck exactly (declarer's count − 1) = 3 − 1 = 2 rounds
Win on round 3
```

Partner's parity signal plus auction context makes this calculation
possible.

---

## Parity Limitation: Wrong-Parity Holdings

When ALL your cards share the wrong parity for your actual count
(e.g., ♦73 = both odd, but you have 2 = even), you signal with
what you have. Partner resolves ambiguity via:
1. **Context** — auction, dummy, hand patterns
2. **Truth check** — if a second card is played, the direction
   (high→low vs low→high) confirms or denies
3. **Show-out** — failing to follow suit is the ultimate count signal

This is rare (most holdings have mixed parity), but when it happens,
bridge judgement fills the gap.

---

## Key Takeaway

The Ace is not just a trick — it's a TIMER. Partner's parity count
signal starts the clock. Calculate declarer's length, duck the right
number of rounds, and strand the suit. Even when the parity signal
is imperfect, combining it with auction inference gives you the
answer.
