# 02 — Anomaly Audit: H-8, H-31, H-34 and Boards T12 B16, T16 B9/B13/B14

- Audit date: 2026-11-17
- Scope: development examples only, **not** a validation set. Nothing here changes a table
  agreement. Source corrections and proposed treatments go to the Director separately.
- Tools: `bridge-tools check-deal` (`src/bridge-tools/src/sim/check_deal.rs`). HCP 4-3-2-1
  (F-HCP-001); LTC = `holding_ltc` (`src/bridge-tools/src/sim/deal.rs:34-51`): for the first
  min(len, 3) cards, count each missing A/K/Q.

Labels are kept separate:

| Label | Meaning |
|---|---|
| **CURRENT AGREEMENT** | What the live system card, spec, requirements or decisions say today |
| **EXISTING METHOD** | A published or in-repo evaluation method, with its definition |
| **TABLE EVIDENCE** | What governance or post-mortems report happened at the table |
| **COMPUTATIONAL RESULT** | Output of `check-deal` run during this audit |
| **SOURCE ERROR** | A claim in a source that contradicts its own data or the arithmetic |
| **GAP** | Missing source, undefined term, or claim that cannot be checked |
| **ASSESSMENT** | This audit's classification (provisional where marked) |

---

## 0. Source inventory and gaps

**GAP — T16 post-mortem missing.** `contract-bridge/analysis/2026-09-08.md` is absent from the
checkout and has **never existed in the submodule history** (`git -C contract-bridge log --all
-- analysis/2026-09-08.md` returns nothing). Governance cites it at `gov/hypotheses.yaml:309`
(H-8), `:1717` (H-29) and `:1827` (H-30). Every T16 statement below is taken from governance text
only. **Treat it as provisional.** No full T16 deal is available, so no T16 DD claim can be
checked.

**GAP — AMD-31 missing.** H-8 (`hypotheses.yaml:298`), H-29 (`:1709`) and H-30 (`:1816`) cite
"AMD-31" as the T16 cascade. `gov/amendments.yaml` goes from AMD-30 (`:910`) straight to AMD-32
(`:968`). `git log --all -S "id: AMD-31"` is empty, so AMD-31 has never existed. The T16 cascade
(edits to H-8, H-29 and H-30, and the filing of H-34) has no amendment record. It was committed six
weeks after the event, in `0b6bc7b` (2026-10-21, "H-35 refuted ... H-34 filed"). AMD-32
(`amendments.yaml:968-987`) mentions T16 B14 only as context for RKCB. It is not a T16 debrief.

**GAP — H-31 tracker missing.** The protocol log `gov/knowledge/opener-evaluation-log.md`
(`hypotheses.yaml:1951`) does not exist.

**Available T16 card data:** B13 has North only. B14 has North and South. B9 and B6 have no
cards.

---

## 1. Current agreements and methods used in this audit

### CURRENT AGREEMENT (with file:line)

