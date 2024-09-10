# Per Raffle Proof

## Notes

- Hash function: sha256
  - Using patched crate to improve performance: https://docs.succinct.xyz/writing-programs/patched-crates.html

## Performance

```
cargo run --release -- --execute --num-participants 1000 --num-winners 10
```

Data below assumes random seed of `12345`

| Participants | Winners | Cycle Count | Merkle Root                                                        |
| ------------ | ------- | ----------- | ------------------------------------------------------------------ |
| 100          | 10      | 51183       | 0xced48ce2a54393639311900331de7fa22ee1a0ed8410119aa3ee6c17733aeb75 |
| 1000         | 10      | 51183       | 0xbf449f1db6ee091b380b110ef54ff7112835222d91753c2ec8998971a2fd6c53 |
| 1000         | 100     | 389721      | 0xcf8d690ca84b2bae826470057e3b7585e5d3530f019e78c7fc6f262cf6869a73 |
| 10000        | 10      | 51183       | 0x06444ad88114b90ab4db3019d37c2955bbde6e15630d06e43074b346862cdac6 |
| 10000        | 100     | 395997      | 0x9971ec5531a3fad1e9c38c6a77a54d24bf6cafe950928608f8304e3a7fe0aa1c |
| 100000       | 10      | 52741       | 0x5d39e729f2250f4ef96561c7afbcd1bc0d03264bc634b5a9c52b07a6bddaf42d |
| 1000000      | 10      | 53091       | 0x9dd3660d5aa5552da30cb6f196bd1c21cf2b9e6676f0a97c7068cc7dcdc9543e |
| 10000000     | 10      | 53418       | 0xb72907e34b589edee7cdb4236efe59f2367d527a2bbb7c06104100fe02f1c0fc |
| 10000000     | 100     | 420673      | 0x76fe23ca034289be494fcccb15a4f91af2233150fc27b198b5ffa8d6aef26224 |
| 10000000     | 1000    | 6291455     | 0x62383a7facecc690676ae8f078d164a26f8c75c1f58d66eae2602740d500345d |
