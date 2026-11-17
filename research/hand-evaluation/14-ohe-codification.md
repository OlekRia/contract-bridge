# OHE (Darricades) — codification used in the battle

**Status: SECONDARY-SOURCE CODIFICATION — not verified against the book.**
Source used: Neil H. Timm, "Hand Evaluation Revisited" (bridgewebs.com/ocala/Hand-Evaluation-Revisited.pdf,
fetched 2026-11-17; text extracted locally with `pdftotext`), section "Optimal Hand Evaluation
(Overview)". Primary source *not* accessed: P. Darricades, *Optimal Hand Evaluation* (Master
Point Press, 2019) and *Optimal Hand Evaluation in Competitive Bidding* (2020). An unauthorised
copy exists on Scribd; it was not used. Timm's other handout gives different Qxx / Jxx values
(per search snippets), so the secondary sources themselves disagree.

Validation: the implementation reproduces both worked examples in Timm's text exactly —
♠AKQJxxx ♥xxx ♦xx ♣x = 18½ (opener, HLD), and North 15 HL + South 17 HL + 4 fit = 36 (NT).
Code: `ohe_hl`, `ohe_d`, `ohe_partnership` in `src/sim/hand_eval.rs` (6 unit tests).

| Rule (Timm text) | Implementation choice | Ambiguity |
|---|---|---|
| A 4½, K 3 | as stated | — |
| Q w/ A, K, J 2; Qxx 1½; Qx 1 | singleton Q = 1, then singleton-honour −1 | singleton Q not stated |
| J w/ A, K, Q 1; Jxx ½; Jx 0 | singleton J = 0, then singleton-honour −1 | singleton J not stated |
| "10K=½, 10A=0, 10Q=1, 10J=1, 10Jx=2" | T with Q or J +1; with K (no Q/J) +½; else 0 | "10Jx=2" unclear; not used |
| No aces −1 (opener), no Q −1, no K −1, max −2 | as stated; "opener" = declarer in the battle | — |
| 3 K +1, 4 K +2, 4 Q +1 | as stated | — |
| Singleton honour −1 | per singleton A/K/Q/J | which honours |
| "Honor doubletons = −1 pt for 2 honor doubletons" | −1 once if ≥ 2 doubletons contain an honour | — |
| 3+ honours in 6-card suit +2, 5-card +1 | honours = A K Q J; 7+ treated as 6 | 7+ not stated |
| L: 5-card with "QJ/K" +1; 6-card +2 (w/o 1); 7-card "2 per card from the 7th" | "QJ/K" read as any of K, Q, J (needed to reproduce 15 HL); 7+ = 6-card value + 2 per card beyond 6 | reproduces 18½ |
| Responder HL only, L ≤ 2 | dummy's L capped at 2 | — |
| D (opener, fit found): void 4, singleton 2, two doubletons 1, 4333 −1; singleton in NT −1 | declarer gets D in a suit contract; −1 per singleton (both hands) in NT | whether 4333 applies without a fit |
| F: 8/9/10-card fit +1/+2/+3, all suits, suit and NT | each partner adds F for each 8+ fit (reproduces "+4" in example 2) | could be counted once |
| Semi-fit: Kx/Qx/J10/Jx in partner's 5+ suit +1 | as stated, both partners | — |
| S (shortness) by trumps 4/3/2: void 4/3/2, singleton 3/2/1, doubleton 2/1/0 | dummy only, suit contracts | — |
| Misfit −3/−2/−1 (void/singleton/doubleton opposite partner's long suit) | partner 5+, combined < 8, not trumps | "long suit" length |
| Wasted honours: K/Q/J opposite S/V −2/−3; non-honours +2/+3; A opposite singleton +1 | per side suit, both directions, suit contracts | overlap with S/D points |
| Lines: 26 NT, 27 major game, 30 minor game, 33 suit slam, 34 NT slam, 36/37 grand | "@pub" contestant: suit 27/33/36, NT 26/34/37 | — |

**Known scale problem.** In the pilot, the tuned game line for this codification is about 41
(suit), against the published 27: the dynamic adjustments as codified (fit points from each
partner, D + S shortness, wasted honours) inflate totals far beyond Darricades' scale. The
"@pub" result therefore says more about this codification than about the book, and the
tuned OHE results measure the codification's *ranking* ability only. A faithful test needs
the book's exact rules.