| Topic | Text | Source |
|---|---|---|
| 1-suit opening | "12+ HCP (~7 losers)" | `contract-bridge/all-fives.yaml:78`; verbose `:273,288,303,316` "7 losers typical (range 5-8)" |
| Single raise | "6-9 HCP, 3+ support, 9-10 losers" | `all-fives.yaml:115`; verbose `:421-424` |
| Limit raise | "10-12 HCP, 4+ support, 7-8 losers"; "Opener bids game with 5-6 losers" | `all-fives.yaml:116`; verbose `:425-428` |
| Game raise | "Distributional, 5+ support, 6-7 losers" | `all-fives.yaml:117`; verbose `:429-432` |
| LTC note | "After fit, use LTC: 24 - combined losers = tricks (H-8)" | `all-fives.yaml:118`; verbose `:433-437` |
| Requirement | "Use LTC alongside HCP to evaluate suit contracts with a known fit" | R-BID-015, `gov/requirements.yaml:182-194` |
| Game rule | "Bid game with a combined 25+ HCP and an 8+ card major fit" | R-BID-008, `requirements.yaml:107-118` |
| 1M-3M (1st/2nd) | "Slam try — 4+ trumps, GF, continue with cue-bids"; invitational hands go through forcing 1NT | `all-fives.yaml:124`; verbose `:478-491` |
| 1M-4M (1st/2nd) | "Game raise, no slam interest"; LTC "Mixed — from 5-6 losers (sound) to 8+ losers" | `all-fives.yaml:125`; verbose `:492-499` |
| 2NT LTC ask | Opener shows a side suit with 3 losers, defined as "**xxx or Qxx**" | verbose `:446-471`; `all-fives.yaml:122` |
| Opener 3M after forcing 1NT | "6+ in opened major, invitational" | `all-fives.yaml:101` |
| Slam path (2/1 GF) | "Cuebid controls up the line (bypass=denial), then RKCB 4NT (30-41-22)" | `all-fives.yaml:142`; verbose `:1151-1177` (titled "Slam Exploration **After 2/1 GF**") |
| Benji 2♣ Type 2 | "Strong major, ~5 losers, 5+ suit, 3 top honours" | `all-fives.yaml:149`; verbose `:1229-1234`; R-BID-009 |
| Benji 2♣ Type 3 | Strong two-suiter "~4 losers (≤3 losers → UCM opening instead)", 5-5+ | `all-fives.yaml:151`; verbose `:1236-1239` |
| 2♦ GF | `ltc: "3-4 losers"`, and in the convention list "**GF, 24+ HCP or 4- losers**" | `all-fives.yaml:163` and `:541`; verbose `:1349-1351` "3-4 losers (game in hand or near it)"; R-BID-010 (`requirements.yaml:130-134`) has no HCP or loser criterion |
| UCM 3NT/4♣/4♦ | "5-5+, ≤3 losers" | `all-fives.yaml:216-218`; DEC-12 (`gov/decisions.yaml:270`); R-BID-042 |
| Weak twos | "2H/2S — 5-10 HCP, 6-card major" | `all-fives.yaml:182-183,542` |
| Slam HCP | Small slam ≈ 33, grand ≈ 37 | F-HCP-003, `facts.yaml:181-185` |

### EXISTING METHOD — the LTC definition is not consistent across sources

- F-LTC-001 (`gov/facts.yaml:207-217`) lists void, A, x, AK, Ax/Kx/KQ, xx, AKQ, AKx, Axx, KQx, Kxx
  and xxx. It **does not define Qxx, Qx, singleton K, QJx or KJx**.
- The tool counts Qxx = 2, Qx = 2 and singleton K = 1 (Courtenay/classic).
- The spec's 2NT ask treats **Qxx as a 3-loser suit** (verbose `:449,455-461`), as Klinger's Modern
  LTC does.
- **GAP:** the partnership has no single LTC definition. This matters for B14 (see §4.3).
- F-LTC-002 (`facts.yaml:219-232`) gives "24 − combined losers": 14 = game, 12 = small slam,
  11 = grand. It is classed as a *model* sourced from "table experience". It assumes an 8+ fit
  and says nothing about trump quality.

### Internal conflicts in the current agreements

These are GAPs or ambiguities, not errors at the table.

1. **The generic `raises.limit` does not exist over a 1st/2nd-seat 1M.** There, 1M-3M is a GF slam
   try and limit hands go through forcing 1NT (`all-fives.yaml:116` vs `:124`). Over 1m, 3m needs
   **5+** support (`:130`), but the generic limit raise says 4+. In practice the generic band
   applies only to 3rd/4th-seat majors.
2. **Game raise bands disagree.** `game: "... 5+ support, 6-7 losers"` (`:117`) and H-8's
   `game_raise: "6-7 losers"` conflict with the 1st/2nd-seat 4M, which covers "5-6 to 8+ losers"
   (verbose `:499`).
3. **The limit-raise acceptance rule ignores loser bands.** "Opener bids game with 5-6 losers"
   (verbose `:428`) is right only opposite an 8-loser raise. Opposite a 7-loser raise, a 7-loser
   opener has 24 − 14 = 10 tricks by the spec's own formula.
4. **The 2♦ trigger has three versions.** "3-4 losers" (`:163`, verbose `:1351`), "24+ HCP **or**
   4- losers" (`:541`), and no criterion at all (R-BID-010). The "4-" version overlaps Benji
   Type 3 (~4 losers, 5-5) and UCM (≤3 losers, 5-5+, DEC-12).
5. **The written slam path is scoped to 2/1 GF** (verbose `:1153`) and 1M-3M (`:487-491`). No
   written slam protocol covers auctions such as 1♦-1♠-… or 1♣-…. RKCB is listed globally
   (`all-fives.yaml:558`).
6. **There is no precedence rule when HCP and LTC disagree.** R-BID-015 says "alongside". R-BID-008
   (25 HCP and a major fit → game) is binding at rank 3. The verbose note says "Trust LTC in suit
   contracts" (`:437`). The spec cites an unratified hypothesis, "(H-8)", as the authority for a
   live agreement (`all-fives.yaml:118`).

