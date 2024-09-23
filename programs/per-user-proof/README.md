# Per User Proof

## Notes

- Hash function: sha256
  - Using patched crate to improve performance: https://docs.succinct.xyz/writing-programs/patched-crates.html
- Shuffle: Fisher-Yates

## Performance （cycle tracking)

```
cargo run --release -- --execute --num-participants 1000 --num-winners 10
```

Using `raffle_naive`:

| Participants | Winners | Cycle Count | Groth16 Proof Time   | Groth16 Prover Network URL                                            |
| ------------ | ------- | ----------- | -------------------- | --------------------------------------------------------------------- |
| 100          | 10      | 62326       | 2 minutes 38 seconds | https://explorer.succinct.xyz/proofrequest_01j8fd3288e4br5bkk8wfv8qad |
| 1000         | 10      | 62326       |                      |                                                                       |
| 1000         | 100     | 528562      |                      |                                                                       |
| 10000        | 10      | 62326       |                      |                                                                       |
| 10000        | 100     | 513766      |                      |                                                                       |
| 100000       | 10      | 63884       |                      |                                                                       |
| 100000       | 100     | 530716      |                      |                                                                       |
| 100000       | 1000    | 7440941     |                      |                                                                       |
| 100000       | 10000   | 310329795   |                      |                                                                       |
| 1000000      | 10      | 64234       |                      |                                                                       |
| 1000000      | 100     | 534754      |                      |                                                                       |
| 1000000      | 1000    | 7462340     |                      |                                                                       |
| 1000000      | 10000   | 300019549   | 5 minutes 59 seconds | https://explorer.succinct.xyz/proofrequest_01j8fde317e4btefd5q0bcdbq0 |
