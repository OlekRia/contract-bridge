# Team round robin — evaluation methods as teams (random deals, DD scoring)

Requested by the Director. Plain uniformly random deals: `data/battle-100k-s1000.jsonl`
(100 000 shuffled deals, seed 1000, every deal used, no constraints). Thresholds tuned on
deals 1-30 000; the tournament is played on the untouched deals 30 001-100 000.

Model: every board is played at both tables with the same cards; the partnership with more
HCP declares (NS on 20-20); each team picks partscore / game / small slam / grand from its
pooled partnership count (perfect exchange of counts), strain by the shared rule (8-card
major fit, else NT); score = double-dummy tricks; standard 16-board vulnerability; IMPs per
board; WBF 20-VP scale per 32-board match; 16 teams, full round robin (120 matches) =
3 840 boards; 18 round robins on distinct boards. "All Fives" = Zar with a major fit
(DEC-15) and HCP in NT; "Two-lens" = Zar with a fit, Fifths in NT; "SAYC" = HCP with the
classic 25 / 33 / 37 lines. Only *evaluation* differs between teams — not the bidding system.
Command: `eval_lab tournament data/battle-100k-s1000.jsonl 0.3 32`.

```
Team round robin on random deals: 16 teams, 32-board matches, 18 complete round robins (70000 test boards available, 3840 per round robin)

Event 1 — final standings (15 matches each, 480 boards per team):
  #  team                               IMPs      VPs
  1  BUM-RAP                            +312   194.87
  2  All Fives (Zar fit, HCP NT)        +267   193.36
  3  Two-lens (Zar fit, Fifths NT)      +222   186.48
  4  HCP+len                            +191   181.66
  5  Zar                                +113   170.21
  6  Kaplan-Rubens                      +120   165.62
  7  HCP                                +157   163.68
  8  OHE start (HL+D)                    +52   157.63
  9  HCP+len+short                       -46   144.38
 10  OHE full                            -11   144.25
 11  Fifths                              -31   143.16
 12  NLTC                                -60   139.80
 13  Zar @52/62/67                       -66   137.94
 14  Classic LTC                        -223   119.16
 15  SAYC (HCP 25/33/37)                -203   113.90
 16  OHE full @pub                      -794    43.88

All 18 events combined (each team: 270 matches):
  #  team                            VP/match  IMPs/board     W-D-L   titles   podiums
  1  All Fives (Zar fit, HCP NT)        12.54      +0.525 187-10-73       7        16
  2  Two-lens (Zar fit, Fifths NT)      12.45      +0.485 196-9-65       6        13
  3  HCP+len                            11.56      +0.369 172-2-96       0         3
  4  BUM-RAP                            11.53      +0.317 172-6-92       3         8
  5  Fifths                             11.49      +0.315 170-4-96       2         3
  6  HCP                                11.18      +0.289 153-5-112      0         3
  7  Zar                                11.18      +0.245 162-10-98       0         2
  8  Kaplan-Rubens                      11.07      +0.239 158-6-106      0         5
  9  OHE start (HL+D)                   10.79      +0.167 154-3-113      0         1
 10  HCP+len+short                       9.80      +0.005 113-3-154      0         0
 11  NLTC                                9.72      +0.003 117-2-151      0         0
 12  OHE full                            9.48      -0.070 110-3-157      0         0
 13  Zar @52/62/67                       8.70      -0.235  93-0-177      0         0
 14  SAYC (HCP 25/33/37)                 8.30      -0.295  87-3-180      0         0
 15  Classic LTC                         7.68      -0.429  73-2-195      0         0
 16  OHE full @pub                       2.52      -1.930   9-0-261      0         0
```

## Format 2 — 10 000-board matches (Director request)

Every pairing plays one 10 000-board match; the 70 000 untouched random test deals form 7
blocks of 10 000 that rotate through the 120 matches (both teams in a match play the same
boards at both tables). Boards numbered 1-10 000: dealer rotates N/E/S/W, vulnerability
follows the 16-board cycle. Dealer has no effect in this model (no auction; opponents
silent); vulnerability does (each team uses its vulnerable / non-vulnerable lines).
Command: `eval_lab tournament data/battle-100k-s1000.jsonl 0.3 10000`.