**SOURCE ERROR — verbose example `all-fives-verbose.yaml:435-436`.** "A hand with **10 HCP** but
7 losers (♠KQxxx ♥x ♦Axxx ♣xxx)". This hand has **9 HCP** (K3 + Q2 + A4). The 7 losers are
correct (1+1+2+3). The example's point survives, but it is also a 1M-3M "limit raise", which does
not exist over a 1st/2nd-seat major (conflict 1).

---

## 2. H-8 — LTC Integration for Suit Raises and Competitive Re-evaluation

`gov/hypotheses.yaml:291-367`

- **Status:** `active`, priority high, `review_by: 2026-12-31` (`:295`). The review was extended
  from 2026-06-01, which is six weeks before today.
- **Exact claim (`:313-316`):** "Adding Losing Trick Count thresholds alongside HCP for suit
  raises, competitive re-evaluation, and slam investigation will improve bidding accuracy in suit
  contracts." The claim gives two examples: 10 HCP with 7 losers reaches game, and 14 HCP with
  8 losers stops in partscore.
- **Thresholds (`:326-343`):**

  | Use | Losers |
  |---|---|
  | Single raise | 9-10 |
  | Limit raise | 7-8 |
  | Game raise | 6-7 |
  | Small slam (combined) | ≤12 |
  | Grand slam (combined) | ≤11 |
  | Benji Type 2 | ~5 |
  | 2♦ GF | ~3-4 |

- **Test (`:351-360`):** 5 sessions. Record HCP and LTC on every suit-raise board. Target: LTC
  correct in 60%+ of disagreement cases.

### Arithmetic check

| Claim | Recomputed | Verdict |
|---|---|---|
| "10 HCP with 7 losers will reach game" | 7 + 7-loser opener = 14 → 10 tricks | consistent |
| "14 HCP with 8 losers will stop in partscore" | 8 + 7 = 15 → 9 tricks; but 14 + 12 = 26 HCP | **conflicts with R-BID-008** |
| B9 "HCP 22 + LTC 7+8=15 both said partscore" | 7 + 8 = 15 → 9 tricks; 22 < 25 | arithmetic correct; hands unavailable (GAP) |
| B13 "HCP 25 … LTC 4+8=12" | N 17 HCP and LTC 4 verified (§4.2); S 8 HCP and LTC 8 unverifiable | N verified, S GAP |
| B14 "HCP 23 … LTC 7+5=12" | N 10 HCP and LTC 7; S 13 HCP and LTC 5 (tool-verified, §4.3) | correct under the classic definition; **13 under the spec's Qxx=3 definition** |
| "LTC vs HCP 1-1 on disagreement boards" (`:305`) | see below | **SOURCE ERROR** |

### Source errors and gaps

**SOURCE ERROR — scoring (`:305`).** H-8 scores B14 "HCP right". HCP 23 said game, and the source
itself says 4♠ fails (DD 9 tricks: `:304`, H-29 `:1711`). By H-8's own test criterion (2), "which
predicted the correct level", **neither** measure was right on B14. HCP missed by 1 trick and LTC
by 3. The correct tally is **LTC 1 – HCP 0 – neither 1**, or "HCP closer" if closeness is the
metric. 1-1 is not a supportable reading.

**GAPs:**

- **Lex superior conflict.** The "14 HCP with 8 losers → partscore" leg conflicts with Active
  R-BID-008 (rank 3, above hypotheses at rank 5). It cannot be applied at the table while H-8 is
  unratified.
- **Unsourced claim.** "Our 4 bottom boards (passing with shape)" (`:322`) cites no boards.
- **Selective logging.** The test plan requires *every* suit-raise board. Only T16 B9, B13 and B14
  were logged ("first three-board table data set", `:298`), roughly 9 months after filing. T16 B6
  (28 HCP, 9-card fit, per H-34 `:2185`) is a fit board that H-8 does not log.
- **Undefined counting rule.** "losers in the trump suit that partner cannot cover must not be
  netted" (`:306-307`) is not written into the thresholds at `:337-339`. The slam threshold still
  reads "Combined 12 or fewer losers" with no trump qualifier.
- **Overlapping bands.** Limit is 7-8 losers and game is 6-7, so a 7-loser hand fits both.

