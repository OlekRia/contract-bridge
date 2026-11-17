# 01 — Prior Art: Hand Evaluators (literature and source audit)

- Audit date: 2026-11-17
- Scope: exact definitions and empirical evidence for HCP and alternative honour scales, LTC/NLTC,
  Kaplan–Rubens CCCC, Zar, Bergen/Goren adjustments, controls, quick and playing tricks, SQT, LoTT,
  DD/ML studies, and prior art for fit/duplication/simulation evaluators.
- Honesty rules: "Accessed" means the text was retrieved and read (via WebFetch or curl plus
  pdftotext). WebFetch returns a model-written summary, not raw text, so numbers taken from
  WebFetch summaries are marked **[WF]**. Numbers read from raw text, raw source code or a PDF
  are marked **[RAW]**. Numbers I computed myself are marked **[OURS]**. Anything not accessed is
  listed in §6.

---

## 1. Search log

Engine: Claude Code `WebSearch` (US index), plus `WebFetch`, `curl`, `gh api` and `pdftotext`.
All searches were run on 2026-11-17.

| # | Search string / fetch | Tool | What came back | Coverage / notes |
|---|---|---|---|---|
| S1 | `Thomas Andrews hand evaluator double dummy aces kings queens jacks tens value notrump` | WebSearch | Andrews site index, 3NT article, Deal docs, Zhang project | Good. The old `/valuations/*` URLs return 404; the live path is `/bridge/valuations/` |
| S2 | `Kaplan Rubens hand evaluator algorithm Bridge World October 1982 CCCC` | WebSearch | Pavlicek 8J19 and AC01, BBO threads, Leicestershire page | Primary BW article not online |
| S3 | `Zar Points formula Petkov HCP controls two longest suits difference longest shortest 26` | WebSearch | Wikipedia, csbnews (Petkov), Pavlicek AC01, BBO | Good |
| S4 | `New Losing Trick Count NLTC 1.5 1 0.5 missing ace king queen formula 25` | WebSearch | Wikipedia LTC and mirrors | Koelman primary not online |
| S5 | `Jeff Goldsmith hand evaluator Kaplan Rubens 4C calculator` | WebSearch | BBO 67766, Leicestershire, Ted Muller software | Goldsmith CGI located |
| S6 | `dealer program "cccc" "quality" function Danil Suits c4.c Kaplan Rubens source` | WebSearch | Irrelevant (car dealers) | Failed. Found the code another way (F10) |
| S7 | `"four C's" Kaplan Rubens evaluation "Bridge World" 1982 suit quality "multiply" length tenth` | WebSearch | Pavlicek, Leicestershire, BW index | Same set as S2 |
| S8 | `Law of Total Tricks accuracy double dummy study percent deals exact within one trick` | WebSearch | Ginsberg (mirrors), Pavlicek 8J69, Zhang, vector.co.jp | Good |
| S9 | `lajollabridge Matthew Kidd Law of Total Tricks analysis` | WebSearch | Wikipedia LoTT, Ginsberg mirror | **No Kidd LoTT study found** |
| S10 | `neural network predict double dummy tricks bridge hand evaluation paper` | WebSearch | Mossakowski & Mańdziuk (2004/2006/2009), Dali PyData 2018, Winograd-Cort 2008, Dharmalingam 2014 | Good |
| S11 | `Gordon Bower taigabridge double dummy research hand evaluation` | WebSearch | csbnews column only | taigabridge research not reached |
| S12 | `Klinger "Modern Losing Trick Count" adjustments aces queens "AQ" half loser 7 losers opening rules` | WebSearch | Wikipedia, BBO, bridgehands | Secondary only |
| S13 | `Koelman "New Losing Trick Count" Bridge World 2003 missing ace 1.5 losers 25 minus` | WebSearch | Wikipedia and mirrors | Secondary only |
| S14 | `Marty Bergen "Points Schmoints" adjust-3 hand evaluation rules quality suit length points dummy points` | WebSearch | Lesson PDFs, unit524, Wikipedia | Secondary only. Bergen's book not accessed |
| S15 | `"I Fought the Law of Total Tricks" Lawrence Wirgren findings purity fit adjustments percent` | WebSearch | Wikipedia, BBO, vector.co.jp | Paper and book not accessed |
| S16 | `lajollabridge.com Kidd double dummy hand evaluation statistics` | WebSearch | Kidd slam/game statistics pages | No evaluator-accuracy study by Kidd |
| S17 | `BUM-RAP point count origin 4.5 3 1.5 0.75 0.25 who invented` | WebSearch | Irrelevant | **Origin of BUM-RAP not found** |
| S18 | `Suit Quality Test Klinger length plus honours ...` | WebSearch | Irrelevant (tailoring) | Failed. Used Wikipedia (F6) |
| S19 | `Tysen Streib TSP hand evaluator rec.games.bridge Binky points auction` | WebSearch | BBO thread 3278, Google Groups | Good |
| S20 | `Ginsberg "GIB: Imperfect Information..." JAIR 2001 ...` | WebSearch | JAIR record, DOI 10.1613/jair.820 | Abstract only |
| S21 | `deep learning bridge bidding double dummy reward Yeh Lin 2016 Rong 2019 ...` | WebSearch | arXiv 1903.00900, 1607.03290 | Abstracts only |
| S22 | `Lorand Dali BEN bridge engine neural network double dummy estimator github` | WebSearch | github.com/lorserker/ben | README read via gh |
| S23 | `arxiv contract bridge double dummy trick prediction transformer neural network 2023 2024 2025` | WebSearch | arXiv 2310.06624 (BridgeHand2Vec), 2406.10306 | PDF read |

Fetches (all 2026-11-17):

