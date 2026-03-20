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
            ♠ Q J 10 9
            ♥ 8 5 3
            ♦ K J 7 4
            ♣ Q 6

West (Partner)              East (You)
♠ 8 6 4 2                   ♠ 7 3
♥ Q 10 4                    ♥ A K J 9 2
♦ 9 5 3                     ♦ 10 8 2
♣ A 9 5                     ♣ 10 7 3

            South (Declarer)
            ♠ A K 5
            ♥ 7 6
            ♦ A Q 6
            ♣ K J 8 4 2
```

Contract: **3NT by South**

Auction: 1♣ – Pass – 1♠ – Pass – 2NT – Pass – 3NT (all pass)

---

## The Play

**Trick 1**: Partner leads ♥4 (4th best from Q104). Dummy ♥3.
You play ♥9 (third hand high, cheapest winner from AKJ92).
Declarer plays ♥6.

**Trick 2**: You cash ♥A. Declarer ♥7, partner ♥10, dummy ♥5.

**Trick 3**: You cash ♥K. Partner ♥Q (unblocking), dummy ♥8,
declarer discards ♣2.

**Trick 4**: You play ♥J. Dummy discards ♠9. Declarer discards ♣4.

Partner must discard. Four hearts cashed — you need ONE more trick
to beat 3NT. You still have ♥2 to lead. But which suit?

Partner holds: ♠864, ♦953, ♣A95. The ♣A is the setting trick —
but only if YOU lead a club next.

---

## The Italian Discard

**Partner's discard: ♣5** (ODD card in clubs = "I want clubs").

You read it: odd club = partner has club values. You lead ♣3.

Declarer plays ♣J. Partner wins ♣A.

**Defence: 4 hearts + ♣A = 5 tricks. Down 1.**

---

## The Wrong Switch

Without the Italian discard, you might lead ♦2 (hoping partner has
♦K or ♦Q). Declarer wins ♦A, establishes clubs (♣K draws out
nothing), and runs: ♠AKQJ + ♦AKQ + ♣KJ8 = 9+ tricks. **Contract
makes.** Partner's ♣A is stranded — declarer never leads clubs
toward partner.

The club switch was the ONLY defence. Partner's ♣5 found it.

---

## The Even Card Redirect

What if partner's clubs were ♣A82 (all even — no odd card available)?

Partner can't discard an odd club. Instead, they redirect from
another suit:

**♠2** (EVEN LOW in spades) = "I want the LOWER of the remaining
suits (♦ and ♣)." Lower = clubs. Same message, indirect route.

The odd card is always clearest when available. Even cards are the
fallback — they point away from the discarded suit toward one of the
other two.

---

## The Encoding Explained

Suppose the three side suits are ♠ (high), ♦ (middle), ♣ (low).

| Your Discard | Meaning |
|---|---|
| Odd ♠ (♠3, ♠5, ♠7, ♠9) | Want spades |
| Odd ♦ (♦3, ♦5, ♦7, ♦9) | Want diamonds |
| Odd ♣ (♣3, ♣5, ♣7, ♣9) | Want clubs |
| Even low ♠ (♠2, ♠4) | Want the LOWER of ♦/♣ = clubs |
| Even high ♠ (♠8, ♠10) | Want the HIGHER of ♦/♣ = diamonds |
| Even low ♦ (♦2, ♦4) | Want the LOWER of ♠/♣ = clubs |
| Even high ♦ (♦8, ♦10) | Want the HIGHER of ♠/♣ = spades |

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
redirects. More information per card than standard discards, and
the adversarial cost is low — declarer sees the discard but still
has to guess whether it's a genuine signal or a deceptive one.
