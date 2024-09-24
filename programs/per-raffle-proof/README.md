# Per Raffle Proof

## Notes

- Hash function: sha256
  - Using patched crate to improve performance: https://docs.succinct.xyz/writing-programs/patched-crates.html

## Performance

```
cargo run --release -- --execute --num-participants 1000 --num-winners 10
```

Data below assumes random seed of `12345`

| Participants | Winners | Cycle Count (sha2-v0-9-8) | Merkle Root                                                        | Groth16 Proof Time   | Groth16 Prover Network URL                                            |
| ------------ | ------- | ------------------------- | ------------------------------------------------------------------ | -------------------- | --------------------------------------------------------------------- |
| 100          | 10      | 51216                     | 0xced48ce2a54393639311900331de7fa22ee1a0ed8410119aa3ee6c17733aeb75 | 2 minutes 56 seconds | https://explorer.succinct.xyz/proofrequest_01j8fbfz8ae2cahb2hv4p1dbdk |
| 1000         | 10      | 51216                     | 0xbf449f1db6ee091b380b110ef54ff7112835222d91753c2ec8998971a2fd6c53 |                      |                                                                       |
| 1000         | 100     | 399627                    | 0x3c78346b865e4b4ae9ac1fdcbc27a00e2d0289626643e235ee0ffbbc943742d3 |                      |                                                                       |
| 10000        | 10      | 51216                     | 0x06444ad88114b90ab4db3019d37c2955bbde6e15630d06e43074b346862cdac6 |                      |                                                                       |
| 10000        | 100     | 399627                    | 0x3c78346b865e4b4ae9ac1fdcbc27a00e2d0289626643e235ee0ffbbc943742d3 |                      |                                                                       |
| 100000       | 10      | 52741                     | 0x5d39e729f2250f4ef96561c7afbcd1bc0d03264bc634b5a9c52b07a6bddaf42d |                      |                                                                       |
| 100000       | 100     | 395895                    | 0x9971ec5531a3fad1e9c38c6a77a54d24bf6cafe950928608f8304e3a7fe0aa1c |                      |                                                                       |
| 10000000     | 10      | 53418                     | 0xb72907e34b589edee7cdb4236efe59f2367d527a2bbb7c06104100fe02f1c0fc |                      |                                                                       |
| 10000000     | 100     | 420571                    | 0x76fe23ca034289be494fcccb15a4f91af2233150fc27b198b5ffa8d6aef26224 | 2 minutes 46 seconds | https://explorer.succinct.xyz/proofrequest_01j8fbzpcjfezajrz9xhqfvccx |
| 10000000     | 1000    | 6289598                   | 0x62383a7facecc690676ae8f078d164a26f8c75c1f58d66eae2602740d500345d | 3 minutes 13 seconds | https://explorer.succinct.xyz/proofrequest_01j8fc6645e6abb6k7q5266qa7 |
| 10000000     | 10000   | 288003808                 | 0x647ee7b3d3e20c516d108670e48250d7fc1f76ebdc19fbf6ab86e91d25a63434 | 6 minutes 29 seconds | https://explorer.succinct.xyz/proofrequest_01j8fcd45ce6a930tvqngrqqst |

### Hash algorithm

10000000 participants, 1000 winners

| sha2-v0-9-8 (sha256) | sha2-v0-10-8 (sha256) | sha3-v0-10-8 (keccak256) | tiny-keccak (keccak256) |
| -------------------- | --------------------- | ------------------------ | ----------------------- |
| 6289598              | 9709598               | 22346206                 | 22566445                |

### Shuffle algorithm

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

when # winners > 60%, fisher-yates is much faster