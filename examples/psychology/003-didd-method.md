# DIDD Method — Distribute, Infer, Decide, Do

## The Concept

DIDD is a systematic four-step process for reading the unseen hands.
It turns "guessing" into structured deduction. By trick 8-9, you
should know all four suit lengths for both defenders. Most bridge
"guesses" aren't guesses — the data has already narrowed the layout.

---

## Deal

```
            North (Dummy)
            ♠ A Q 6 3
            ♥ K 7 4
            ♦ 10 8 5
            ♣ Q 9 3

West                        East
♠ J 10 9 5                  ♠ 8 7
♥ Q 10 8 6                  ♥ J 9 5 2
♦ A K 3                     ♦ J 9 7 4
♣ 7 4                       ♣ 10 8 6

            South (You)
            ♠ K 4 2
            ♥ A 3
            ♦ Q 6 2
            ♣ A K J 5 2
```

Contract: **3NT by South**

Auction: 1♣ – Pass – 1♠ – Pass – 1NT – Pass – 3NT – All Pass

---

## Step 1: DISTRIBUTE (Gather Clues)

**From the bidding:**
- Both opponents passed throughout → neither has 12+ HCP
- West didn't overcall → no good 5-card suit with values
- East didn't compete → limited values

**From the opening lead:**
West leads ♦K (from AK). You see ♦1085 in dummy, ♦Q62 in hand.

**Trick 1**: West ♦K. Dummy ♦5. East ♦4 (lowest = odd count → 1 or
3 diamonds). You ♦2 (ducking to preserve ♦Q as a later stopper).

**Trick 2**: West continues ♦A. Dummy ♦8. East ♦7 (second card,
confirms count: 4→7 low-high, but wait — East played 4 then 7.
Low-high = even = 4 diamonds). You ♦6.

**Revision**: East has 4 diamonds (♦J974). West has 3 diamonds
(♦AK3). West's diamond suit is only 3 cards — that's why they
didn't overcall (short suit, no source of tricks).

**Trick 3**: West switches to ♠J (top of sequence J1095).

---

## Step 2: INFER (Construct a Hypothesis)

**What you know by trick 3:**

| Suit | West | East |
|---|---|---|
| ♦ | AK3 (3 cards) | J974 (4 cards) |
| ♠ | J109x (4+ cards, sequence lead) | ? |
| ♥ | ? | ? |
| ♣ | ? | ? |

West has 3 diamonds and 4+ spades → at most 6 cards in hearts + clubs.

**HCP check**: West has ♦AK (7 HCP) + ♠J10 (2 HCP) = 9 HCP visible.
West passed, so max ~11 HCP total. That leaves 0-2 HCP unaccounted
for (a Queen or a couple of Jacks at most).

**Hypothesis**: West is 4-4-3-2 or 4-3-3-3. With ♦AK and ♠J109x,
West has at most ♥Q or ♣J as remaining honours.

---

## Step 3: DECIDE (Test the Hypothesis)

You need 9 tricks: ♠AKQ (3) + ♥AK (2) + ♣ tricks (4 needed → all 5
club tricks). The club suit needs to run for 5 tricks.

**The club question**: you have AKJ52 opposite Q93. Missing: ♣10876.
Normal play: cash ♣A, ♣K, finesse ♣J if needed. But which defender
has ♣10?

**Test your hypothesis**: West has 2 clubs (4-4-3-2 shape). East
has 3 clubs. If West has ♣10x, the finesse works. If East has ♣10xx,
the finesse still works (you finesse against East).

Wait — you're finessing the ♣J through EAST. If East has ♣1086,
you play ♣A, ♣K (both follow), then ♣J — East covers ♣10? No, East
plays ♣6, your ♣J wins.

But FIRST, let's gather more data before committing.

**Trick 3**: You win ♠A in dummy.

**Trick 4**: Dummy leads ♥4. East ♥2. You win ♥A. West ♥6.

**Trick 5**: You lead ♣2. West ♣4. Dummy ♣Q. East ♣6.

**Trick 6**: Dummy leads ♣3. East ♣8. You ♣K. West ♣7.

**Updated count**: West followed to 2 clubs (♣74 → doubleton).
East has followed to 2 clubs so far, has 1 more (♣10 remaining).

---

## Step 4: DO (Execute the Winning Line)

West is out of clubs. East has ♣10 remaining. You hold ♣AJ5.

**Trick 7**: You lead ♣5. West discards ♥8 (confirming out of clubs).
You finesse ♣J in... wait, you already played ♣Q and ♣K. You hold
♣AJ5 still? Let me recount.

♣ holding: AKJ52 (hand) vs Q93 (dummy).
Trick 5: led ♣2 → West ♣4, dummy ♣Q, East ♣6.
Trick 6: dummy ♣3 → East ♣8, you ♣K, West ♣7.

You now hold ♣AJ5. East has ♣10 remaining.

**Trick 7**: You lead ♣5. West discards (out of clubs). Dummy plays
♣9. East plays ♣10. You let East win? No — you're leading from hand.
Lead ♣J → West discards, East must play ♣10. ♣J wins.

**Trick 8**: ♣A clears the suit. You cash ♣5 for trick 9.

**9 tricks: ♠A, ♥A, ♣AKQJ5.** Contract made.

---

## The DIDD Summary

```
DISTRIBUTE: ♦AK3 with West (count signal), ♠J109x with West (lead)
INFER:      West is 4-4-3-2, max 11 HCP → ♣ doubleton
DECIDE:     Play clubs from the top, then finesse against East's ♣10
DO:         Cash ♣Q, ♣K, then run ♣J through East
```

Without DIDD, you might guess the club position wrong — or worse,
not even realise there was a position to read.

---

## Key Takeaway

DIDD converts chaos into structure. By trick 3 you have the opening
lead (shape), count signals (length), and bidding negatives (HCP
limits). By trick 6, shape is nearly complete. The "guess" at trick
7 is no longer a guess — it's a deduction. Build the picture one
clue at a time, and the answer reveals itself.
