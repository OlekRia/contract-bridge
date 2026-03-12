# 1NT Response to 1-Suit Openings: Forcing or Artificial?

**Date**: 2026-03-12
**Hypothesis**: H-2 (expanded)
**Status**: COMPLETE — H-2a (Forcing 1NT) promoted to model
**Engine**: bridge-tools simulate nt-response (DDS double-dummy, dds-bridge 0.11)
**Sample**: 99,996 deals (8,333 per opening x vulnerability combo), seed=42

## Simulation Results

| Scheme | Avg Gain vs Quo | Disaster% | Gain:Loss | Verdict |
|--------|-----------------|-----------|-----------|---------|
| **H-2a (Forcing 1NT)** | **+1.118** | **5.2%** | **6.7:1** | **WINNER** |
| H-2b (Transfers) | +0.775 | 3.6% | 6.9:1 | Rejected |
| H-2c (Artificial GF) | +0.687 | 14.6% | 2.2:1 | **Rejected** |

**H-2a wins decisively over both alternatives.**
- vs H-2b: +0.342 tricks/deal. Transfer fatal flaw: 40.9% stuck rate.
- vs H-2c: +0.431 tricks/deal. Art. GF fatal flaw: 40.9% rescue hands
  average -0.188 tricks (worse than doing nothing). 14.6% disaster rate
  is nearly 3x H-2a's 5.2%. Wrong-siding penalty (West declares).

Facts filed: F-SIM-001, F-SIM-002, F-SIM-003, F-SIM-004.

## Background

In All Fives, every 1-level suit opening promises 5+ cards (1st/2nd seat).
The current 1NT response is natural non-forcing: 6-9 HCP, no fit, no 5-card
major. The Director's observation: this bid is nearly dead space.

**Why 1NT non-forcing is rarely useful:**

1. Opener has 5+ in a suit. If responder has a misfit, opponents likely have
   a fit and will bid — 1NT rarely survives as final contract.
2. Even when opponents are silent, opener rebids with a second suit, extra
   values, or 6+ cards. The "natural 1NT" just delays the auction.
3. With 12-14 balanced hands opening 1NT directly (weak NT), the 1-suit
   openings guarantee shape — making the misfit scenario more common.

## Three Competing Schemes

### H-2a: Forcing 1NT (Natural, Wide Range)

After any 1-suit opening (1st/2nd): 1NT = 6-12 HCP, forcing one round.

**Opener's rebids after 1H - 1NT (forcing):**

| Rebid | Meaning |
|-------|---------|
| 2C    | 4+ clubs, any strength (may be 3 with 5332) |
| 2D    | 4+ diamonds, any strength |
| 2H    | 6+ hearts, minimum (12-14) |
| 2S    | Reverse — 16+ HCP, 4+ spades |
| 2NT   | 15-17 balanced (confirms 4-card opening) |
| 3H    | 6+ hearts, invitational (15-17) |

**After 1C - 1NT (forcing):**

| Rebid | Meaning |
|-------|---------|
| 2C    | 6+ clubs, minimum |
| 2D    | 4+ diamonds (may be just convenient rebid) |
| 2H/2S | Reverse — 16+ HCP, 4+ in major |
| 2NT   | 15-17 balanced (confirms 4-card opening) |
| 3C    | 6+ clubs, invitational |

**Responder then:** pass (6-9), bid again (10-12 invitational), or GF.

**Key advantage:** Opener always has a rebid because 5+ cards guarantees a
second suit or a rebiddable long suit. Simple, well-tested (standard in 2/1
systems worldwide).

**Key risk:** Playing 2-of-a-minor in a misfit when 1NT would have scored
better. Frequency of this disaster is the critical simulation question.

### H-2b: Transfer Responses

After 1M opening, responses below 2M become transfers:

| Response | Transfer to | Shows |
|----------|------------|-------|
| 1NT      | Clubs      | 5+ clubs, any strength |
| 2C       | Diamonds   | 5+ diamonds, any strength |
| 2D       | Other major | 5+ in OM, or artificial raise |

After 1m opening: scheme needs separate design (more complex because minor
openings already have 1-level suit responses available).

**Key advantage:** Right-sides contracts (opener declares), more descriptive
sequences, modern expert technology used by top pairs.

**Key risk:** Enormous complexity. Four different suit openings each need a
full transfer scheme. Violates "bids mean what they look like." Memory
failures at the table are catastrophic (undisclosed agreements = director
calls, penalties, trust erosion).

### H-2c: Artificial Game Force

1NT = artificial game force, replacing traditional 2/1 GF bids.

**Effect:** All 2-level responses become available for non-GF meanings:
- 2C/2D = natural, non-forcing (currently these would be 2/1 GF)
- Fit jumps, invitational bids, etc. get dedicated space

**Key advantage:** Restructures the entire response framework. Maximum
information density.

**Key risk:** Responder with a genuine weak balanced hand (6-9, no fit) has
no natural bid. Must use a 2-level suit as a "nowhere to go" response. Very
unusual — no established partnership base to learn from.

## Simulation Design

### Methodology

Double-dummy simulation, same engine as CRASH study (dds-bridge 0.11).

**Deal generation:**
- Random deals, 100,000 total
- Filter: one hand holds a 1-suit opening (5+ cards, 12+ HCP, not 12-14
  balanced which opens 1NT)
