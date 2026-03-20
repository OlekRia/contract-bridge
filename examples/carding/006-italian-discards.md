# Italian Discards — Suit Preference on Your First Discard

## The Rule

When you must discard (can't follow suit), your first discard uses
Italian Lavinthal encoding:

- **ODD card** in a suit = I WANT this suit (interest)
- **LOW EVEN card** in a suit = I want the LOWER of the other two suits
- **HIGH EVEN card** in a suit = I want the HIGHER of the other two suits

Italian discards carry ~1.58 bits — more information than binary
attitude (1.0 bit). They encode WHICH suit you want, not just
"like / dislike" in one suit.

---

## Deal

```
            North (Dummy)
            ♠ K Q J 10
            ♥ 8 5 3
            ♦ A K Q 6
            ♣ 7 4

West (Partner)              East (You)
♠ 8 7 5 3                   ♠ 9 4
♥ Q 10 4                    ♥ A K J 9 2
♦ 9 8 5                     ♦ 10 7 3
♣ J 10 2                    ♣ 9 6 5

            South (Declarer)
            ♠ A 6 2
            ♥ 7 6
            ♦ J 4 2
            ♣ A K Q 8 3
```

Contract: **3NT by South**

Auction: 1♣ – Pass – 1♠ – Pass – 1NT – Pass – 3NT (all pass)

---

## The Play

**Trick 1**: Partner leads ♥4 (4th best). Dummy ♥3. You play ♥9
(third hand high from AKJ92 — win cheaply). Declarer follows ♥7.

**Trick 2**: You cash ♥A. Declarer ♥6, partner ♥10, dummy ♥5.

**Trick 3**: You cash ♥K. Declarer discards ♣3, partner ♥Q, dummy ♥8.

**Trick 4**: You play ♥J. Declarer discards ♣8, dummy discards ♠10.

Partner must discard. Partner holds: ♠875 ♦985 ♣J102.

The defence has cashed 4 heart tricks. You need one more trick to
beat 3NT. Where is it? Partner holds ♣J102 — the ♣J could be a
trick if declarer can't finesse against it.

But partner needs to tell you what to lead next (you still have ♥2).

**Partner's discard: ♣J** — ODD card in clubs = "I want clubs."

Wait — the ♣J is a potential trick card! Is that wise?

Actually, from partner's perspective: ♣J102 is under declarer's ♣AKQ.
The Jack will never win a trick — declarer has AKQ. So the ♣J is a
FREE card for signaling. Partner plays it as an odd card to say
"lead a club" — or more precisely, "I have something in clubs."

Hmm, that's actually the wrong message here. Let me reconsider.

---

## Better Example: Partner Wants Diamonds

Let's say instead partner holds ♠875 ♦K95 ♣1082 and needs a diamond
return.

**Partner's discard: ♦5** (ODD card in diamonds = "I want diamonds").

You lead a diamond. Partner's ♦K wins. Down 1.

---

## The Encoding Explained

Suppose the three side suits are ♠ (high), ♦ (middle), ♣ (low).
You must discard from one of them.

| Your Discard | Meaning |
|---|---|
| Odd ♠ (♠3, ♠5, ♠7, ♠9) | Want spades |
| Odd ♦ (♦3, ♦5, ♦7, ♦9) | Want diamonds |
| Odd ♣ (♣3, ♣5, ♣7, ♣9) | Want clubs |
| Even low ♠ (♠2, ♠4) | Want the LOWER of ♦/♣ = clubs |
| Even high ♠ (♠8, ♠10) | Want the HIGHER of ♦/♣ = diamonds |
| Even low ♦ (♦2, ♦4) | Want the LOWER of ♠/♣ = clubs |
| Even high ♦ (♦8, ♦10) | Want the HIGHER of ♠/♣ = spades |

The odd card is simplest: discard an odd card in the suit you want.
Even cards are a redirect — they point away from the discarded suit
toward one of the remaining two.

---

## When to Use Italian Discards

1. Your FIRST discard only (subsequent discards revert to count/attitude)
2. When you cannot follow suit
3. Use the cheapest card that sends the right message
4. ODD in the wanted suit is always clearest — prefer it when available

---

## Key Takeaway

Italian discards solve the discard problem: "which suit should
partner attack?" An odd card says "I want THIS suit." An even card
redirects. More information per card than standard discards, at
no extra adversarial cost — declarer already sees the discard.
