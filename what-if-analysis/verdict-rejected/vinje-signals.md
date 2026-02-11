# What If: Vinje Distribution Signals?

> **Proposal:** Replace the standard trump echo with Vinje signals, which communicate the parity of the entire hand (one even suit + three odd, or one odd suit + three even).
>
> **Verdict: Rejected.** Theoretically powerful, practically fragile. Loses the trump echo for an abstract signal that requires real-time calculation.

---

## How Vinje Works

Every 13-card hand has exactly one of two parity types:

- **Even parity:** one even-length suit, three odd-length suits (e.g., 4-3-3-3, 5-4-3-1, 6-3-3-1)
- **Odd parity:** one odd-length suit, three even-length suits (e.g., 4-4-3-2, 5-4-2-2, 6-4-2-1)

The suit that's different from the other three is the **"unique suit."**

**In trumps:** high-low = even parity, low-high = odd parity. Partner combines this with bidding, dummy, and their own hand to reconstruct declarer's entire distribution.

## When It's Brilliant

Partner signals even parity (one even suit, three odd). You see dummy is 3-4-3-3. You hold 4-3-2-4. From the parity signal plus what you know, you work out partner's exact shape — and therefore declarer's. You defend double-dummy.

## Why It's Impractical

### 1. You know the parity type but not WHICH suit is unique

The signal says "one even, three odd" — but which of four suits is the even one? You have to infer it from everything else. At the table under time pressure, this calculation is hard. Lee Goodman tested it and reported: "we didn't think it did us any good once we had the information since it did not identify the odd or even suit."

### 2. You lose the trump echo

Standard trump high-low = "I have a third trump, give me a ruff." This is immediately actionable. Vinje replaces it with distributional parity — an abstract, inferential message instead of a concrete, urgent one. On many hands, the ruff signal saves or costs a trick. The distribution signal rarely has that immediate impact.

### 3. Signal availability

Need two trumps with usable spot cards. Roughly half the time you can't give the signal. When you can, declarer sees it too and can use the same inference.

### 4. Contradicts the system's carding philosophy

The All Fives defensive carding section says: "Use your head first, not partner's signals. Signals are a last resort." Vinje is the ultimate signal-first approach — it asks defenders to process a mathematical lookup table at the table instead of thinking about the hand.

The defenders who would benefit most from Vinje are already good enough to work out distributions without it. The defenders who need help won't be able to process the parity inference at the table.

## Comparison

| Feature                    | Trump Echo (current) | Vinje Signal            |
| -------------------------- | -------------------- | ----------------------- |
| Information                | Trump count (ruff?)  | Entire hand parity      |
| Immediately actionable     | Yes                  | Rarely                  |
| Requires calculation       | No                   | Yes (lookup table)      |
| Declarer reads it          | Simple               | Same complexity         |
| Available on every hand    | Most hands           | ~50% of hands           |
| What you lose              | Nothing              | The trump echo          |

## Verdict

Mathematically elegant but wrong for this system. The current attitude + count + Italian discards already give partner what they need at critical moments, without sacrificing the trump echo. Vinje trades something concrete and useful for something abstract and fragile.