- Partner holds 6-12 HCP (the range where 1NT response applies)
- No opponent interference (all schemes are OFF in competition)

**For each qualifying deal, solve:**
1. **Status quo (non-forcing 1NT):** Responder passes 1NT when 6-9 with no
   fit. DD-solve 1NT by responder. Also solve opener's "natural rebid" in
   case opener would bid again.
2. **Forcing 1NT (H-2a):** Opener makes best rebid. DD-solve the resulting
   contract.
3. **Transfers (H-2b):** Responder transfers. DD-solve the resulting contract
   (declared by opener = right-sided).
4. **Best possible contract:** DD-solve all strains both directions. This is
   the theoretical maximum.

### Metrics

| Metric | What it measures |
|--------|------------------|
| **Frequency** | How often does 1NT response occur (% of all deals)? |
| **Final contract accuracy** | How close to the DD-optimal contract? |
| **Matchpoint estimate** | Estimated MP% vs field playing non-forcing 1NT |
| **Misfit disaster rate** | % of deals where scheme produces worse result than 1NT |
| **Right-siding gain** | (H-2b only) How often does declaring from opener's side gain a trick? |

### Stratification

Results stratified by:
- **Opener's suit:** clubs, diamonds, hearts, spades (4 groups)
- **Opener's shape:** single-suited (5-3-3-2, 6-x) vs two-suited (5-4, 5-5, 6-4)
- **Responder's strength:** 6-9 (weak) vs 10-12 (invitational)
- **Vulnerability:** all four combinations

### Success Criteria

| Scheme | Adopt if... | Reject if... |
|--------|-------------|--------------|
| H-2a (forcing) | MP% > 52% vs status quo AND misfit disaster < 15% | MP% < 50% or disaster > 25% |
| H-2b (transfers) | MP% > 55% vs status quo (higher bar due to complexity cost) | MP% < 52% or memory burden unjustified |
| H-2c (artificial GF) | MP% > 55% vs status quo AND weak-hand route works cleanly | MP% < 52% or weak-hand problem unsolved |

The higher bar for H-2b and H-2c reflects the complexity premium: a complex
scheme must win by MORE than a simple one to justify the memorization and
error risk (Art. IV.3: Discipline Over Brilliance).

### Priority Order

If multiple schemes pass their threshold:
1. **H-2a** wins by default (simplest, lowest error risk)
2. **H-2b** wins only if it beats H-2a by 3+ MP% (complexity premium)
3. **H-2c** wins only if it beats H-2a by 3+ MP% AND solves the weak-hand
   routing problem cleanly

## Implementation Notes

### What needs building

The bridge-tools simulation engine already supports:
- Random deal generation with filtering
- DD solving for all strains/seats
- Matchpoint scoring model

**New components needed:**
1. **Bidding classifier:** Given a deal, determine the "natural" auction
   under each scheme (status quo, forcing, transfers, artificial GF)
2. **Contract mapper:** Map the classified auction to a final contract
3. **Right-siding detector:** (H-2b) Compare DD results when declared from
   North vs South

### Interaction with other hypotheses

- **H-5 (Jump Responses):** If H-2b or H-2c is adopted, the jump response
  structure changes completely. Simulate independently first, then check
  interaction.
- **H-8 (LTC Integration):** LTC thresholds for suit raises are orthogonal
  to 1NT response scheme — no interaction expected.
- **H-9 (Prepared 1C):** 3rd seat 1C-1NT is a special case (prepared bid).
  This study covers 1st/2nd seat only. 3rd seat excluded.

## Structural Insight: Cleaning Up 1m - 1M Ambiguity

Currently after 1m opening, a 1M response promises 5 cards if weak (6-9)
but may show only 4 if strong (invitational+). This forces opener to raise
with just 3-card support sometimes — a systemic compromise.

**If 1NT becomes artificial (any of H-2a/b/c), this can be resolved:**
- **1m - 1M = always 5+** (weak or strong, no ambiguity)
- **1m - 1NT (artificial) = includes 4-card major inv+ hands**

The artificial 1NT acts as a relay: responder bids 1NT, opener describes
hand, and responder then shows the 4-card major at their second turn with
invitational+ values. Opener's raise decision becomes clean: support = 3+
cards facing a guaranteed 5-card suit.

**Simulation must measure this separately:** stratify by "responder has
4-card major inv+" to quantify how often this case arises and whether
routing through 1NT improves final contract vs the current ambiguous route.

This insight applies to **all three schemes** — even H-2a (forcing natural)
benefits, because the wide 6-12 range lets invitational 4-card major hands
go through 1NT forcing, then show the major on the second round.

## Open Questions

1. Should the forcing 1NT range be 6-12 or 6-9? Wider range (6-12) is more
   flexible but means opener can't distinguish weak from invitational until
   responder's second bid.
2. After 1m - 1NT forcing, can opener always find a natural rebid? With
   1C opening and 5-3-3-2 shape, the "convenient" rebid of 2D with 3 cards
   feels wrong.
3. For transfers (H-2b): what does responder bid with a balanced 6-9 and no
   5-card suit? There's no "I have nothing" transfer.
4. For all schemes: what happens when opponents double 1NT (artificial)?
   Need escape/redouble agreements.
