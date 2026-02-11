# Worth Investigating: System Extensions

> Five practical gaps that cost tricks at the table. These are the next priorities for system development.

---

## 1. Weak Two Enquiry (Ogust 2NT Ask)

**Problem:** Partner opens 2♡ (weak). Responder has game interest but no way to explore.

After 2♡/2♤ - **2NT** (enquiry):

| Opener's Rebid | Meaning                                |
| -------------- | -------------------------------------- |
| 3♧             | Minimum (6-7), poor suit (one honour)  |
| 3♢             | Minimum (6-7), good suit (two+ honours)|
| 3♡             | Maximum (8-10), poor suit              |
| 3♤             | Maximum (8-10), good suit              |

The strength/quality boundaries match our disciplined weak two requirements. The combination of min/max and suit quality tells responder exactly whether to bid game.

**Why it matters:** Without this, responder guesses. With it, responder knows.

---

## 2. Super-Accepts on 1NT Transfers

**Problem:** After 1NT - 2♢ (transfer to hearts), opener always completes with 2♡. Responder with an invitational 5-card major is guessing whether opener's 12-14 is working for them.

| Opener's Action | Meaning                                          |
| --------------- | ------------------------------------------------ |
| 2♡              | Default completion. Any hand.                    |
| 3♡              | Maximum (13-14), 4-card support. "I like this."  |
| 2NT             | Maximum (13-14), doubleton support. Points but no fit. |

Same applies after 1NT - 2♡ (transfer to spades):

| Opener's Action | Meaning                                          |
| --------------- | ------------------------------------------------ |
| 2♤              | Default completion.                              |
| 3♤              | Maximum, 4-card support.                         |
| 2NT             | Maximum, doubleton support.                      |

**Why it matters:** Responder with ♠Qxxxx ♡Kx ♢Axx ♧xxx (invitational) can pass 2♤ or bid game over 3♤ — no guessing.

---

## 3. Defense to Opponents' 1NT Opening

**Problem:** RAPTOR handles opponents' suit openings. When they open 1NT, the system has nothing.

| Action  | Meaning                                          |
| ------- | ------------------------------------------------ |
| X       | Penalty (15+ HCP)                                |
| 2♧      | Both majors (5-4 either way, 10+ HCP)            |
| 2♢      | One major, 6+ cards (partner bids 2NT to ask which) |
| 2♡ / 2♤ | Natural, good 6+ card suit                       |
| 2NT     | Both minors (5-5+)                               |

The 2♧/2♢ split handles the common "I have a major but which one?" problem. Natural 2-level overcalls cover strong single-suited hands.

**Why it matters:** Opponents open 1NT frequently. Having no defense means you pass hands where you should compete.

---

## 4. Jump Responses to 1-Level Openings

**Problem:** Partner opens 1♡. Responder bids 3♧. Is it weak? Strong? Invitational? The system doesn't say.

### After 1♡ or 1♤ opening:

| Response | Meaning                                                     |
| -------- | ----------------------------------------------------------- |
| 2NT      | Forcing raise, 4+ trump support, game values                |
| 3♧ / 3♢  | Fit jump: good suit + 3-card support, invitational          |
| 3♡ / 3♤  | Preemptive raise, 4+ trumps, weak (Law of Total Tricks)    |
| 4♡ / 4♤  | To play, 5+ trumps, distributional, not slam interest       |

### After 1♧ or 1♢ opening:

| Response | Meaning                                                     |
| -------- | ----------------------------------------------------------- |
| 2NT      | Invitational, balanced, 11-12 HCP                           |
| 3♧ / 3♢  | Invitational raise, 5+ card support, 10-12 HCP              |
| Jump in major | Strong, 5+ cards, game-forcing                         |

**Why fit jumps work here:** Opener's 5+ card promise means 3-card support guarantees an 8-card fit. The fit jump shows both a side suit (lead direction) and support in one bid.

---

## 5. 2♧ Type 3: Strong Two-Suiters (18-21, 5-5+)

**Problem:** Type 3 is empty. The 4441 approach was rejected (too rare at 17+ HCP). But strong two-suiters are more common and genuinely hard to bid from the 1-level.

**Example:** ♠x ♡AKQxx ♢AKJxx ♧xx (18 HCP, 5-5)

Opening 1♡ risks partner passing with 6 HCP and missing game. Opening 2♢ (game-force) overstates it. Type 3 fills the gap.

After 2♧ - 2♢ - **3 of a suit** = 5-5+, the bid suit and a higher suit, 18-21 HCP:

| Rebid | Meaning                                      |
| ----- | -------------------------------------------- |
| 3♧    | Clubs + a higher suit (3♢ asks which)        |
| 3♢    | Diamonds + a major (3♡ asks which)           |
| 3♡    | Hearts + spades                              |

Responder picks the fit or bids 3NT. The 3-level start leaves room for exploration.

**Why it matters:** These hands occur more often than 4441 with 17+ and more often than the balanced 22-23 (Type 1). Filling Type 3 makes 2♧ earn its place in the system.

---

## Priority Order

| # | Extension                     | Frequency at table | Impact |
|---|-------------------------------|--------------------|--------|
| 1 | Weak two enquiry (Ogust)      | Every weak two     | High   |
| 2 | Defense to opponents' 1NT     | Very common        | High   |
| 3 | Super-accepts on transfers    | Common             | Medium |
| 4 | Jump responses                | Moderate           | High   |
| 5 | 2♧ Type 3 two-suiters        | Rare               | Medium |
