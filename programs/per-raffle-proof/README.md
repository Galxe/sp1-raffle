# Per Raffle Proof

## Performance

```
cargo run --release -- --execute --num-participants 1000 --num-winners 10
```

Data below assumes random seed of `12345`, using sha2-v0-9-8 hash function:

| Participants | Winners | Cycle Count (Naive shuffle) | Cycle Count (Modified Fisher-Yates) |
| ------------ | ------- | --------------------------- | ----------------------------------- |
| 1000         | 10      | 62199                       | 54458                               |
| 1000         | 100     | 413718                      | 374117                              |
| 100000       | 10      | 63757                       | 551277                              |
| 100000       | 100     | 426828                      | 886440                              |
| 100000       | 99999   | 275900427                   | 249677765                           |
| 10000000     | 10      | 64417                       | 50051915                            |
| 10000000     | 100     | 434213                      | 50394012                            |
| 10000000     | 1000    | 4102868                     | 53787792                            |
| 10000000     | 10000   | 40953495                    | 87887844                            |
| 10000000     | 100000  | 409503565                   | 427893269                           |

### Hash algorithm

10000000 participants, 1000 winners

| sha2-v0-9-8 (sha256) | sha2-v0-10-8 (sha256) | sha3-v0-10-8 (keccak256) | tiny-keccak (keccak256) |
| -------------------- | --------------------- | ------------------------ | ----------------------- |
| 4100088              | 7523082               | 20156008                 | 20378001                |

We choose sha2-v0-9-8 for its performance.