```
Team round robin on random deals: 16 teams, 10000-board matches, 1 round robin(s), matches rotate through 7 distinct blocks of 10000 random deals (70000 test deals)
Dealer rotates N/E/S/W by board number; vulnerability follows the standard 16-board cycle.

Event 1 — final standings (15 matches each, 150000 boards per team):
  #  team                               IMPs      VPs
  1  Two-lens (Zar fit, Fifths NT)    +78090   290.63
  2  All Fives (Zar fit, HCP NT)      +73519   284.31
  3  BUM-RAP                          +57815   257.73
  4  Fifths                           +47727   230.31
  5  HCP                              +39938   208.98
  6  HCP+len                          +38503   206.60
  7  Kaplan-Rubens                    +32075   179.45
  8  Zar                              +27560   172.37
  9  OHE start (HL+D)                 +21750   146.96
 10  OHE full                          +2381   105.99
 11  HCP+len+short                      -516   102.37
 12  NLTC                               -134    94.30
 13  SAYC (HCP 25/33/37)              -32484    59.41
 14  Zar @52/62/67                    -43328    38.55
 15  Classic LTC                      -63209    22.04
 16  OHE full @pub                   -279687     0.00

Event 1 — match results, IMPs per board (row team vs column team):
                                  #1    #2    #3    #4    #5    #6    #7    #8    #9   #10   #11   #12   #13   #14   #15   #16
#1 Two-lens (Zar fit, Fifths NT)     — +0.02 +0.10 +0.17 +0.24 +0.28 +0.29 +0.35 +0.34 +0.52 +0.51 +0.53 +0.72 +0.68 +0.97 +2.07
#2 All Fives (Zar fit, HCP NT) -0.02     — +0.10 +0.12 +0.22 +0.22 +0.28 +0.31 +0.32 +0.47 +0.49 +0.51 +0.76 +0.71 +0.82 +2.07
#3 BUM-RAP                     -0.10 -0.10     — +0.07 +0.10 +0.12 +0.15 +0.22 +0.23 +0.34 +0.34 +0.38 +0.62 +0.64 +0.76 +2.02
#4 Fifths                      -0.17 -0.12 -0.07     — +0.03 +0.06 +0.12 +0.10 +0.16 +0.30 +0.35 +0.24 +0.55 +0.59 +0.66 +1.98
#5 HCP                         -0.24 -0.22 -0.10 -0.03     — -0.01 +0.06 +0.07 +0.13 +0.22 +0.28 +0.28 +0.51 +0.49 +0.59 +1.96
#6 HCP+len                     -0.28 -0.22 -0.12 -0.06 +0.01     — +0.07 +0.05 +0.10 +0.23 +0.30 +0.25 +0.48 +0.47 +0.59 +1.97
#7 Kaplan-Rubens               -0.29 -0.28 -0.15 -0.12 -0.06 -0.07     — +0.03 +0.08 +0.17 +0.18 +0.24 +0.36 +0.43 +0.67 +2.00
#8 Zar                         -0.35 -0.31 -0.22 -0.10 -0.07 -0.05 -0.03     — +0.06 +0.17 +0.20 +0.16 +0.37 +0.41 +0.57 +1.93
#9 OHE start (HL+D)            -0.34 -0.32 -0.23 -0.16 -0.13 -0.10 -0.08 -0.06     — +0.11 +0.11 +0.15 +0.39 +0.43 +0.54 +1.86
#10 OHE full                   -0.52 -0.47 -0.34 -0.30 -0.22 -0.23 -0.17 -0.17 -0.11     — +0.01 +0.04 +0.24 +0.29 +0.41 +1.78
#11 HCP+len+short              -0.51 -0.49 -0.34 -0.35 -0.28 -0.30 -0.18 -0.20 -0.11 -0.01     — +0.02 +0.22 +0.30 +0.50 +1.69
#12 NLTC                       -0.53 -0.51 -0.38 -0.24 -0.28 -0.25 -0.24 -0.16 -0.15 -0.04 -0.02     — +0.27 +0.30 +0.44 +1.76
#13 SAYC (HCP 25/33/37)        -0.72 -0.76 -0.62 -0.55 -0.51 -0.48 -0.36 -0.37 -0.39 -0.24 -0.22 -0.27     — +0.13 +0.19 +1.92
#14 Zar @52/62/67              -0.68 -0.71 -0.64 -0.59 -0.49 -0.47 -0.43 -0.41 -0.43 -0.29 -0.30 -0.30 -0.13     — +0.10 +1.45
#15 Classic LTC                -0.97 -0.82 -0.76 -0.66 -0.59 -0.59 -0.67 -0.57 -0.54 -0.41 -0.50 -0.44 -0.19 -0.10     — +1.50
#16 OHE full @pub              -2.07 -2.07 -2.02 -1.98 -1.96 -1.97 -2.00 -1.93 -1.86 -1.78 -1.69 -1.76 -1.92 -1.45 -1.50     —

All 1 events combined (each team: 15 matches):
  #  team                            VP/match  IMPs/board     W-D-L   titles   podiums
  1  Two-lens (Zar fit, Fifths NT)      19.38      +0.521  15-0-0        1         1
  2  All Fives (Zar fit, HCP NT)        18.95      +0.490  14-0-1        0         1
  3  BUM-RAP                            17.18      +0.385  13-0-2        0         1
  4  Fifths                             15.35      +0.318  12-0-3        0         0
  5  HCP                                13.93      +0.266  10-0-5        0         0
  6  HCP+len                            13.77      +0.257  11-0-4        0         0
  7  Kaplan-Rubens                      11.96      +0.214   9-0-6        0         0
  8  Zar                                11.49      +0.184   8-0-7        0         0
  9  OHE start (HL+D)                    9.80      +0.145   7-0-8        0         0
 10  OHE full                            7.07      +0.016   6-0-9        0         0
 11  HCP+len+short                       6.82      -0.003   5-0-10       0         0
 12  NLTC                                6.29      -0.001   4-0-11       0         0
 13  SAYC (HCP 25/33/37)                 3.96      -0.217   3-0-12       0         0
 14  Zar @52/62/67                       2.57      -0.289   2-0-13       0         0
 15  Classic LTC                         1.47      -0.421   1-0-14       0         0
 16  OHE full @pub                       0.00      -1.865   0-0-15       0         0
```