**Conflicts with the current agreement:** see §1 conflicts 1-4 and 6. The "(H-8)" citation in
`all-fives.yaml:118` makes a rank-5 hypothesis the authority for a rank-6 agreement, although
R-BID-015 (rank 3) already authorises using LTC with a fit.

---

## 3. H-31 — Opener Evaluation Boost (6-card major + 18-19 HCP + side AKQ)

`gov/hypotheses.yaml:1898-1967`. T12 board: `contract-bridge/analysis/2026-06-02.md:741-783,
1316-1327`.

- **Status:** `active`, priority medium. **`review_by: 2026-09-30` is overdue by 48 days**
  (`:1902`). The test window was T13-T17, but no exposures are logged and the tracker file does
  not exist.
- **Exact claim (`:1913-1917`):** "With a 6-card major, 18-19 HCP, and a side AKQ or AKQx source
  of tricks … opening 1M and rebidding 2M loses game contracts opposite responding hands with
  6-9 HCP + 2-3 cards in our major." The hypothesis proposes an Acol 2M opening or a jump rebid.
- **Falsifiable prediction (`:1962-1965`):** at least 3 profile hands in 5 events, and 1M-2M
  stays out of game at least 2 of 3 times **"AND game is cold per DD"**.

### T12 Board 16: full deal (West deals, EW vul)

```
              ♠ A93
              ♥ JT85
              ♦ Q4
              ♣ QJ96
  ♠ Q5                    ♠ KT7642
  ♥ 632                   ♥ AKQ
  ♦ T8763                 ♦ A
  ♣ 874                   ♣ KT3
              ♠ J8
              ♥ 974
              ♦ KJ952
              ♣ A52
```

**COMPUTATIONAL RESULT** (`check-deal`, full DD):

| Hand | HCP | LTC | Shape |
|---|---|---|---|
| N | 10 | 9 | 3-4-2-4 |
| E | **19** | **4** | 6-3-1-3 |
| S | 9 | 9 | 2-3-5-3 |
| W | **2** | **11** | 2-3-5-3 |

| DD tricks | N | E | S | W |
|---|---|---|---|---|
| ♣ | 6 | 7 | 6 | 7 |
| ♦ | 6 | 6 | 6 | 6 |
| ♥ | 7 | 6 | 7 | 6 |
| ♠ | 4 | **9** | 4 | **9** |
| NT | 4 | 7 | 4 | 7 |

**East-West make 9 tricks in spades, not 10. 4♠ fails double-dummy.** Likely losers: ♠A, a second
spade (N ♠A93 and S ♠J8 against ♠KT7642/♠Q5), and two clubs. West has no entry to lead toward ♣K.

**TABLE EVIDENCE** (`2026-06-02.md:762-767`): E 1♠+1, 1♠+3, 1♠+2, and ours E 2♠+1 (−140 for NS,
3.0 MP). The 1♠+3 result (10 tricks) needed a defensive error. **Our 2♠+1 = 9 tricks = DD, and
+140 is roughly par for EW.**

### Source errors in H-31 and the T12 report

1. **"DD says EW makes 10 spades (= 4S)"** (H-31 `:1907`; T12 `:770`, "DD: E makes 10 spades").
   **SOURCE ERROR: DD is 9.** The T12 report marks B17 as "verified with check-deal" (`:807`) but
   does not mark B16. The B16 DD figure was never run.
2. **"East holds S=KQ10764"** (H-31 `:1904`; T12 recommendation `:1326`). **SOURCE ERROR.** The
   diagram (`:745-753`) gives East ♠KT7642 and West ♠Q5. The diagram is internally consistent:
   52 unique cards, 19 + 2 + 10 + 9 = 40, matching "NS 19 | EW 21". With ♠KQT764, East would have
   **21** HCP, not 19. The text holding and the stated HCP cannot both be true. Sensitivity check
   with the text holding (E ♠KQT764, W ♠52): DD is still **9** spade tricks, so the conclusion
   does not depend on which reading is right.
3. **"Partner with 9 HCP + Sx + flat shape cannot raise to game"** (`:1909-1910`). **SOURCE
   ERROR:** West holds **2 HCP** (♠Q only). A 9-HCP hand is South, an opponent.
4. **"Three of four EW pairs stopped in 1S+1, 1S+3, 2S+1 (us); only pair 3 reached 1S+3"**
   (`:1905-1907`). The sentence is self-contradictory: it says "reached" for a 1-level contract and
   lists three results for four tables. The T12 report names the EW pairs as "pair 3" and "pair 5"
   (`:771-772`), but the table (`:762-765`) shows those are the **NS** pair numbers. The EW pairs
   were 8 and 4. This is a minor SOURCE ERROR.
