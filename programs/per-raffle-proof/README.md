# Per Raffle Proof

## Performance

```
cargo run --release -- --execute --num-participants 1000 --num-winners 10
```

Data below assumes random seed of `12345`

| Participants | Winners | Cycle Count (sha2-v0-9-8) |
| ------------ | ------- | ------------------------- |
| 100          | 10      | 62403                     |
| 1000         | 10      | 62389                     |
| 1000         | 100     | 413656                    |
| 10000        | 10      | 62372                     |
| 10000        | 100     | 409818                    |
| 100000       | 10      | 63947                     |
| 100000       | 100     | 426748                    |
| 10000000     | 10      | 64607                     |
| 10000000     | 100     | 434133                    |
| 10000000     | 1000    | 4100088                   |
| 10000000     | 9999000 | 4100895                   |
| 10000000     | 10000   | 40923709                  |
| 10000000     | 100000  | 409203282                 |

### Hash algorithm

10000000 participants, 1000 winners

| sha2-v0-9-8 (sha256) | sha2-v0-10-8 (sha256) | sha3-v0-10-8 (keccak256) | tiny-keccak (keccak256) |
| -------------------- | --------------------- | ------------------------ | ----------------------- |
| 4100088              | 7523082               | 20156008                 | 20378001                |

### Shuffle algorithm

NOTE: this was run before changes to the raffle lib, where we use loser merkle root when # winners > half of participants

10000000 participants, 1000 winners

both with sha2_v0_10_8 hash function

| # participants | # winners | naive    | fisher-yates |
| -------------- | --------- | -------- | ------------ |
| 100            | 100       | 1929695  | 708070       |
| 1000           | 1000      | 55368412 | 6958918      |
| 1000           | 750       | 10196994 | 6345143      |
| 1000           | 600       | 6562402  | 5977268      |
| 1000           | 500       | 4978739  | 5716230      |
| 1000           | 250       | 2030330  | 5094563      |
| 1000           | 100       | 751887   | 4722970      |

when # winners > 60%, fisher-yates is much fasterw
