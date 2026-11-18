# Beyond Points — packet 1 (opening memo)

Brief: `../../hand-evaluation-discovery.yaml`. PI: Claude. Codex: not connected this session.
Status of every claim: see the labels in each file. Nothing here changes an agreement.

## Discovery thesis

A bridge hand does not have one value. Its worth is (1) **strain-relative** — the same
thirteen cards rank differently for notrump and for trump play; (2) **partner-relative** —
most of the decision value lives in how the two hands interlock, which no single-hand
count can see; and therefore (3) **auction-relative** — the practical gains come less
from a cleverer count than from using a good count on *both* sides of an exchange.
Within one strain, well-chosen published counts are already close to what any
own-hand method can achieve.

## Five ways to perceive a hand (evidence status, packet 1)

| # | Perception | Evidence | Status |
|---|---|---|---|
| 1 | **Strain lenses** — value a hand separately for NT and for a trump fit; a scalar tuned for one strain is no better than HCP in the other | Same opener hands: NT-fitted vs trump-fitted values r = 0.69; transfer loses 0.44 IMP/deal (held-out, 40k) | COMPUTATIONAL RESULT, confirmed (S7) |
| 2 | **Aces are tempo; loser counts that ignore them fail** — classic LTC scores KQx better than Axx | Classic LTC ≈ HCP for the 1M–2M game decision; Zar +0.39 IMP/deal over LTC (held-out, 40k) | confirmed (P2 inconclusive vs HCP, P3 confirmed) |
| 3 | **Holdings, not cards** — a suit's value depends on length and companions (singleton honours half-wasted; tens/nines make 11-counts into games) | Holding table beats Zar by +0.07 (held-out); matched pairs F2, F4 | confirmed in EC-2 (S1); rediscovered (Andrews' Binky points) |
| 4 | **Invitations are worth what both evaluations are worth** | Zar invite +0.10 over Zar direct; SHV invite +0.23 over Zar direct; HCP-evaluated invites gain ≈ 0 | confirmed (P4); new operational measure |
| 5 | **The headroom is in the partnership, not the hand** | S2 Monte-Carlo ceiling ≈ Zar for the direct decision; S3 partnership table +0.96 vs HCP | DIAG / exploratory |

## Files

| File | Content |
|---|---|
| `00-ledger.md` | Shared ledger, reproducibility header, datasets |
| `01-prior-art.md` | Literature audit, exact definitions, comparison matrix (subagent; sources dated) |
| `02-anomaly-audit.md` | H-8 / H-31 / H-34, T16, T12 B16 recomputed (subagent; verified B16 DD) |
| `03-hypothesis-cards.md` | 14 hypothesis cards (HE-01..HE-14) incl. 4 outside the seed invitations |
| `04-experiment-contracts.md` | EC-1..EC-3 contracts and 7 Codex challenge packets |
| `05-confirmation-plan.md` | Frozen endpoints, hashes, addendum |
| `06-results.md` | Dev and confirmation results, failure atlas |
| `07-teaching-positions.md` | 24 positions, 6 matched pairs, Monte-Carlo solutions |
| `08-human-procedure.md` | Draft memory card, drills, usability plan (not run) |
| `09-director-packet.md` | Scoped superiority statement, adoption proposal, cascade, pilot |
| `10-next.md` | Packet 2 plan and Codex handoff |
| `11-zar-method.md` → `zar-method.pdf` | How we play Zar after 1M–2M (DEC-15): counting, thresholds, examples, weak spots, drill, memory card |
| `hand-evaluation-report.md` → `.pdf` | Full packet-1 report (19 pages) |
| `13-nltc-method.md` → `nltc-method.pdf` | How to count and use NLTC (cross-check to Zar; slam candidate; never for NT) |
| `17-tournament-report.md` → `tournament.pdf` | Tournament report with analytics: level mix, accuracy by deal type, errors per 1 000 boards, losses by vul/strain/HCP band, featured match |
| `28-h45-interlock.md` | H-45 Fermat Interlock refuted (−0.11); two-hand ceiling +0.27 over Fermat Complete — the room is in partnership shortness |
| `27-replay-20261116.md` | Replay of Winterthur 2026-11-16 as pair 1 with Fermat Complete + Fifths: 71.25% → 69.17%, still 1st |
| `26-h42-systems.md` | H-42 systems check: Fermat Complete wins all 12 auction-model round robins |
| `25-fermat-complete.md` | Fermat Complete: the extracted system — wins 8/8 on fresh deals, +0.17 IMP/board over All Fives today |
| `24-slam-weakest-link.md` | H-38 slam trigger + keycard gate; H-41 weakest-link (series reliability) evaluation — both corroborated in DD |
| `23-fermat2.md` | H-40 Fermat-2 challenger — refuted on fresh deals; Fermat fit vs Zar confirmed |
| `22-lenses-fermat.md` | 55-team SAYC round robin including Fermat lenses (Fermat fit lens sweeps the podium) |
| `21-fermat.md` | H-39 Fermat Points (one base count + fit dial) vs Zar/Fifths across 12 tournaments |
| `20-competitive.md` | Competitive systems study: opponents interfere, compete (LoTT) and sacrifice |
| `19-systems.md` | Systems study: SAYC, Precision, Acol, 2/1 GF, All Fives and perfect exchange — 41-team round robins + system shoot-out |
| `18-lenses-tournament.md` | 41-team two-lens round robin (fit lens × NT lens, SAYC framework, 5 000-board matches) |
| `16-tournament.md` | Team round robin of 16 evaluation teams on random deals (All Fives / Two-lens win) |
| `15-battle.md` → `battle.pdf` | 14 methods on 100 000 random deals vs DD-optimal level; P8 Zar > NLTC confirmed |
| `14-ohe-codification.md` | Darricades OHE as codified from Timm's overview (secondary source) |
| `12-scorecard.md` → `scorecard.pdf` | All 10 methods × 5 decisions (EC-1, EC-2, EC-4, EC-5, EC-6); P5 refuted, P6/P7 confirmed |
| `data/`, `confirm/`, `teaching/` | Seeded datasets (JSONL), paired per-deal CSVs, MC outputs |

Tooling: `src/bridge-tools/src/sim/hand_eval.rs`, `src/sim/eval_lab.rs`,
`src/bin/eval_lab.rs` (R-TOOL-015, TDD).