5. **"All four EW pairs missed game by underbidding"** (`:1907-1908`). **SOURCE ERROR**, because
   game is not makeable DD.

### Assumptions and their status

- **"Side AKQ source of tricks."** ♥AKQ tripleton is three top tricks with no length winners. That
  is quick tricks, not a source of extra tricks. The profile mixes up "controls/tops" and "long
  side-suit source". (GAP: define "source".)
- **The profile does not fit its own example.** The hypothesis targets responder with 6-9 HCP and
  2-3 trumps. B16's responder had 2 HCP and would pass 1♠ (a 1NT response needs 6+).
- **Unclear range.** "will land game with 12-15 HCP opposite our 18-19" (`:1917`) does not say
  what the 12-15 refers to.

### Conflicts with the current agreement (EXISTING METHOD overlooked)

- **The 1♠ opening did not follow the card's 2♦ rule.** East's hand has **LTC 4**. Under
  `all-fives.yaml:541` ("Strong 2D — GF, 24+ HCP **or 4- losers**") and `:163` ("3-4 losers"),
  the card already routes it to a **2♦ GF opening**.
  - That is "failure to follow an agreement" in the literal sense.
  - **Following it would have scored worse.** A GF auction lands in 4♠, which is −1 vulnerable
    (−100, about 0 MP) instead of +140 (3 MP).
  - So **B16 is counter-evidence to the "4- losers ⇒ 2♦" clause** and to H-34 gate (3).
- **Benji 2♣ Type 2 already exists**, as an "Acol-style" strong major (verbose `:1229-1234`). H-31
  says "Acol 2-bids are reserved … for strong two-suiters" (`:1928-1930`), which is wrong:
  - 2♥/2♠ are weak twos (`all-fives.yaml:182-183`).
  - Strong two-suiters are Benji Type 3 or UCM.
  - East does not qualify for Type 2 anyway: the spade suit has only 1 of the top 3 honours.
  - **SOURCE ERROR** (misstatement of the current agreement).
- **The "jump rebid 3M" already exists.** It is opener's 3M after forcing 1NT, "6+ in opened major,
  invitational" (`all-fives.yaml:101`), and 3M after 2/1 = "6+ extras" (`:138`). H-31's concern
  that 3M is "sometimes used as preemptive (Lebensohl style)" (`:1931`) does not describe this
  system.
- **Evaluation methods agreed with DD on B16.** HCP said partscore (21 combined). LTC said
  4 + 11 = 15 → **9 tricks = DD exactly**. There was no evaluation failure to fix.

### ASSESSMENT for B16 and H-31

- **Unrealistic hindsight assessment**, built on SOURCE ERRORs (wrong DD, wrong holding, wrong
  partner strength). The board is a **correct partscore result**, not an anomaly.
- H-31's seed board fails its own "game is cold per DD" condition. H-31 therefore has **0
  qualifying exposures** and an overdue review.

**Mechanisms this board points at:**

- **Trump quality and honour fit.** ♠Q5 opposite ♠KTxxxx leaves A, J, 9, 8 outstanding.
- **Dummy entries.** A 2-HCP dummy cannot lead toward ♣K.
- **Tops versus length sources.** ♥AKQ is three tricks, no more.
- **Responder's working values.** Opener-only evaluation cannot see them.

---

## 4. H-34 — Slam-Zone Gate Protocol, and the T16 board accounts

`gov/hypotheses.yaml:2166-2279`

- **Status:** `active`, priority high, filed 2026-09-08 (committed 2026-10-21), `review_by:
  2027-01-31` (`:2171`).
- **Exact claim (`:2193-2204`):** three gates — (1) **LTC trigger**: with a confirmed 8+ fit and
  combined LTC ≤ 12, the partnership must investigate; (2) **keycard gate**: never bid six missing
  2+ of 5 keycards, or missing 2 of the top 3 trumps unless RKCB with a queen ask confirms;
  (3) **2♦ trigger**: "any hand with 4 or fewer losers opens 2♦ **regardless of HCP**". The
  predicted effect is that slam-zone MP yield rises from 40% to ≥ 65% and doubled-slam minuses
  disappear.

**Arithmetic check of the H-34 summary:**

