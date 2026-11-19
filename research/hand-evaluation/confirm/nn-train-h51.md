cases: trump 364998 train / 14543 val; notrump 348334 train / 13885 val
trump: val RMSE 0.8609 tricks after 30 epochs (135s)
notrump: val RMSE 1.2757 tricks after 30 epochs (25s)

| Predictor | trump val RMSE | notrump val RMSE |
|---|---|---|
| linear HCP | 1.1374 | 1.3614 |
| linear Fermat (fit count / NT count) | 0.9983 | 1.3365 |
| linear Zar | 1.0171 | 1.7070 |
| neural (H-51) | 0.8609 | 1.2757 |
