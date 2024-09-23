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

| Participants | Winners | Cycle Count | Groth16 Proof Time | Groth16 Prover Network URL                                            |
| ------------ | ------- | ----------- | ------------------ | --------------------------------------------------------------------- |
| 100          | 10      | 62338       | 2m30s              | https://explorer.succinct.xyz/proofrequest_01j8eghd0wfpz8yy33ph1qxj15 |
| 1000         | 10      | 62338       |                    |                                                                       |
| 1000         | 100     | 512989      |                    |                                                                       |
| 10000        | 10      | 62338       |                    |                                                                       |
| 10000        | 100     | 513868      |                    |                                                                       |
| 100000       | 10      | 63896       |                    |                                                                       |
| 100000       | 100     | 530818      |                    |                                                                       |
| 100000       | 1000    | 7412270     |                    |                                                                       |
| 100000       | 10000   | 284134198   |                    |                                                                       |
| 1000000      | 10      | 64246       |                    |                                                                       |
| 1000000      | 100     | 534856      |                    |                                                                       |
| 1000000      | 1000    | 7463342     |                    |                                                                       |
| 1000000      | 10000   | 298288259   | 5m56s              | https://explorer.succinct.xyz/proofrequest_01j8emngr4egtsharwzjv3f8fm |

## Logs

100 participants, 10 winners （Groth16）

```
2024-09-23T04:02:00.006226Z  INFO Proof request fulfilled
Verification Key: 0x0045e7d83fdafd41a15012355b47c5e83cf232237dbacb8cc6d63da869e2f5a8
Public Values: 0x0000000000000000000000000000000000000000000000000000000000000064000000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000000303900000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000
Proof Bytes: 0x6a2906ac1bcf54c145c83d3b3849fc724d07a38035d0ba1e740dd20880ff6e97fe3feafd22e5b2f69a58dd8a3207a7099176711c57b753bb06a42cb28af1b86a9a71ac581cbdcc95f222014da8a7d969838caf760408c3d16c6c70a523831cb1237a14140ad30389e6e089de657e2c6855b590192b719ce20451e88c0cfde64e974253c62903eea65c5119d99d9ee41ca7105c73de472abb80c182943ae99a077a96e5e2116c39c14ccb14710c17c374caa7416627c99daf03e02f67c2028eab897a02ca1758f968d26d6426e8d1ed09d7297e74dca31cfb54ea592e91419e1059e83f4c0700aa813d250b6f2e2bd5b427974fc9e22906727c1967966c38f0125b7df5ec
```

1000000 participants, 10000 winners

```
Verification Key: 0x0045e7d83fdafd41a15012355b47c5e83cf232237dbacb8cc6d63da869e2f5a8
Public Values: 0x00000000000000000000000000000000000000000000000000000000000f42400000000000000000000000000000000000000000000000000000000000002710000000000000000000000000000000000000000000000000000000000000303900000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000
Proof Bytes: 0x6a2906ac0dfeea46ba844e22fa44f7edef156c390756272f705fd2c7df5747ba6c347211296ca2cf1540c3878f666aa5ac55481bb2a07f2b3b483dbf38079b23f11a63d90440863829090e0d0a76fe6039a1d63e321ef57677279001c256c9212479bb960e17dd625c798b58c73d80245d5acdbcd864f35c10029ef19f781c693c56448d074896dddc64961d04d155b7daecb01ffcd401c659da812e7f910a468e2f72f62b93abc6e80a6c74f9e02c0a5b170985b40c1924b41756e7c8e1b3fa233d17762e00175aecae51d17333016146806b6e
```
