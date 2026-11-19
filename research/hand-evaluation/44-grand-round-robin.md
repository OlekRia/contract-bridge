# Grand round robin — every laboratory team, 30 000 brand-new deals

28 teams of four; each team's four players use one evaluation method.

| | |
|---|---|
| Runner | `eval_lab grand <data> 0.5 5000 <out> [competitive] [sayc]` (R-TOOL-021) |
| Deals | brand-new seed 13000 (sha256 [exited …) |
| Lines | tuned on the 30 000 seed-1000 deals |
| Format | round robin, 378 matches of 5 000 boards, WBF 20-VP scale |
| Systems | All Fives or SAYC auction model; opponents silent or competing |
| Outputs | `confirm/grand-s13000-*.md` |

## Winners

| Event | Winner | Runner-up | 3rd |
|---|---|---|---|
| **All Fives, competing** | Fermat Fit-Split (H-53), 17.51 VP | Fermat + partnership shortness (H-46), 17.44 | **H-54 card**, 17.37 |
| All Fives, silent | **H-54 card**, 18.20 (26/27) | Fit-Split, 17.89 | Fit-Quality L2, 17.59 |
| SAYC, competing | Fermat + partnership shortness (H-46), 17.55 (27/27) | Fit-Quality L2, 16.86 | Fit-Split, 16.67 |
| SAYC, silent | **H-54 card**, 18.45 (26/27) | Fit-Split, 18.08 | Fit-Quality L2, 18.06 |

With opponents competing, the top five are within 0.3 VP of each other:

| Team | VP, All Fives competing |
|---|---|
| Fermat Fit-Split (H-53) | 17.51 |
| Fermat + partnership shortness (H-46) | 17.44 |
| H-54 card | 17.37 |
| Fermat Auction Neural (H-52) | 17.26 |
| Fermat Complete | 17.25 |

Every Fermat-family team finishes in places 1-8 in all four events.
Then come the neural networks and Zar Max (8-10), then the published methods.
SAYC classic (HCP 25/33/37) finishes last, 28th, in every event, with 0 VP.

## The H-54 card (Fermat Complete + trump quality) vs the field

IMPs per board, 95% CI.

| Rival | All Fives competing | All Fives silent | SAYC competing | SAYC silent |
|---|---|---|---|---|
| Fermat Complete | +0.015 [−0.008, +0.038] | **+0.035** [+0.010, +0.059] | +0.002 | **+0.055** [+0.029, +0.081] |
| Fit-Quality L2 | +0.015 | −0.008 | −0.005 | −0.007 |
| Fermat Auction Neural (H-52) | +0.010 | +0.054 | −0.001 | +0.069 |
| Fermat Neural (H-51) | +0.080 | +0.084 | +0.068 | +0.116 |
| Zar Max (H-49) | +0.125 | +0.159 | +0.076 | +0.179 |
| Zar / HCP (All Fives today) | +0.205 | +0.211 | +0.145 | +0.242 |
| SAYC classic | +0.843 | +1.002 | +0.740 | +1.016 |

## H-54 replication

The trump term vs Fermat Complete, All Fives, competing, on independent deal sets:

| Deals | Role | Result |
|---|---|---|
| seed 6000 | exploration (H-53) | +0.012 |
| seed 11000 | H-53 secondary | +0.024 [+0.001, +0.048] |
| seed 12000 | H-54 primary | **+0.051** [+0.027, +0.074] |
| seed 13000 | this event | +0.015 [−0.008, +0.038] |

**All four are positive, and the mean is about +0.026 IMP/board.** The H-54 confirmation run (+0.051) was on the high side.
With silent opponents the term is clearly positive every time (+0.035 to +0.064).

Verdict:
- H-54 stands: the pre-stated test was met and no run is negative.
- The realistic size of the gain under competition is about **+0.02 to +0.03 IMP/board**, not +0.05.

## Reading

1. **There is no single best method under competition.** The Fermat-family variants are within about 0.02 IMP/board of each other there. What all of them have is the Fermat Complete core: the Fermat count, the stopper check, slam coverage and keycards.
2. **The H-54 card is the best *simple* choice.** It is 1st in both silent events and 3rd in the main All Fives competing event. It adds a single line to the card, which the H-46 / Fit-Split / L2 variants do not; those need partner's shortness.
3. **The gap to everything outside the Fermat family is large and stable:**

| Rival | H-54 card's edge, IMPs/board |
|---|---|
| neural networks | +0.07 to +0.12 |
| Zar Max | +0.08 to +0.18 |
| All Fives today (Zar / HCP) | +0.15 to +0.24 |
| SAYC classic | +0.74 to +1.02 |

Limits: double-dummy play, range-information auction model, simulated opponents.