- 8/20 MP = 40% (5 boards × 4-MP top). Consistent.
- ≥ 65% of 24 = 15.6. Consistent.
- Baseline counts (`:2188-2190`): bid-and-made 4, missed 8, failed 2. The counts match the lists.
  "~40-45% yield" (`:2241`) cannot be derived from counts without MPs. GAP.

### 4.1 T16 B9 (H-8 `:299-300`)

- **TABLE EVIDENCE** (governance only): HCP 22 and LTC 7 + 8 = 15 "both said partscore". The
  partnership bid 4♥ vulnerable and went −1, for 0 MP.
- **COMPUTATIONAL RESULT:** none. No hands are available.
- **Arithmetic:** 7 + 8 = 15 → 24 − 15 = **9 tricks**. The table result (−1 in 4♥) is exactly
  9 tricks, so LTC predicted the trick count.
- **GAP:** auction, competition, hands and DD are unknown. Was 4♥ competitive, for example over
  enemy 4-level action? H-8 says "judgement overrode both measures".
- **ASSESSMENT (provisional): failure to follow an agreement.** R-BID-015 and the raise_note both
  pointed to partscore, and the bid overrode them. It cannot be ruled out that the true cause was a
  competitive or sacrifice decision. The board does not point at an evaluation mechanism. It points
  at **discipline** (Law of Priority: Discipline over Brilliance).

### 4.2 T16 B13 (H-8 `:300-303`; H-34 `:2175-2179`)

North ♠AQT ♥2 ♦AKT7642 ♣A5.

**COMPUTATIONAL RESULT** (`check-deal --no-dd`, with synthetic E/S/W filler; only N's line is
meaningful): **17 HCP, LTC 4, 3-1-7-2.** By hand: ♠AQT = 1 loser, ♥2 = 1, ♦AKT = 1, ♣A5 = 1.
Total 4. **Verified.**

- **South "8 HCP / LTC 8": GAP.** The hand is not given.
- **25 HCP:** correct if S has 8.
- **Combined LTC 12 → 12 tricks:** correct arithmetic.
- **"6♠, 6♦ and 6NT all cold (DD 12)":** GAP, cannot be verified without the full deal.
- **TABLE EVIDENCE:** opened 1♦; final contract 4♠+2 = 12 tricks; 0 MP.

**CURRENT AGREEMENT:** N's hand qualifies for 2♦ under `all-fives.yaml:541` (4- losers) and
`:163` (3-4 losers). It was opened 1♦.

**Hindsight check:**

- "Combined LTC 12 was never computed" (`:2179`) assumes the players could see both loser counts.
  At the table, N knows only their own 4 losers. Responder's 8 has to be inferred from the auction.
- No current agreement over a 1♦ opening transmits responder's loser count. The 2NT ask exists
  only over 1M, and the written slam path is only for 2/1 GF and 1M-3M.
- After 1♦-1♠ (presumably), N with 4 losers and 3-card support should still read slam interest
  opposite any 8-9 loser response (24 − 13 = 11+).

**ASSESSMENT: failure to follow an agreement** (2♦ opening; R-BID-015 after the fit), **plus a
missing agreement** (no slam-investigation protocol for 1-minor openings, and three inconsistent
2♦ triggers). The "never computed" framing is partly hindsight. All of this is provisional (no
deal, no auction).

**Mechanisms:** long side-suit source (seven-card ♦AKT), controls (three aces), and HCP
undervaluing a 4-loser hand. Counterweight: B16 (§3) shows the same "LTC 4 ⇒ 2♦" rule
over-reaching opposite an 11-loser hand.

### 4.3 T16 B14 (H-8 `:303-307`; H-29 `:1709-1714`; H-34 `:2180-2184, :2213-2215`)

North ♠96532 ♥AKJ4 ♦Q62 ♣6. South ♠QJ87 ♥2 ♦K ♣AK85432.

**COMPUTATIONAL RESULT** (`--no-dd`, synthetic E/W filler):

| Hand | HCP | LTC | Shape | Loser breakdown (by hand) |
|---|---|---|---|---|
| N | **10** | **7** | 5-4-3-1 | 3 + 1 + 2 + 1 |
| S | **13** | **5** | 4-1-1-7 | 2 + 1 + 1 + 1 |

Both are **verified**.

- **23 HCP:** correct.
- **9-card fit:** correct.
- **Opponents' trumps:** ♠AKT4 (the source says "no A/K/T").
- **Keycards held by NS:** ♥A and ♣A, so 2 of 5. ♠A, ♠K and ♦A are with E-W. Correct.
- **Under 30-41-22:** a 4NT ask from either side gets a 1-keycard reply (5♦), which reveals three
  keycards missing. Correct.

