system allfives: 1899718 train / 75854 val cases, 25 buckets
trump: 929100 train cases, val RMSE 1.1191 tricks (189s)
notrump: 970618 train cases, val RMSE 1.5237 tricks (39s)

| Predictor (same information) | trump val RMSE | notrump val RMSE |
|---|---|---|
| linear HCP: captain + E[partner | range] | 1.2253 | 1.5432 |
| linear Fermat (fit / NT count): captain + E[partner | range] | 1.1536 | 1.5339 |
| auction-aware neural (H-52) | 1.1191 | 1.5237 |
