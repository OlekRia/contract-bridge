# Fermat Complete — the extracted evaluation system

Built from the winners of every test in this programme: Fermat Points (H-39), the keycard gate
(H-38) and weakest-link coverage (H-41). Final test: round robin of 5 000-board matches on
**70 000 held-out random deals** — the untouched fresh deals (seed 2000) for confirmation, and
the original test deals (seed 1000). Thresholds tuned only on the 30 000 training deals.
Command: `eval_lab final data/rematch-train30k-plus-fresh70k.jsonl 0.3 5000`.

## The procedure (partnership totals; both hands counted)

```
BASE (each hand)  = HCP + ½ per ten − ½ per K, Q or J in a singleton or doubleton
FIT  (each hand)  = BASE + controls (A=2, K=1) + (2 × longest + second longest − shortest)

NO MAJOR FIT (notrump):
   NT count = BASE + BASE − 3 per suit that neither hand stops (A, Kx, Qxx, Jxxx)
   3NT at 24½+    6NT at 31½+    7NT at 34½+

MAJOR FIT:
   Game:  FIT + FIT ≥ 52½ (vul) / 53½ (non-vul)  → bid game
   Slam:  FIT + FIT − 4 per side suit neither hand controls (A, K, void, singleton) ≥ 62
          → ask keycards (RKCB 30-41-22): two missing → stop; otherwise small slam
   Grand: slam count ≥ 65½ and all five keycards held
```

(The tuned lines reproduce Zar Petkov's published 52 / 62 almost exactly.)

## Fresh deals (confirmation)

| # | Team | Won | VP/match | % optimal vs DD | IMPs lost vs DD/board | Fermat Complete vs team (IMPs/board, 95% CI) |
|---|---|---|---|---|---|---|
| 1 | FERMAT COMPLETE | 8/8 | 18.48 | 73.5% | -2.140 | — |
| 2 | Fermat WL 2-lens | 7/8 | 14.51 | 72.7% | -2.231 | +0.094 [+0.076, +0.111] |
| 3 | Kaplan-Rubens / Fifths | 6/8 | 13.67 | 71.8% | -2.271 | +0.138 [+0.115, +0.160] |
| 4 | Fermat 2-lens | 5/8 | 12.45 | 71.9% | -2.263 | +0.132 [+0.117, +0.147] |
| 5 | Zar / Fifths | 4/8 | 11.90 | 71.4% | -2.278 | +0.144 [+0.124, +0.164] |
| 6 | All Fives today (Zar / HCP) | 3/8 | 10.90 | 71.0% | -2.300 | +0.169 [+0.149, +0.189] |
| 7 | Fifths | 2/8 | 5.31 | 69.6% | -2.436 | +0.322 [+0.295, +0.348] |
| 8 | HCP | 1/8 | 2.77 | 68.6% | -2.493 | +0.386 [+0.359, +0.413] |
| 9 | SAYC classic (HCP 25/33/37) | 0/8 | 0.00 | 67.3% | -2.907 | +0.871 [+0.838, +0.904] |

Tuned lines: Fermat Complete cuts (non-vul): NT [24.75, 31.25, 34.25], game 53.25, slam 61.75, grand 65.25 Fermat Complete cuts (vul): NT [24.25, 31.25, 34.25], game 52.25, slam 61.75, grand 65.25 

## Original test deals (seed 1000)

| # | Team | Won | VP/match | % optimal vs DD | IMPs lost vs DD/board | Fermat Complete vs team (IMPs/board, 95% CI) |
|---|---|---|---|---|---|---|
| 1 | FERMAT COMPLETE | 8/8 | 18.60 | 73.1% | -2.176 | — |
| 2 | Fermat WL 2-lens | 7/8 | 15.12 | 72.2% | -2.281 | +0.106 [+0.088, +0.124] |
| 3 | Fermat 2-lens | 6/8 | 13.31 | 71.3% | -2.309 | +0.141 [+0.125, +0.156] |
| 4 | Kaplan-Rubens / Fifths | 4/8 | 11.84 | 71.4% | -2.314 | +0.147 [+0.124, +0.170] |
| 5 | Zar / Fifths | 5/8 | 11.75 | 70.9% | -2.324 | +0.154 [+0.134, +0.174] |
| 6 | All Fives today (Zar / HCP) | 3/8 | 11.13 | 70.5% | -2.348 | +0.180 [+0.160, +0.200] |
| 7 | Fifths | 2/8 | 5.45 | 69.4% | -2.462 | +0.316 [+0.289, +0.343] |
| 8 | HCP | 1/8 | 2.80 | 68.3% | -2.520 | +0.382 [+0.354, +0.409] |
| 9 | SAYC classic (HCP 25/33/37) | 0/8 | 0.00 | 66.9% | -2.936 | +0.868 [+0.835, +0.901] |

## Limits

Perfect exchange of counts and coverage (the auction is assumed to reveal them), opponents
silent, double-dummy play, strain by a fixed rule (8-card major fit, else notrump), and the
five-level stop modelled as staying in game. Not yet tested inside the system models
(SAYC / Precision / All Fives auctions) or with opponents bidding, and not at the table.