**Definition sensitivity (GAP):** under the spec's own 2NT-ask convention (Qxx = 3 losers), N is
**8 losers** and the combined count is **13 → 11 tricks**. H-34 gate (1), "LTC ≤ 12", **would not
fire**. The gate's outcome on its own key example depends on an undefined counting rule.

**SOURCE ERROR (H-34 `:2213-2215`):** "two missing top trumps with a 9-card fit = ~2 trump losers
unless doubleton AK is onside (B14: it was not)". When the A and K of trumps are both missing, they
are **always two losers**. Their position cannot change that, and "doubleton AK onside" never saves
a trick. The real variable is the ♠T:

- 2-2 split: exactly 2 trump losers.
- ♠T in a 3- or 4-card holding: 3 trump losers.

**DD 9 tricks: GAP** (no full deal). It is consistent with 3 trump losers plus ♦A.

**TABLE EVIDENCE:** 6♠X−2 (10 tricks), 0 MP.

**HCP was also wrong.** The game it pointed to fails DD (H-29 `:1711`, "4♠ fails").

**Why LTC overcounted by 3:**

1. **Trump quality.** N's 3 trump losers plus S's 2 trump losers are netted as if partner covers
   them, but neither hand holds A or K.
2. **Double misfit and shortness.** N ♣6 is opposite S's 7 clubs, and S ♥2 is opposite N ♥AKJ4.
   Each hand's shortness is counted as covering the other's losers, but a crossruff with a broken
   9-card trump suit cannot both draw trumps and ruff.
3. **Duplication.** S's singleton ♦K is opposite ♦Q62: K and Q are in the same suit and ♦A is
   offside. The singleton K is scored as 1 loser, the same as a small singleton, so its HCP are
   wasted.
4. **Controls.** Three keycards are missing, and that was visible before the bid through RKCB.

**ASSESSMENT:**

- **Evaluation error (primary).** LTC was applied without trump-quality, misfit or duplication
  adjustments. HCP over-estimated too.
- **Failure to follow an agreement (secondary, provisional).** RKCB was available (`all-fives.yaml:558`)
  and would have shown 3 missing keycards. The written cue-bid path is scoped to 2/1 GF, and the
  auction is unknown.
- **Missing agreement.** No trump-quality qualifier exists anywhere in the spec.
- **Not unlucky.** The over-reach is structural.

### 4.4 T16 B6 (H-34 `:2185-2187`; context only)

**TABLE EVIDENCE:** 28 HCP, 9-card fit, one keycard missing, 6♠X−1 on a 4-0 trump break. No
hands are given (GAP).

**SOURCE ERROR (logic):** "the same auction with a keycard gate would have stopped in 5♠/4♠ once
♣K was also known missing". Gate (2) as written triggers only with **two or more** of the five
keycards missing, and ♣K is not a keycard. The gate as filed would **not** have stopped B6, unless
a king ask is added, which is not part of H-34.

**ASSESSMENT: unlucky outcome**, as the source itself says ("defensible").

### 4.5 Other H-34 claims checked in passing

- **"missed … T12 B17"** (`:2189`). **SOURCE ERROR.** In T12 B17 we sat **EW defending** NS's
  6♣ (`2026-06-02.md:739` "Round 6 — EW vs pair 1"; `:820` "EW defending 6♣ = → 0.0 MP"). It was
  the opponents' slam, not ours to bid. The baseline should read missed 7, not 8.
- **Gate (3), "any hand with 4 or fewer losers opens 2♦ regardless of HCP"**, conflicts with:
  - Benji Type 3 (~4 losers, 5-5).
  - UCM (≤3 losers, 5-5+; DEC-12 / R-BID-042). A ratified decision at rank 4 outranks a hypothesis
    at rank 5.
  - The table evidence of T12 B16 (§3), where an LTC-4 opener forced to game goes down.
- **Three different trump-quality qualifiers are in circulation:**
  - H-29 `:1712`: "AKQ or AK+10/J".
  - H-29 `:1713`: "two of the top three trumps".
  - H-34 gate (2) and R-TOOL-014: "missing two of A-K-Q".

  These are not equivalent (for example, AQ qualifies under "two of three" but not under
  "AK+10/J"). GAP: no single definition.
