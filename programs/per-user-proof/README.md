# Per User Proof

## Notes

- Hash function: sha256
  - Using patched crate to improve performance: https://docs.succinct.xyz/writing-programs/patched-crates.html
- Shuffle: Fisher-Yates

## Performance

```
cargo run --release -- --execute --num-participants 1000 --num-winners 10
```

Using `raffle_naive`:

| Participants | Winners | Cycle Count |
| ------------ | ------- | ----------- |
| 100          | 10      | 62338       |
| 1000         | 10      | 62338       |
| 1000         | 100     | 512989      |
| 10000        | 10      | 62338       |
| 10000        | 100     | 513868      |
| 100000       | 10      | 63896       |
| 100000       | 100     | 530818      |
| 100000       | 1000    | 7412270     |
| 100000       | 10000   | 284134198   |
| 1000000      | 10      | 64246       |
| 1000000      | 100     | 534856      |
| 1000000      | 1000    | 7463342     |
| 1000000      | 10000   | 298288259   |