| # | URL | Result |
|---|---|---|
| F1 | rpbridge.net/8j19.htm (Pavlicek, K-R stats) | OK [WF]. Fetched twice to get the step text |
| F2 | bridge.thomasoandrews.com/bridge/valuations/cardvaluesfor3nt.html | OK [WF] |
| F3 | …/bridge/valuations/ (index), additive.html, original.html, binky-evaluated.html, points.html, cowan.html, doubledummy.html, conclusions.html, acesandspaces.html, after-1s.html, experts4333.html, singledummy.html | OK [WF]. The `/valuations/…` forms of these URLs return 404 |
| F4 | rpbridge.net/ac01.htm (six point-count methods), 8j45.htm (actual vs DD), 8j69.htm (LoTT variance), 8j33.htm (fits; not relevant) | OK [WF] |
| F5 | en.wikipedia.org/wiki/Losing-Trick_Count | OK [WF] |
| F6 | en.wikipedia.org/wiki/Hand_evaluation (first 100k chars) | OK [WF] |
| F7 | en.wikipedia.org/wiki/Zar_Points; csbnews.org Zar article (2016-04-09) | OK [WF] |
| F8 | en.wikipedia.org/wiki/Law_of_total_tricks | OK [WF] |
| F9 | sources.debian.org deal 3.1.9-15: `lib/evaluators.tcl`, `lib/features.tcl`, `lib/binky.tcl`, `lib/binky-data.tcl` | OK [RAW] (GPL-2+) |
| F10 | sources.debian.org dealer 20211121+ds-3: `c4.c` (Danil Suits, 1999) | OK [RAW] (GPL) |
| F11 | github jdh8/contract-bridge `src/eval.rs` (Rust crate) | OK [RAW] |
| F12 | jeff-goldsmith.org/cgi-bin/knr.cgi | **Live domain hijacked** (Japanese spam). Wayback 2017-01-09 shows only the front page (Goldsmith, 2001-10-23). The CGI algorithm is not visible |
| F13 | bocosan.tripod.com/ginsberg/total.HTML via Wayback (2023-10-07 capture) | OK [RAW]. Ginsberg's LoTT analysis |
| F14 | hp.vector.co.jp/… (Ginsberg mirror and LoTT corrections) | **ECONNREFUSED** |
| F15 | pages.mini.pw.edu.pl/~mandziukj/PRACE/IEEETNN09.pdf | OK [RAW] (pdftotext) |
| F16 | arxiv.org/pdf/2310.06624 (BridgeHand2Vec) | OK [RAW] |
| F17 | stat.berkeley.edu/~aldous/Research/Ugrad/Z_Zhang_project.pdf | OK [RAW] |
| F18 | bridgebase.com forums 32125 (helene_t regression), 3278 (TSP), 67766 (K&R) | OK [WF] |
| F19 | forumstest.bridgebase.com/…/31264 (dealer K-R script) | **HTTP 500** |
| F20 | bridgebase.com forum 66910 (DD vs actual) | **ECONNREFUSED** |
| F21 | lajollabridge.com/Articles/PartialGameSlamGrand.htm | OK [WF] |
| F22 | bridgewebs.com/leicestershire/page45.html (4C's summary, Kleinman points) | OK [WF]. Secondary, and has internal discrepancies (§3.3) |
| F23 | newbridgelaw.com (Wirgren) | Dead, parked domain. The 2008 Wayback frame points to web.telia.com, which was **not followed** |
| F24 | bridgeworld.com Esoterica index | Index only. No K-R content |
| F25 | github anntzer/redeal README, lorserker/ben README | OK [RAW] |

---

## 2. Claim-to-source table

Evidence quality: **P** = primary (author's own definition or data); **S** = secondary
(encyclopaedia, summary, third-party implementation); **O** = opinion; **E** = empirical (DD or
actual-play data).

| # | Claim | Source | URL | Section / page | Quality | Limitations |
|---|---|---|---|---|---|---|
| C1 | HCP 4-3-2-1. Wikipedia says it was first published by Bryant McCampbell (*Auction Tactics*, 1915, p.26) and later popularised by Work and Goren. Goren 1954: −1 with no aces, +1 with four aces | Wikipedia "Hand evaluation" | en.wikipedia.org/wiki/Hand_evaluation | HCP section | S | Attribution to McCampbell not checked against the book |
| C2 | Fifths = A 4, K 2.8, Q 1.8, J 1, T 0.4 (deck total 40). Lowest 3NT decision error of the evaluators tested | T. Andrews, "Card Values for Three Notrump" (©1997–2005) | bridge.thomasoandrews.com/bridge/valuations/cardvaluesfor3nt.html | Results tables | P+E | DD data, filtered sample (§5.1). Error metric is the author's own |
| C3 | BUM-RAP = A 4.5, K 3, Q 1.5, J 0.75, T 0.25 (sum 10 per suit). Worse than HCP for 3NT, "more effective, at least for suit contracts" | Andrews, 3NT article and "Conclusions" (©1997–2011) | …/cardvaluesfor3nt.html; …/conclusions.html | — | P+E (DD) | **Originator of BUM-RAP not found**. Wikipedia-derived lesson PDFs attribute the same values to Bergen (2002), unverified |
| C4 | One HCP ≈ ½ trick in NT and ≈ ⅓ trick in suits | Andrews, "Binky Points: An Additive Evaluator" and "HCP vs Binky" | …/additive.html; …/points.html | Text | E (DD) | DD bias. Dataset size not stated |
| C5 | NT and suit evaluation are "very different beasts". In NT, CCCC correlates less with tricks than HCP (0.4812 vs 0.5071). In suits, CCCC is among the best (0.5182 vs HCP 0.4669) | Andrews, "Single Hand Evaluator" (©1997–2005) | …/original.html | Correlation lists | E (DD, GIB library 700k+) | Single-hand vs pair tricks. The analysis says it ran on part of the data only |
| C6 | Binky Points: per-holding plus per-pattern tables (NT, suit), additive across partners. Error SD vs DD tricks: suit 0.99, NT 1.26 (100k deals); vs 20-deal average: suit 0.76, NT 0.83 | Andrews, "Testing Binky Points" (©2008) | …/binky-evaluated.html | Tables 1–2 | P+E | Ignores fit. Tables in Deal `lib/binky-data.tcl` [RAW] |
| C7 | Linear correlation with tricks: Binky-suit 0.925; BUM-RAP (with 3/2/1 shortness) 0.914 (suit); Binky-NT 0.947; Fifths 0.931 (NT) | same | same | Correlations | E | Correlation of a pair evaluator with the 20-deal average |
| C8 | "Cards in long suits are worth less than cards in shorter suits"; cards in combination slightly less than in separate suits (DD) | Andrews, "Conclusions" | …/conclusions.html | Text | E/O | Runs against the premise of the K-R length multiplier. Andrews warns against over-reading it |
| C9 | Ace value by length (DD, suit): x→A +1.33, xx→Ax 1.53, xxx→Axx 1.61, xxxx→Axxx 1.61, xxxxx→Axxxxx 1.59, xxxxxx→Axxxxxx 1.51 tricks | Andrews, additive.html | …/additive.html | Ace table | E | DD |
| C10 | A spade fit reduces side-suit honour value (AKQ −0.25 avg, −0.54 with a 5-card fit). Small side cards gain. Trump honours over-counted ("duplication of values") | Andrews, "After partner opens 1♠" (draft) | …/after-1s.html | Tables | E (draft) | Small samples. Units are ⅓-trick points |
| C11 | Balanced 4333 is the weakest balanced pattern when NT is right (NT tricks − Binky: 4333 0.56–0.60 vs 5332 0.92–0.95) | Andrews, "Experts and 4333" | …/experts4333.html | Table | E (crude) | Assumes the right strain is always chosen |
| C12 | K-R (CCCC) 26-step algorithm (§3.3). Range −0.5…35.6. Mean 10.7725, median 10.65, mode 11.20, SD 4.7233 over all 635,013,559,600 hands. Step 8 is ambiguous in BW, so Pavlicek encodes "Kaplan's intent" | R. Pavlicek, "Kaplan-Rubens Evaluation Stats" (©2014) | rpbridge.net/8j19.htm | Step table | S (faithful re-statement by someone who knew Kaplan) | Primary BW Oct 1982 pp.21–23 not accessed. Mean discrepancy, §3.3.5 |
| C13 | K-R origin: Kaplan with Rubens, BW Oct 1982. "4 C's" = "Caution, Complex Computer Count" | Leicestershire bridgewebs page; Pavlicek AC01 | bridgewebs.com/leicestershire/page45.html | — | S | Anecdotal |
| C14 | Dealer implementation of `cccc()`/`quality()` "use the algorithms described in _The Bridge World_, October 1982", centipoints | Danil Suits, 1999, dealer `c4.c` | sources.debian.org/src/dealer/…/c4.c | Header and code | S (code) [RAW] | Differs from Pavlicek on Qx, KQ/AQ doubletons, tens in 7+ suits, step 8 |
| C15 | Deal (Andrews) `CCCC` "modeled … against Danil Suits implementation" but differs in Q and J handling for short suits | Deal 3.1.9 `lib/evaluators.tcl` | sources.debian.org/src/deal/3.1.9-15/lib/evaluators.tcl | `CCCC.holding` | S (code) [RAW] | A third variant |
| C16 | IMP thresholds for 4♠/5♦/6♥/7♣ by method. K-R: 24/27(26 vul)/31/34. Goren 26/29/33/36. Zar (halved) 27/29/32/34 | Pavlicek, "Point Count Methods" (©2022) | rpbridge.net/ac01.htm | Threshold tables | E (DD, 10,485,760 deals, ±4%/−8% declarer adj.) | Suit only. Adjustments and rounding are arbitrary (author's words) |
| C17 | Actual vs DD (vugraph 1996–2014, 79,942 contracts): make% 64.36 actual vs 62.03 DD; tricks 9.28 vs 9.21. NT 68.85% vs 63.98% (8.72 vs 8.56). Suit 62.58% vs 61.25% (9.50 vs 9.47). Slams 66.34% vs 68.30% | Pavlicek, "Actual Play vs. Double-Dummy" (©2013) | rpbridge.net/8j45.htm | Tables | E | Experts only (top team events). Selection bias: contracts that were actually bid |
| C18 | After the opening lead, DD beats actual play: 67.79% vs 64.35%, 9.44 vs 9.28 tricks; NT 71.82% vs 68.84% | same | same | "After opening lead" | E | Shows that the *opening lead* is the main DD distortion |
| C19 | LoTT (Ginsberg, ~446,741 DD deals): exact 40.0%; error distribution by total trumps; mean abs. error 0.75; within ±1 trick 86.9% [OURS from his table]. "Correct decisions approximately 70%" for ≤20 trumps | M. Ginsberg, "An Analysis of the Law of Total Tricks" (~1996–97; BW-era note) | Wayback of bocosan.tripod.com/ginsberg/total.HTML | Both tables | E [RAW] | Date not on the page. DD; "best side" declarer |
| C20 | LoTT average tricks by total trumps (10,485,760 deals): 14→14.34 … 18→18.01, 19→18.77, 20→19.48, 22→20.65. Fails on average at ≥20 trumps | Pavlicek, "LoTT Variance by HCP" (©2019) | rpbridge.net/8j69.htm | Table | E | Exact-match % in WebFetch output was computed by the summariser (not checked) |
| C21 | Wirgren (2002): LoTT "works on only 35–40% of deals"; Mendelson (1998): "within one trick on the vast majority" | Wikipedia LoTT | en.wikipedia.org/wiki/Law_of_total_tricks | Criticism | S | Primary texts not accessed |
| C22 | LoTT: Vernes, 1950s France; English 1966 *International Bridge Academy Annals*; BW June 1969 [citation needed]; Cohen 1992 *To Bid or Not to Bid* | Wikipedia LoTT | same | History | S | Vernes BW article not accessed |
| C23 | LTC: Courtenay 1934 *The System the Experts Play*; Rule of 18 (18 − losers = level), 24 − losers = tricks; partner's opening assumed 7 losers | Wikipedia LTC | en.wikipedia.org/wiki/Losing-Trick_Count | Basic method, History | S | Courtenay and Klinger books not accessed |
| C24 | NLTC: Koelman, BW May 2003 (vol. 74 no. 8 pp.26–28). Missing A 1.5, K 1.0 (len ≥2), Q 0.5 (len ≥3). Tricks = 25 − total; level = 19 − total | Wikipedia LTC | same | NLTC section | S | Koelman primary not accessed. Deal `newLTC` [RAW] matches the half-loser weights 3/2/1 |
| C25 | LTC refinements: AQ doubleton ½ (Klinger); Kx 1½; AJ10 1 (Harrison-Gray); KJ10 1½ (Magee); Qxx 2.5–3 unless trumps; −1 for a known 9-card fit; Crowhurst & Kambites 1992 pp.62–66 | Wikipedia LTC | same | Refinements | S | Attributions unchecked |
| C26 | Zar Points = HCP + controls (A2 K1) + (a+b) + (a−d). Open at 26. 5 ZP per level: game 52, slam 62, grand 67. Concentration +1 (11–14 HCP in 2 suits, 15+ in 3) | Petkov via csbnews (2016-04-09); Wikipedia (cites Petkov 2003 p.38, 2005 p.165) | csbnews.org/en/hand-evaluation-zar-points-by-zar-petkov/ | — | P (reprint) / S | Statistical claims ("over a million boards at DD") not reproducible from the page |
| C27 | Zar as used by Pavlicek: honours 6-4-2-1, −1 for singleton A/K/Q/J or doubleton KQ/KJ/QJ/Qx/Jx, 2×longest + 2nd − shortest | Pavlicek AC01 | rpbridge.net/ac01.htm | Zar block | S | Pavlicek's reading. Not identical to Petkov's article (§3.4) |
| C28 | Controls A=2 K=1. Expected controls by HCP: 5→1, 7–8→2, 10→3, 12–13→4, 15→5, 17–18→6, 20→7 (Rosenkranz, BW Dec 1974) | Wikipedia Hand evaluation | — | Controls | S | — |
| C29 | Quick tricks: AK 2, AQ 1.5, A 1, KQ 1, Kx ½ | Wikipedia (Klinger 1994) | — | Quick tricks | S | Culbertson original not accessed |
| C30 | Playing tricks: A 1, AK 2, AKQ 3, KQ 1, KQJ 2, plus cards beyond 3 in long suits. Acol strong two = 8 PT (Landy 1998) | Wikipedia | — | Playing tricks | S | — |
| C31 | SQT = suit length + number of honours (A, K, Q; J and T count only if A, K or Q present). 7 → 1-level, 8 → 2-level, 9 → 3-level overcall; needs 5+ cards (Klinger 1994) | Wikipedia | — | SQT | S | Klinger primary not accessed |
| C32 | Rule of 20: HCP + lengths of the two longest suits ≥ 20 → open (high cards mostly in the long suits) | Wikipedia (Francis et al. 2001 p.401) | — | Rule of 20 | S | — |
| C33 | Length points 5→1, 6→2, 7→3. Shortness (dummy) points with 3 trumps 3/2/1, with 4+ trumps 5/3/1 (Downey & Pomer 2005) | Wikipedia | — | Distribution | S | Many incompatible variants exist |
| C34 | Bergen "Adjust-3": +1 if (aces + tens) − (queens + jacks) ≥ 3; −1 if the reverse ≥ 3. Rule of 20 also in Bergen | Lesson PDFs (bridge-tips.co.il, unit524) | bridge-tips.co.il/…Adjust-3-Method.pdf | — | S (not fetched; search snippets) | Bergen *Points Schmoints!* not accessed |
| C35 | Goren, Karpin, Pavlicek, Modified Bissell, Zar point-count rules (full lists in §3.6) | Pavlicek AC01 | rpbridge.net/ac01.htm | Method blocks | S | Pavlicek's codification |
| C36 | Cowan (1987): 5-4-3-2-1 matches suit-isolation trick averages for balanced pairs. Andrews: "a 5-4-3-2-1 evaluator scores very badly" because it ignores tempo | Andrews, "Cowan's 5-4-3-2-1 Result" (©2005) | …/cowan.html | — | E/O | Cowan primary not accessed |
| C37 | DD biases: always guesses queens (two-way finesses), picks up Txxxx onside (single-dummy misses about 2× as often), Kxx/QJxx overvalued | Andrews, "Problems with DD data" | …/doubledummy.html | — | O (reasoned) | No population numbers |
| C38 | Single-dummy (actual-play) data has its own problems: skill mix, auction-dependent defence, contract-dependent play, sample size | Andrews, "Problems with single dummy data" | …/singledummy.html | — | O | — |
| C39 | NN on DD (GIB library 717,102 deals; 100k test). Best NN spades: exact 53.11%, ±1 96.48%. NT: exact 37.80%, ±1 84.31%. Raw WPC applied directly: NT exact 22.76% / ±1 62.07%; spades 16.41% / 48.25%. Human pros (DDBP-4): NT 73.68% exact (Group 1) | Mossakowski & Mańdziuk, IEEE TNN 2009 | pages.mini.pw.edu.pl/~mandziukj/PRACE/IEEETNN09.pdf | Abstract, Tables VII, X; §IX-B | E [RAW] | All four hands visible (DDBP-4) unless stated |
| C40 | Linear 52→1 NN learned NT card weights (scaled 0–4): 2≈0.34, 8≈0.36–0.40, 9≈0.46–0.50, T≈0.66–0.68, J≈1.03–1.06, Q≈1.66–1.69, K≈2.64–2.68, A≈3.97–3.99 | same | same | Table VII | E [RAW] | Includes a constant per card. Subtracting the "2" baseline is my step (§5.4) |
| C41 | Point scales used as NN inputs: Bamberger 7-5-3-1, Collet 4-3-2-½-½, "Four aces" 3-2-1-½, Polish 7-4-3, Reith 6-4-3-2-1, Robertson 7-5-3-2-1, Vernes 4-3.1-1.9-0.9, AKQ 4-3-2 | same | same | Table VIII | S [RAW] | Original references are encyclopedias ([8],[67]) |
| C42 | BridgeHand2Vec (2023): NN for DDBP-2 trained on **mean of DDS over 10 random EW layouts**. Error <0.5 trick: suit 83.95%, NT 72.21%. <1: 99.16% / 94.86% | Sztyber-Betley et al., arXiv 2310.06624 | arxiv.org/abs/2310.06624 | Tables 3–4 | E [RAW] | Target is an average, so these are **not comparable** with exact-DD accuracy. Its Table 4 appears to swap the suit/NT labels when citing [17] |
| C43 | Logistic regression of P(10+ tricks, DD) on a 4-4 spade fit (198,382 GIB deals). Trump honours A 3.66 K 2.51 Q 1.53 J 0.87; side 4+ suit A 3.42 K 2.07 Q 1.01 J 0.53; shortness (4-4) void 2.99 single 1.88 dbl 0.71; (5-4) 4.02/2.36/0.78; 9th trump 1.10 | helene_t, BBO forum (2009-05-19) | bridgebase.com/forums/topic/32125 | Post 1 + follow-ups | E (amateur) | No SEs. 4-4/5-4 fits only. Logit units, not tricks |
| C44 | TSP (Streib 2004): A6 K4 Q2 J1; +1 per suit with ≥2 honours (T counts); shortness 5/3/1; +1 per card over 4. "ERROR" (trick gap): HCP 1.23, HCP+321 1.07, Zar 1.05, BUMRAP+531 1.02, TSP 1.02, Binky 0.99 | Tysen Streib, BBO forum 3278 (2004-05-28) | bridgebase.com/forums/topic/3278 | Table | E (method undisclosed) | Dataset and metric not specified. Petkov disputed the numbers |
| C45 | Kidd: DD category frequencies over ~700k Ginsberg deals: partial 39.81%, game 46.59%, slam 10.65%, grand 2.94%. World-class teams bid ~9% more games than DD-makeable | M. Kidd (2015-05-19) | lajollabridge.com/Articles/PartialGameSlamGrand.htm | — | E | Not an evaluator study |
| C46 | Zhang (Berkeley undergraduate): single-hand correlations with tricks: distribution points ≈0.29, controls ≈0.08, losers ≈−0.2 (1000-deal sample) | Zhang & Aldous (n.d., ~2015) | stat.berkeley.edu/~aldous/Research/Ugrad/Z_Zhang_project.pdf | §2 | E (weak) | Tiny sample. Partner's hand ignored. Low quality |
| C47 | GIB: Monte Carlo sampling of hidden hands + DD solving (partition search) | Ginsberg, JAIR 14:303–358 (2001), doi 10.1613/jair.820 | jair.org/index.php/jair/article/view/10279 | Abstract | P | Full text not read |
| C48 | Deal / Redeal: constrained deal generators with DDS for simulation; BEN: NN bidding/play + DDS plus a NN "single dummy estimates" model | Andrews (Deal); A. Lee (Redeal); L. Dali (BEN, 2022) | github.com/anntzer/redeal; github.com/lorserker/ben | READMEs | P (software) | — |

---

## 3. Exact definitions (implementation-grade)

Notation: holding = cards in one suit; `L` = length; `A,K,Q,J,T,9,8` are booleans for presence.
"Higher honours" means counted among A, K, Q (and J where noted).

### 3.1 Linear honour scales (per card; sum over 4 suits)

| Scale | A | K | Q | J | T | 9 | Deck total | Source |
|---|---|---|---|---|---|---|---|---|
| Work / HCP | 4 | 3 | 2 | 1 | 0 | 0 | 40 | C1 |
| Fifths (Andrews, 3NT) | 4 | 2.8 | 1.8 | 1 | 0.4 | 0 | 40 | C2 |
| BUM-RAP | 4.5 | 3 | 1.5 | 0.75 | 0.25 | 0 | 40 | C3 |
| WPC + ½T (Andrews) | 4 | 3 | 2 | 1 | 0.5 | 0 | 42 | C2 |
| WPC + ½T + ¼A (Andrews) | 4.25 | 3 | 2 | 1 | 0.5 | 0 | 43 | C2 |
| Cowan 5-4-3-2-1 (balanced pairs, NT) | 5 | 4 | 3 | 2 | 1 | 0 | 60 | C36 |
| HCP + controls (Zar, TSP) | 6 | 4 | 2 | 1 | 0 | 0 | 52 | C26/C44 |
| Bamberger | 7 | 5 | 3 | 1 | 0 | 0 | 64 | C41 |
| Collet | 4 | 3 | 2 | 0.5 | 0.5 | 0 | 40 | C41 |
| "Four aces" | 3 | 2 | 1 | 0.5 | 0 | 0 | 26 | C41 |
| Polish points | 7 | 4 | 3 | 0 | 0 | 0 | 56 | C41 |
| Reith | 6 | 4 | 3 | 2 | 1 | 0 | 64 | C41 |
| Robertson | 7 | 5 | 3 | 2 | 1 | 0 | 72 | C41 |
| Vernes | 4 | 3.1 | 1.9 | 0.9 | 0 | 0 | 39.6 | C41 |
| AKQ points | 4 | 3 | 2 | 0 | 0 | 0 | 36 | C41 |
| Modified Bissell (Pavlicek) | 3 | 2 | 1 | 0 | 0 | 0 | 24 | C35 |
| Controls | 2 | 1 | 0 | 0 | 0 | 0 | 12 | C28 |

The "Vienna" count was asked about. **I did not verify** which scale is meant by "Vienna"
(Bamberger 7-5-3-1 is often linked with it, but no source found today makes that link).
Treat it as unverified.

3NT thresholds, Andrews main sample ("Goal" = combined points for 50% games): WPC 25.00,
BUM-RAP 24.75, WPC+½T 25.50, +¼A 26.25, Fifths 24.20. At 40%: 24.00 / 24.50 / 25.00 / 25.50 /
23.80. [WF]

**Test fixtures** (by definition): `AKQJ AKQ AKQ AKQ` gives HCP 37, Fifths 4+2.8+1.8+1 + 3×(4+2.8+1.8)
= 9.6 + 25.8 = 35.4, BUM-RAP 9.75 + 3×9 = 36.75. The yarborough scores 0 on all of them.

### 3.2 Losing Trick Count family

**Classical LTC** (Courtenay 1934, per Wikipedia C23). Per suit, look only at the top
`min(L,3)` cards:
- L = 0: 0. L = 1: 0 if A else 1. L = 2: AK 0; Ax or Kx 1; Qx or xx 2. L ≥ 3: count missing
  A, K, Q among the top three, so AKQ 0; AKx/AQx/KQx 1; Axx/Kxx/Qxx 2; xxx 3.
- Equivalent formula: `losers = [L≥1 ∧ ¬A] + [L≥2 ∧ ¬K] + [L≥3 ∧ ¬Q]`. This is the same as Deal's
  `losers` and jdh8 `ltc`. Note Kx = 1 and a singleton K = 1.
- Tricks = 24 − (my losers + partner's losers). Level = 18 − total ("Rule of 18"). Partner's
  opening assumed to be 7 losers. Use only after an ~8-card trump fit.
- Published worked example: `AQxx Qxx Kxxx Qx` = 1+2+2+2 = **7 losers**. With 7 + 7 = 14:
  **10 tricks** / 4-level. [WF, verified by hand]

**Refinements** (secondary, C25): AQ dbl = ½ (Klinger); Kx = 1½; KQ dbl = 1; AJT = 1
(Harrison-Gray); KJT = 1½ (Magee); Qxx = 2.5–3 unless trumps; QJx = 2; −1 loser with a known
9-card fit. Klinger *Modern LTC* (2nd ed. 2011, p.13) discusses control-count adjustments
(content not accessed).

**NLTC** (Koelman, BW May 2003, per Wikipedia C24, not the primary):
- Per suit: `+1.5 if L≥1 ∧ ¬A; +1.0 if L≥2 ∧ ¬K; +0.5 if L≥3 ∧ ¬Q`. Void 0.
- So: singleton non-A 1.5; xx 2.5; Kx 1.5; Ax 1.0; AQ 1.0; KQ 1.5; AKx 0.5; AQx 1.0; Axx 1.5;
  KQx 1.5; Kxx 2.0; Qxx 2.5; xxx 3.0.
- Tricks = **25 − (sum of both hands)**. Level = **19 − total**.
- Deal's `newLTC` [RAW] returns half-losers 3/2/1 with the same length gates. jdh8 `nltc` is
  identical.
- Published fixtures: `Axxx Axx Axx Axx` = **6** (LTC 8); `Kxxx Kxx Kxx Kxx` = **8** (LTC 8);
  `Qxxx Qxx Qxx Qxx` = **10** (LTC 8). [WF, re-derived by hand]
- Wikipedia (unverified) also mentions a "modified LTC (mLTC)" used by helene_t: A 1.5 (1 if
  stiff), K 1 (0 if stiff), Q 0.5 (0 if stiff or doubleton), void 3, singleton 2, doubleton 1.
  This is given in C43 as a *value* scale, not a loser count.

### 3.3 Kaplan–Rubens "Four C's" (CCCC), BW Oct 1982 — three non-identical codifications

#### 3.3.1 Pavlicek 26-step statement (rpbridge 8J19, quoted step text [WF])

Per suit, accumulate part 1:
1. Ace 4
2. King 3
3. Queen 2
4. Jack 1
5. Ten 0.5 (no length condition stated)
6. "2-6 cards with ten accompanied by jack or two (or more) higher honors": +0.5
7. "2-6 cards with nine accompanied by eight, ten, or exactly two higher honors": +0.5
8. "4-6 cards with nine (no eight or ten) and exactly three higher honors": +0.5. **Pavlicek's
   interpretation of an ambiguous BW rule.**
9. "7+ cards missing queen or jack (or both)": +1
10. "8+ cards missing queen": +1
11. "9+ cards missing queen and jack": +1
12. "Multiply current total by suit length, divide by 10."

Then add part 2 (not multiplied):

13. Ace +3
14. King guarded +2
15. King singleton +0.5
16. Queen (3+ cards) with A or K (or both) +1
17. Queen (3+) with no higher honour +0.75
18. Queen doubleton with A or K +0.5
19. Queen doubleton with no higher honour +0.25
20. Jack with exactly two higher honours +0.5
21. Jack with exactly one higher honour +0.25
22. Ten with exactly two higher honours +0.25
23. Ten and nine with exactly one higher honour +0.25
24. Void +3
25. Singleton +2
26. Doubleton +1

**Final:** add the four suit totals, subtract 1, and if the hand is 4-3-3-3 add 0.5.

The page does not define "higher honours" for steps 7–8 and 20–23. The implementation in jdh8
(F11) counts {A,K,Q} for J (steps 20–21) and {A,K,Q,J} for T and 9 (steps 7, 8, 22, 23). Steps
9–11 telescope to: extra = +1 (L≥7 and not both Q and J) +1 (L≥8 and no Q) +1 (L≥9 and neither
Q nor J). Since the shortness credits are 3/2/1 and 1 is subtracted, a hand with exactly one
doubleton gets 0 net shape. Flat 4333 gets −0.5. Everything is in multiples of 0.05.

#### 3.3.2 Dealer `c4.c` (Danil Suits 1999, "algorithms described in TBW Oct 1982") [RAW]

Quality part (same as Deal `Quality`), with `SF = 10·L` and `HH` = count of {A,K,Q,J} held:
`Q = SF·(4A+3K+2Q+J)`. If `L > 6`: `rc = 3 − 2·[Q] − [J]`, `rc = min(rc, L−6)`, `Q += rc·SF`.
**Tens and nines are ignored for L>6.** Else, if T: `Q += SF` if (HH>1 or J) else `SF/2`; if 9 and
(HH==2 or T or 8): `Q += SF/2`. There is no step 8.

Honour part (centipoints): A +300. K +200, or +50 if singleton. Q +100, −75 if L==1, −25 if L==2,
−25 if no higher honour. So singleton Q = 0, AQ/KQ dbl = **75**, Qx = **50**, Q(3+) with higher
= 100, Q(3+) without = 75. J: +50 if exactly 2 higher, +25 if 1 (any length). T: +25 if HH==2 or
(HH==1 and 9). Shape: if there is no suit ≤2, −50; else Σ(3−L for L≤2)·100 − 100.

#### 3.3.3 Deal `CCCC` (Andrews, "modeled … against Danil Suits") [RAW]

Same quality part and shape as dealer. Honour part differs: Q with L≤2 gets +50 only if exactly
one higher honour (so **Qx = 0**, AQ/KQ dbl = 50). J credited **only if L>2**.

#### 3.3.4 Where the codifications disagree [OURS]

Values below come from a scratch Python transcription of all three (not committed):

| Hand (S H D C) | Pavlicek (8J19 / jdh8) | dealer c4.c | Deal Tcl | Cause |
|---|---|---|---|---|
| `xxxx xxx xxx xxx` | **−0.50** | −0.50 | −0.50 | (published by Pavlicek) |
| `AKQJ AKQ AKQ AKQ` | **35.60** | 35.60 | 35.60 | Pavlicek publishes 35.6 as the maximum ("7NT claimer"; his table lists 8 hands at 35.60 [WF]). That this hand is one of the 8 is my derivation. The other 4 were not identified |
| `Qx AKxx Kxxx Axx` | 15.85 | 16.10 | 15.60 | Qx bonus 0.25 / 0.50 / 0 |
| `KQ Axxx Kxxx Axx` | 15.50 | 15.75 | 15.50 | KQ dbl bonus 0.5 / 0.75 / 0.5 |
| `AKTxxxx Kx xx xx` | 15.80 | 15.45 | 15.45 | Step-5 ten in a 7+ suit (Pavlicek literal) vs dropped |
| `KJ Axxx Kxxx Axx` | 15.05 | 15.05 | 14.80 | Deal skips J in doubletons |
| `AKQ9 xxx xxx xxx` | 9.30 | 9.10 | 9.10 | Step 8 (Pavlicek only) |
| `AQxx KJx Kxx Qxx` | 13.60 | 13.60 | 13.60 | Agreement |

A secondary summary (Leicestershire, F22) gives yet another reading: "two **or more** of AKQJ"
for the nine, and "Ten … 0.50 with a 9 and one higher honour". Treat it as unreliable.

#### 3.3.5 Distribution check against Pavlicek's published statistics [OURS]

Exact expectation by enumerating all 8192 holdings per suit plus all shapes, then a
200,000-hand Monte Carlo (seed 1):
- Exact mean: Pavlicek-literal **10.8724**, dealer 10.8911, Deal 10.8251. Pavlicek reports
  **10.7725**.
- MC (Pavlicek-literal): median **10.65** (matches the published 10.65), SD 4.715 (published 4.7233).
- **Unresolved:** the reported mean is about 0.10 below all three codifications, while the median
  matches. Either Pavlicek's mean has an arithmetic slip, or his code differs from his own step
  table in a way that leaves the median unchanged. The published frequency table (e.g. 27,500,000
  hands at exactly −0.50; 8 hands at 35.60) can serve as an exact enumeration fixture once a
  variant is chosen. Recommendation: implement the Pavlicek step table as canonical (it is the
  most widely cited, and jdh8's Rust implementation follows it), flag `variant=dealer|deal` as
  alternatives, and test against the −0.5 / 35.6 endpoints plus counts from his frequency table.

Rounding to integers (Pavlicek AC01, partnership totals): fractions ≥12/20 round up, ≤8/20 round
down; 9/20–11/20 round up iff partnership body = 3·tens + 2·nines + eights ≥ 12.

### 3.4 Zar Points

- **Petkov (csbnews reprint, 2016)**: `ZP = HCP + controls(A2,K1) + (a+b) + (a−d)`, where
  a≥b≥c≥d are the suit lengths. Equivalent distribution form `(13 − 2d) + (a − c)` (per search
  snippet of Petkov's paper). Open with **26**. Concentration +1 (11–14 HCP all in two suits;
  15+ HCP all in three suits). Spades +1 only to move 25 → 26. Respond with 16. Fit: trump
  honours (incl. T) +1 each, max +2; second-suit honours max +2; **+3 per trump beyond 8**
  (csbnews). Wikipedia instead gives +2 per extra trump with a void, +1 with a singleton.
  **Sources disagree.** Levels: 52 game (4M/3NT), 57 five-level, 62 small slam, 67 grand.
  Wikipedia: tricks = (ZP − 2)/5.
- **Pavlicek's codified Zar (AC01)** adds −1 for singleton A/K/Q/J and for doubleton
  KQ/KJ/QJ/Qx/Jx, uses `2a + b − d` (identical to (a+b)+(a−d)), and halves partnership totals.
- **jdh8 `zar`** [RAW] uses a different short-honour deduction: singleton K/Q/J, doubleton with Q or J.
- **Published fixtures** (base formula, no adjustments; all re-derived by hand):

| Hand (S H D C) | HCP | Ctrl | a+b | a−d | Published ZP |
|---|---|---|---|---|---|
| `x Kxxxx Kxxx Axx` | 10 | 4 | 9 | 4 | **27** |
| `KQxxx KJxxx xxx -` | 9 | 2 | 10 | 5 | **26** |
| `Kxxxxx Axxxx xx -` | 7 | 3 | 11 | 6 | **27** |
| `Axxx AJxxxx x xx` | 9 | 4 | 10 | 5 | **28** |
| `AQxx Jx Axxx xxx` | 11 | 4 | 8 | 2 | **25** (+1 spade rule = 26) |
| Shape-only: 4333 / 7600 / 13-0-0-0 | — | — | — | — | distribution **8 / 20 / 26** |

  Two other printed examples are defective: `Axxx ATxxxx xxxx -` has 14 cards, and
  `Qx AKxxx Jxxxx x` is stated as 9 HCP but has 10. Do not use them as fixtures. Under
  Pavlicek's codification the `Jx` hand scores 24, which shows that the variants diverge.

### 3.5 Binky Points (Andrews, 2008 tables)

`value(hand, strain) = pattern[sorted shape][strain] + Σ_suits holding[h][strain]`, in **tricks**,
additive across partners (the pair sum estimates tricks). Holdings use spot cards 2–8 → `x`. The
data is in Deal `lib/binky-data.tcl` (551 rows; columns NT, suit) [RAW]. Examples (NT, suit):
pattern 4-3-3-3 3.12/3.60; 5-4-4-0 3.17/5.12; 13-0-0-0 −0.59/7.69. Holdings: `-` 0/0; `x`
−0.29/−0.17; `A` 1.84/1.16; `xx` −0.76/−0.49; `AK` 2.79/1.93; `AQ` 2.36/1.55; `xxx` −1.26/−0.84;
`Axx` 1.06/0.77; `Kxx` 0.29/0.20; `Qxx` −0.34/−0.30. License: GPL-2+. Re-deriving from DD data
is preferable to copying.

### 3.6 Additive point-count methods as codified by Pavlicek (AC01) [WF]

- **Goren:** 4-3-2-1; void 3, singleton 2, doubleton 1; singleton K or Q −2, singleton J −1;
  doubleton QJ/Qx/Jx −1; four aces +1, aceless −1. As dummy with 4 trumps: void +2, singleton +1.
  Dummy with trump K/Q/J (<4 trump HCP) +1. Dummy 4333 −1. Declarer with 5 trumps +1, each trump
  over 5 +2.
- **Karpin:** 4-3-2-1; +1 per card over 4 in any suit; singleton K/Q, doubleton KQ/KJ/QJ/Qx/Jx −1.
  Dummy: 3 trumps void +2, singleton +1; 4 trumps 3/2/1. Declarer: 5 trumps void +2, singleton +1;
  6 trumps 3/2/1.
- **Pavlicek:** 4-3-2-1 + 3/2/1 shortness, but for a short honour (singleton K/Q/J, doubleton
  KQ/KJ/QJ/Qx/Jx) take max(HCP, shortness). No shortness in partner's suit unless there is another
  fit. +1 for four aces and/or four tens. Dummy with 4 trumps void +2, singleton +1. Aceless 4333
  dummy with minimal fit −1. Declarer: 5 trumps (if not shown) +1; each trump over 5 +2; each side-suit
  card over 3 +1.
- **Modified Bissell:** 3-2-1; per suit 4+ cards +1, 5+ +2, each over 5 +3; ace with 3+ cards +1;
  A or K with 4+ cards +1; ≥2 of AKQJ +1; ≥2 of QJT9 with 3+ cards +1; short-honour −1; cap per suit
  = 3·L.
- **TSP (Streib 2004, C44):** 6-4-2-1; +1 per suit with ≥2 honours (T counts), except doubletons,
  where QJ is −1; singleton honours demoted one step (A4 K2 Q1 J0); shortness 5/3/1; +1 per card
  over 4; no shortness in partner's suit without another 8-card fit; +2 per trump beyond 8. NT: HP
  plus 1 per 5+ suit; targets 37 for 3NT, 44 for 6NT, 48 for 7NT.
- **Kleinman "DK" (BW letter context, via F22):** A13 K9 Q5 J2; T with 1 or 2 higher honours
  +1/+2; +1 for AK/AQ/AJ/KQ/KJ; +1 for AQJ/KQJ; +2 for AKQ/AKJ/AKQJ; −1 per unguarded picture card;
  −1 for a suit whose lowest card is above the T; −2 for 4333; ÷3, with remainder 0/1/2 = bad /
  ordinary / good. Secondary source only.

### 3.7 Other small evaluators (Wikipedia-level definitions, §2)

- **Quick tricks:** per suit AK 2, AQ 1.5, A 1, KQ 1, Kx 0.5, singleton K 0. Sum over suits.
- **Playing tricks:** A 1, AK 2, AKQ 3, KQ 1, KQJ 2, plus each card beyond 3 in a long suit.
  Strong two = 8.
- **SQT:** `L + #{A,K,Q} + (#{J,T} if any of A,K,Q present)`; overcall level = SQT − 6 (7→1,
  8→2, 9→3); require L ≥ 5.
- **Rule of 20:** `HCP + a + b ≥ 20` (Rule of 19 variant).
- **Bergen Adjust-3:** `n = (#A + #T) − (#Q + #J)`; +1 if n ≥ 3, −1 if n ≤ −3. Length points:
  1 per card over 4. Dummy points: doubleton 1, singleton 2 (3 with 4+ trumps), void (variants:
  3, 4 or 5).
- **LoTT:** total tricks (both sides, each in its longest fit) ≈ total trumps. Adjustments
  (purity, double fits, honour position) are in Cohen 1992 (not accessed).

---

## 4. Comparison matrix

| Method | Original purpose | Definition summary | Permitted inputs | Stage / strain | Fitted assumptions | Strengths | Known failure cases | Evidence | Mental cost |
|---|---|---|---|---|---|---|---|---|---|
| HCP 4-3-2-1 | Opening and game decisions, NT | Linear A4 K3 Q2 J1 | Own hand | Any; best in NT | Equal-step honour values | Universal, additive, ½-trick per point in NT | Undervalues A and T, overvalues Q and J (DD); blind to shape in suits | DD: 3NT error 38.99 (Andrews); NN study exact 22.8% NT / 16.4% ♠ (raw WPC, all four hands) | Very low |
| Fifths | 3NT game decisions | A4 K2.8 Q1.8 J1 T0.4 | Own hand | NT, balanced | Fitted to DD 3NT make-probability | Best of tested scales for 3NT (error 6.31) | Not for suits; fractional borders | DD only (Andrews) | Low–medium |
| BUM-RAP | Suit-contract strength | A4.5 K3 Q1.5 J.75 T.25 | Own hand | Suits | DD-ish ace premium | Corr 0.914 with suit tricks (with 3/2/1 shape) | Worse than HCP for 3NT (59.85 vs 38.99) | DD (Andrews) | Low–medium |
| WPC+½T(+¼A) | 3NT | as named | Own hand | NT | DD fit | 3NT error 16.12 / 9.70 | Deck not 40 | DD | Low |
| Controls | Slam (control asks) | A2 K1 | Own hand | Slam | — | Pairs with RKCB and cue bids | Correlation with tricks alone is low (≈0.08, weak study) | Weak | Very low |
| Quick tricks | Defence and opening strength (Culbertson) | AK2 AQ1.5 A1 KQ1 Kx½ | Own hand | Opening, defence | — | Defensive potential | Ignores length | None found | Low |
| Playing tricks | Strong twos, pre-empts | Honour tricks + long cards | Own hand | Suit, offence | Normal splits | Direct trick estimate | Assumes splits | None found | Medium |
| LTC (Courtenay) | Fit-based level | 24 − Σlosers; 7 = opening | Own + partner's assumed 7 | After an 8+ fit; suits | Top-3 cards, A=K=Q | Captures shape and fit | Treats A=K=Q as loser savers; overvalues unsupported Q; shortness overvalued (helene_t) | DD logistic critique (C43) | Low |
| NLTC (Koelman) | Same, weighted | 25 − Σ(1.5A/1K/0.5Q missing) | Own + partner | After an 8+ fit | Half-finesse | Fixes ace/queen imbalance | Same fit dependence | None found beyond author examples | Low–medium |
| Zar | Aggressive opening and level decisions | HCP+ctrl+(a+b)+(a−d); 26 open, 5/level | Own; fit adjustments | Opening, competitive, game/slam | Fitted by author (unpublished data) | Shape-sensitive opening | Undisclosed fitting; NN study: Zar-only ♠ exact 16.8%; LoTT-like fit rules vary between sources | Pavlicek thresholds; Streib error 1.05 | Medium |
| K-R CCCC | Post-mortem "true" value | 26 steps; length-weighted quality + 3-2-1 + shape | Own hand | Opening and invites; suits | Expert judgement (Kaplan) | Best single-hand correlate for suit offence (0.518) | Worse than HCP in NT (0.481 vs 0.507); ambiguous step 8; 3+ divergent codings; 4♠ threshold 24 (not 26) | DD (Andrews, Pavlicek) | Very high (computer) |
| Binky | Additive trick estimator | Pattern + holding tables (tricks) | Own hand (pair sum) | NT and suit separately | Fitted to DD (2008) | SD 0.99 (suit) / 1.26 (NT) vs DD; corr 0.925 / 0.947 | No fit/duplication; DD optimism; stiff Q oddity | DD 100k validation | Table lookup (computer) |
| TSP | Simple integer Binky substitute | 6-4-2-1 + combo + 5/3/1 + length | Own; fit | Suits; NT variant | Tuned against Binky | Error 1.02 ≈ Binky 0.99 | Undisclosed test set | Forum only | Medium |
| Goren / Karpin / Pavlicek / Bissell | Textbook level decisions | §3.6 | Own + fit | Suits | Expert rules | Standard 26/29/33/36 thresholds (Goren, Pavlicek) | Double counting of short honours (Goren deducts) | DD thresholds (AC01) | Low–medium |
| SQT | Overcall / pre-empt suit adequacy | L + honours | Own suit | Competitive | — | Simple | No published accuracy | None | Very low |
| Rule of 20 | Light openings | HCP + a + b ≥ 20 | Own | Opening | — | Simple | Ignores honour location (unless the rule is applied with judgement) | None | Very low |
| Bergen Adjust-3 | HCP correction | ±1 for A+T vs Q+J imbalance | Own | Any | — | Approximates BUM-RAP direction | Coarse | None | Low |
| LoTT | Competitive level | Total tricks ≈ total trumps | Own + inferred fits | Competitive partscores | Normal honours | 70% correct decisions (≤20 trumps) | Exact on only 40% of deals; overestimates at ≥20 trumps | Ginsberg (446,741 deals), Pavlicek (10.5M) | Low |
| NN DD predictors | Fast trick estimation | Learned | All 4 hands (DDBP-4) or 2 (DDBP-2) | Both | Trained on DD | Exact 53% (♠, 4 hands) | Black box; DD target | MM09, BridgeHand2Vec | n/a (computer) |
| Monte Carlo + DDS (GIB, Deal, Redeal, BEN) | Decision under hidden hands | Sample constrained deals → DD | Auction constraints | Any | Sampling model | Captures fit, duplication, honour location jointly | Inherits DD bias; constraint mis-specification | GIB (JAIR 2001) | n/a (computer) |

---

## 5. Known empirical results on evaluator accuracy

### 5.1 Andrews — 3NT card values (DD) [WF]
Setup: DD simulations estimating P(≥9 NT tricks) as a function of the side's A/K/Q/J/T counts.
South balanced; no 8-card major fit; North not freakish; no doubleton Qx/Jx unsupported; North
has no singleton K. Error = Σ over rows of |P−50%|·freq for rows where the evaluator's
bid/pass decision is wrong.

| Evaluator | Goal @50% | Error @50% | Goal @40% | Error @40% | Less-ideal @50% err | Less-ideal @40% err |
|---|---|---|---|---|---|---|
| WPC | 25.00 | 38.99 | 24.00 | 65.12 | 32.13 | 71.95 |
| BUM-RAP | 24.75 | 59.85 | 24.50 | 67.06 | 61.76 | 61.21 |
| WPC+½T | 25.50 | 16.12 | 25.00 | 12.15 | 16.86 | 20.40 |
| +¼A | 26.25 | 9.70 | 25.50 | 9.87 | 7.50 | 10.90 |
| Fifths | 24.20 | **6.31** | 23.80 | **9.18** | **6.71** | 13.17 |

### 5.2 Andrews — single-hand correlation with DD tricks (GIB library, 700k+ deals, partial run) [WF]
- NT offence: offence.nt 0.5174, p6421 0.5110, **hcp 0.5071**, hcpplus 0.4904, **CCCC 0.4812**,
  controls 0.4791, losers −0.3767.
- Suit offence: offence.suit 0.5311, **CCCC 0.5182**, hcpplus 0.5051, **hcp 0.4669**, controls
  0.4487, losers −0.4888.
- Andrews' conclusion: a ceiling of about 0.5 for any single-hand evaluator, because partner's
  hand matters. This motivates additive pair evaluators.

Single honour, average DD tricks (best offence / NT offence / suit offence): A 9.62/7.23/9.61;
K 8.97/6.26/8.96; Q 8.89/6.02/8.88; J 8.85/5.92/8.85; T 8.82/5.86/8.81; 9 8.82/5.85/8.81.

### 5.3 Andrews — Binky validation (DD, ©2008) [WF]
- 100,000 DD deals: mean error suit 0.005 / NT 0.001; **SD 0.99 / 1.26**. A constant baseline
  gives SD 2.11 / 2.78.
- Against the 20-deal average for a fixed NS pair: SD **0.76 / 0.83**.
- Correlations: Binky-suit 0.925, BUM-RAP(+shape) 0.914, Binky-NT 0.947, Fifths 0.931,
  Binky-NT vs Fifths 0.982.
- Interpretation: about 0.8 trick of irreducible noise from the opponents' layout even for a
  perfect pair evaluator. The DD vs 20-average gap (0.99→0.76) bounds what any NS-only evaluator
  can achieve.

### 5.4 Mossakowski & Mańdziuk 2009 (IEEE TNN), GIB library 717,102 deals, 100k test [RAW]
Format: % within ±2 | ±1 | exact.
- Best NN, all four hands: NT 97.34 | 84.31 | 37.80; ♠ (rotating lead) 99.88 | 96.48 | 53.11.
- NN from WPC sum only: NT 93.73 | 76.41 | 31.37; ♠ 76.22 | 49.55 | 16.93.
- NN from WPC + 16 suit lengths: NT 93.73 | 76.34 | 31.32; ♠ **96.98 | 82.43 | 35.36**. Length
  information is worth roughly double the exact-hit rate in suits.
- NN from Zar per hand: NT 91.98 | 71.77 | 28.21; ♠ 77.45 | 49.98 | 16.76.
- NN from 9 point-count scales: NT 94.87 | 78.30 | 32.39; from 8 distributional methods: ♠ 98.53 |
  87.64 | 39.93.
- Raw human estimators applied directly (no NN): WPC NT 86.44 | 62.07 | 22.76, ♠ 74.64 | 48.25 |
  16.41; Asset NT 79.99 | 55.14 | 19.68; "Stayman system" NT 89.10 | 67.63 | 26.02, ♠ 74.11 |
  50.96 | 18.32.
- 24 Polish pros, 30 s per deal (DDBP-4): ♠ exact 53.06% (top 10) / 48.66%; NT exact 73.68% /
  60.78%. Humans beat the NN only in NT with all four hands visible.
- Learned linear NT weights (Table VII), baseline "2" ≈ 0.34. Subtracting the baseline [OURS]
  gives A≈3.64, K≈2.31, Q≈1.34, J≈0.70, T≈0.33, 9≈0.13. Rescaled to A=4: **K 2.54, Q 1.47,
  J 0.77, T 0.36, 9 0.14**. This is close to BUM-RAP's Q/J/T and to Fifths' T, but the K is lower
  than in any human scale. Caveat: it comes from a 4-hand linear model with a per-card constant.

### 5.5 BridgeHand2Vec (2023) [RAW]
DDBP-2 (NS only) with the target = mean DDS over 10 EW layouts. Error <0.5 / <1 / <2: suit
83.95 / 99.16 / 99.996%; NT 72.21 / 94.86 / 99.78%. Because the target is an average, the noise
floor of §5.3 is removed. Its numbers **cannot** be set beside exact-DD hit rates.

### 5.6 Pavlicek — thresholds by method (DD, 10,485,760 deals, IMPs) [WF]
4♠ / 5♦ / 6♥ / 7♣ thresholds: Goren 26/29/33/36; Karpin 25/28/32/35; Pavlicek 26/29/33/36;
Bissell 25/28/33/36; Zar (halved) 27/29/32/34; **K-R 24/27 (26 vul)/31/34**. K-R at 4♠ (nonvul):
23 → 33.00% make, 24 → 47.37%, 25 → 61.60%. Declarer boost: +4% (games), 0 (6♥), −8% (7♣). No
single accuracy score was published.

### 5.7 DD vs actual play (Pavlicek 8J45) [WF]
79,942 expert vugraph contracts (1996–2014). Actual declarers make **more** than DD overall
(64.36% vs 62.03%; NT +4.9 pp, suits +1.3 pp; tricks +0.07). At slams DD is ahead (68.30% vs
66.34%). After fixing the actual opening lead, DD is ahead everywhere (67.79% vs 64.35%; +0.16
tricks). Conclusion: for expert play, DD is a slightly **pessimistic** proxy from the start, mainly
because DD defenders always find the best lead. For our programme, a DD-trained evaluator should
not be "corrected downward" by a blanket 0.x tricks. The sign depends on the stage and on whether
the lead is fixed.

### 5.8 LoTT accuracy [RAW for Ginsberg]
Ginsberg (446,741 DD deals): P(exact) 0.400; P(off by ±1) 0.245 + 0.224, so **86.9% within one
trick**. Mean signed error −0.05; mean |error| 0.75. By total trumps, mean error: 14→−0.15,
16→+0.10, 18→−0.01, 19→−0.22, 20→−0.50, 21→−0.89, 22→−1.31, 24→−2.22. Decision accuracy about 67–70%.
Pavlicek 8J69 (10.5M deals) confirms that average tricks match trumps for 14–19 and fall short
from 20 up. Wirgren 2002 (35–40% exact) agrees with Ginsberg's 40%.

### 5.9 Others
- helene_t logistic regression (198,382 deals): trump honours worth more than side honours
  (A 3.66 vs 3.42; J 0.87 vs 0.53). Doubleton side honours are discounted (Q 0.68, J 0.25).
  Secondary-suit A and K are worth less (A 2.96, K 1.03). Shortness with a 4-4 fit: 2.99/1.88/0.71.
  Her conclusion: mLTC "overvalues shortness drastically".
- Streib TSP table (2004): mean trick error HCP 1.23 → HCP+321 1.07 → Zar 1.05 → BUMRAP+531
  1.02 = TSP 1.02 → Binky 0.99. Setup undisclosed.
- Zhang (~2015): low-quality single-hand correlations (dist pts 0.29, controls 0.08, losers −0.2).
- Kidd (2015): DD frequencies of partial / game / slam / grand: 39.81 / 46.59 / 10.65 / 2.94%.

---

## 6. Gaps and sources not accessed

1. **The Bridge World, Oct 1982, pp.21–23 (Kaplan & Rubens, "4 C's")**: not accessed (print and
   paywall). This is the single most important gap. Three codifications conflict (§3.3.4), and
   Pavlicek's own mean does not reproduce (§3.3.5).
2. **Jeff Goldsmith K&R calculator**: the live domain was hijacked on 2026-11-17. The 2017
   Wayback capture shows the form only; the CGI source is unavailable.
3. **Koelman, NLTC, BW May 2003**: not accessed. Definition taken from Wikipedia and Deal code.
4. Books not accessed: Courtenay 1934/1935; Klinger *Modern LTC* (2011) and *Guide to Better
   Acol* (SQT, 1994); Bergen *Points Schmoints!* (2002); Cohen *To Bid or Not to Bid* (1992);
   Lawrence & Wirgren *I Fought the Law of Total Tricks* (2004); Petkov *Zar Points* (2003,
   2005); Diamond *Mastering Hand Evaluation* (2015); Crowhurst & Kambites (1992).
5. Vernes, "The Law of Total Tricks", BW (1969): not accessed.
6. **Matthew Kidd LoTT / evaluator study**: not located (S9, S16). His site has DD level
   frequencies only.
7. Wirgren's web paper (newbridgelaw.com): the domain is dead. The archived frame
   (→ web.telia.com/~u40906560/) was not followed.
8. Gordon Bower (taigabridge) DD research: not reached.
9. hp.vector.co.jp (Japanese LoTT corrections; Ginsberg library notes): connection refused.
   The claims seen only in snippets ("double-double fit occurs 12.0%"; "+0.92 / 0.87") are
   **unverified**.
10. BBO forum 31264 (dealer K-R script, HTTP 500); BBO 66910 (DD vs actual, refused).
11. **Origin of BUM-RAP**: not found. "Vienna" point count: not verified.
12. Ginsberg JAIR 2001; Rong/Qin/An 2019; Yeh & Lin 2016/2018; Winograd-Cort 2008;
    Dharmalingam & Amalraj 2014; Mossakowski & Mańdziuk ICAISC 2004/2006: abstracts or snippets
    only (MM09 full text read).
13. Andrews: the Single-Hand-Evaluator holdings and pattern tables (old `holdings.txt`,
    `patterns.txt`) now return 404. The 2008 Binky tables survive in Deal `lib/binky-data.tcl`.
    `2nt-3nt.html`, `stiff-queen.html` and `misunderstandings.html` were not fetched.
14. Pavlicek's 4♠ / 5♦ / 6♥ / 7♣ verification pages (ac02–ac05) and his full K-R frequency table:
    not fetched in full.
15. **No academic study found that compares K-R, Zar, LTC/NLTC and HCP head-to-head on the same
    DD dataset with a common metric, separately for NT and suits.** Andrews (correlations,
    partial run) and Streib (undisclosed) come closest. Our research programme can fill this gap.
16. Prior art for "entries", "self-sufficient suits / partner dependence" and explicit vector
    evaluators is thin. Only Andrews' Binky pattern×holding decomposition, Andrews' after-1♠
    fit-revaluation draft, helene_t's regression and the MM09 suit-length inputs address these
    quantitatively. Honour concentration appears as heuristics only: Zar's concentration point,
    the Rule of 20 proviso, the K-R ×L/10 multiplier and TSP's ≥2-honour bonus. These run against
    Andrews' DD finding that cards in long suits are worth slightly *less* (C8, C9).
