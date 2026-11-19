system sayc: 1988804 train / 79294 val cases, 14 buckets
trump: 977966 train cases, val RMSE 1.1535 tricks (39s)
notrump: 1010838 train cases, val RMSE 1.4927 tricks (39s)

| Predictor (same information) | trump val RMSE | notrump val RMSE |
|---|---|---|
| linear HCP: captain + E[partner | range] | 1.2414 | 1.5140 |
| linear Fermat (fit / NT count): captain + E[partner | range] | 1.1860 | 1.5065 |
| auction-aware neural (H-52) | 1.1535 | 1.4927 |
