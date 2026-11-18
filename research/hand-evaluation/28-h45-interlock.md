# H-45 — Fermat Interlock (refuted) and the two-hand ceiling (headroom confirmed)

Plan: `05-confirmation-plan.md` Addendum E, frozen before any interlock code ran. Code:
`sim::hand_eval::{interlock_fit, interlock_nt, interlock_ceiling_features}` (6 unit tests),
runner `eval_lab interlock <data> 0.3 32`. Train = deals 0-29 999 of seed 1000 in both files.
Outputs: `confirm/h45-interlock-explore.md` (seed 1000 test region),
`confirm/h45-interlock-confirm.md` + `.err` (fresh seed 2000, 70 000 boards). Each run
takes about one second.

## Verdict

| Endpoint | Criterion | Fresh deals (seed 2000) | Result |
|---|---|---|---|
| **S-H45 headroom** | Ceiling vs Fermat Complete ≥ +0.10 | **+0.269 [+0.243, +0.295]** IMP/board (fit boards +0.421, NT +0.124) | **Corroborated** — room exists |
| **P-H45 primary** | Fermat Interlock vs Fermat Complete, CI lower > +0.03 | **−0.113 [−0.132, −0.093]** | **Refuted** (CI upper < 0) |

The exploratory region gave the same picture (ceiling +0.259, Interlock −0.113), so the
result is not a seed accident.

## Ablations (fresh deals, IMPs per board vs Fermat Complete)

| Term | Alone | Removed from the full set |
|---|---|---|
| Wasted honours (−1 K/Q/J opposite a singleton; void scale) | **+0.021 [+0.008, +0.034]** | −0.113 (removing it hurts most) |
| Fitting honours (+1 K/Q/J opposite partner's 4+ suit) | −0.021 | −0.067 |
| Double fit (+2 for a second 8-card fit) | −0.039 | −0.053 |
| NT filler (+½ J/T opposite partner's A/K/Q) | −0.042 | — |

- **Wasted honours is the only term that helps**, in both regions (+0.020, +0.021). It falls
  short of the +0.03 bar, so it is a lead, not a finding.
- **Fitting honours and the double fit hurt.** Fermat's fit count already pays for these
  honours and for length; adding them again double-counts.
- **The NT filler hurts.** Fifths already pays tens and the stopper check already catches the
  holdings that matter.

## Where the ceiling finds its +0.42 on fit boards (exploratory diagnostic)

Ridge weights of the fit-side ceiling model (tricks, relative to the other cells of the
table; the model also sees every combined holding):

| Side suits: shorter hand's length ↓ / combined length → | ≤6 | 7 | 8 | 9+ |
|---|---|---|---|---|
| 0 (void) | **+1.09** | +0.26 | +0.03 | +0.09 |
| 1 (singleton) | **+0.55** | +0.27 | +0.18 | +0.12 |
| 2 | −0.11 | −0.26 | −0.34 | −0.30 |
| 3+ | **−0.43** | −0.52 | −0.48 | −0.14 |

Trump suit: a 9+ card fit is worth about +0.4 trick more than an 8-card fit when the short
trump hand has 2-3 trumps. Raw placement counts: −0.06 trick per wasted HCP, −0.04 per
"fitting" honour.

**Reading.** The signal is not honour placement — it is **partnership shortness**: a void or
singleton in a side suit where the partnership is short gains about ½-1 trick; when the
shorter hand holds 3+ cards in every side suit (mirror shape) the partnership loses about ½
trick per suit. Fermat's shape number is computed one hand at a time and cannot see
whether the two shapes duplicate. This is the obvious next candidate (H-46), but it was
found on the confirmation data, so it **must be tested on new random deals**.

## Limits

Perfect exchange (both hands known), double-dummy tricks, silent opponents, strain by the
fixed rule (8-card major fit, else NT), five-level not modelled. The ceiling is a learned
diagnostic (about 740 weights), not a usable method; it measures how much room exists.