- **The H-34 concern (`:2223-2226`)** — "the gates are already in the spec … the fix is drill" —
  holds only for the 2♦ clause and RKCB. The **trump-quality gate and the LTC-≤12 investigation
  trigger are not in the spec**. Only R-BID-015 and the generic raise_note exist. These are missing
  agreements, not execution failures.

---

## 5. Classification summary

| Item | Classification | Source errors | Mechanism it points at |
|---|---|---|---|
| T12 B16 / H-31 | **Unrealistic hindsight assessment** (the board is a correct partscore; game fails DD) | DD 10 → **9**; ♠KQT764 → **♠KT7642**; partner "9 HCP" → **2 HCP**; pair numbers; "Acol 2-bids reserved for two-suiters" | Trump honour fit and holes; dummy entries; tops vs length source (♥AKQ = 3 tricks only). Counter-example to "LTC 4 ⇒ 2♦ GF" |
| T16 B9 | **Failure to follow an agreement** (provisional; may be a competitive decision) | none checkable | Discipline / judgement override. LTC 15 predicted the 9 tricks taken |
| T16 B13 | **Failure to follow an agreement** (2♦ "4- losers"; R-BID-015) **+ missing agreement** (no slam protocol over a 1m opening; inconsistent 2♦ trigger); partly hindsight | none in N's data; S and DD unverifiable | Long side-suit source (♦AKT7642), controls (3 aces), HCP undervaluing a 4-loser hand |
| T16 B14 | **Evaluation error** (primary) **+ missing agreement** (no trump-quality rule) **+ failure to follow** (RKCB not used; provisional) | H-8 "HCP right / 1-1" tally; H-34 "doubleton AK onside" | Trump quality (no A/K/T); misfit and double shortness; duplicated ♦K-Q; controls (3 keycards missing) |
| T16 B6 | **Unlucky outcome** | H-34 claim that the keycard gate would have stopped it | Trump break (4-0); not an evaluation problem |
| H-8 claim | Unresolved. The tally is mis-scored; the "flat 14 HCP → partscore" leg conflicts with R-BID-008 | `:305` 1-1 | Needs trump-quality and misfit adjustments before any slam threshold |
| H-31 claim | No valid exposure; review overdue; the existing method (2♦ via LTC, 3M rebid) was overlooked | five, listed above | Defining "source of tricks" |
| H-34 claim | Partly an execution problem (2♦, RKCB), partly missing agreements (trump gate, LTC trigger); gate (3) conflicts with DEC-12 and B16 | B17 seat; doubleton-AK; B6 gate logic | Controls and trump quality as gates on LTC |
| Spec | — | verbose `:435` "10 HCP" → **9 HCP** | LTC definition (Qxx = 2 or 3) undefined |

---

## 6. Full deals extracted (development examples only)

Only **one** full deal exists for these boards in the live sources. T16 B6, B9, B13 and B14 have
no full deals (GAP: the T16 post-mortem was never committed).

**T12 Board 16** — `contract-bridge/analysis/2026-06-02.md:741-756`. Verified: 13 cards per hand,
52 unique, 40 HCP, `check-deal` "Deal OK".

```
[Event "Winterthur T12"]
[Date "2026.06.02"]
[Board "16"]
[Dealer "W"]
[Vulnerable "EW"]
[Deal "W:Q5.632.T8763.874 A93.JT85.Q4.QJ96 KT7642.AKQ.A.KT3 J8.974.KJ952.A52"]
```

`check-deal` argument order (N E S W):
`"A93.JT85.Q4.QJ96" "KT7642.AKQ.A.KT3" "J8.974.KJ952.A52" "Q5.632.T8763.874"`

DD: E-W ♠ 9, ♣ 7, ♦ 6, ♥ 6, NT 7. N-S ♥ 7, ♣ 6, ♦ 6, ♠ 4, NT 4.

**Partial T16 hands** (governance text only; unknown hands are marked `?`):

| Board | N | S | E / W |
|---|---|---|---|
| T16 B13 | `AQT.2.AKT7642.A5` | ? (8 HCP, LTC 8 claimed) | ? |
| T16 B14 | `96532.AKJ4.Q62.6` | `QJ87.2.K.AK85432` | ? (between them: ♠AKT4, ♥QT987653, ♦AJT987543, ♣QJT97) |

For B14, E-W hold 26 cards: 4 spades, 8 hearts, 9 diamonds and 5 clubs. The fillers used for the
`--no-dd` checks in §4 are **synthetic**. They are not the T16 deals and must not be used for DD.
