# Per Raffle Proof

## Notes

- Hash function: sha256
  - Using patched crate to improve performance: https://docs.succinct.xyz/writing-programs/patched-crates.html

## Performance

```
cargo run --release -- --execute --num-participants 1000 --num-winners 10
```

Data below assumes random seed of `12345`

| Participants | Winners | Cycle Count | Merkle Root                                                        | Groth16 Proof Time | Groth16 Prover Network URL                                            |
| ------------ | ------- | ----------- | ------------------------------------------------------------------ | ------------------ | --------------------------------------------------------------------- |
| 100          | 10      | 51228       | 0xced48ce2a54393639311900331de7fa22ee1a0ed8410119aa3ee6c17733aeb75 | 2m43s              | https://explorer.succinct.xyz/proofrequest_01j8egy82degrts5fay3cn81wq |
| 1000         | 10      | 51228       | 0xbf449f1db6ee091b380b110ef54ff7112835222d91753c2ec8998971a2fd6c53 |                    |                                                                       |
| 1000         | 100     | 389721      | 0xcf8d690ca84b2bae826470057e3b7585e5d3530f019e78c7fc6f262cf6869a73 |                    |                                                                       |
| 10000        | 10      | 51228       | 0x06444ad88114b90ab4db3019d37c2955bbde6e15630d06e43074b346862cdac6 |                    |                                                                       |
| 10000        | 100     | 395997      | 0x9971ec5531a3fad1e9c38c6a77a54d24bf6cafe950928608f8304e3a7fe0aa1c |                    |                                                                       |
| 100000       | 10      | 52741       | 0x5d39e729f2250f4ef96561c7afbcd1bc0d03264bc634b5a9c52b07a6bddaf42d |                    |                                                                       |
| 100000       | 100     | 420673      | 0x76fe23ca034289be494fcccb15a4f91af2233150fc27b198b5ffa8d6aef26224 |                    |                                                                       |
| 10000000     | 10      | 53418       | 0xb72907e34b589edee7cdb4236efe59f2367d527a2bbb7c06104100fe02f1c0fc |                    |                                                                       |
| 10000000     | 100     | 420673      | 0x76fe23ca034289be494fcccb15a4f91af2233150fc27b198b5ffa8d6aef26224 | 2m46s              | https://explorer.succinct.xyz/proofrequest_01j8eh6905egrr635fymhr2ah7 |
| 10000000     | 1000    | 6290600     | 0x62383a7facecc690676ae8f078d164a26f8c75c1f58d66eae2602740d500345d | 3m6s               | https://explorer.succinct.xyz/proofrequest_01j8ema4nvegtt76dqvk0s7brp |
| 10000000     | 10000   | 287693424   | 0xa72920a1add9f3cdf91682d2fe3fa2a09011ed4ea0975f09c621bf3204dff789 | 6m22s              | https://explorer.succinct.xyz/proofrequest_01j8en6ngbegtvz4qvw4jvmvcy |

## Logs

100 participants, 10 winners（Groth16）

```
Verification Key: 0x007502a17bd054ca2d7bfdd7a0a9a7066b41d142eb76c6c2b296880305f8c7c4
Public Values: 0x0000000000000000000000000000000000000000000000000000000000000064000000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000000000000003039ced48ce2a54393639311900331de7fa22ee1a0ed8410119aa3ee6c17733aeb75
Proof Bytes: 0x6a2906ac092b7e497bcb047ad6e008a2f2b78e77b9a2732f282e897645fa339e29cae7750fc6ed66350f1ed84402ed0ed2a3c7104b896c44dcb93e6897eee6ae42f5f5572ad3ada13877fc89e0275d292c2a176d594220db67e075f6f851a34f50d96dbb16e6a7488579d42bbe8b88badb8a1e2a0bc7194e2f241540bb238928b62195ae2196b9a19d4a4279a8581ebf7a4706dbae9f69b941eab1e62d463f503981d8252022f9d4eee756e6fc1530d2cc1b097d0c715e325ca6e06a9723ca607c6ecbca15aff32d40288ddbdaf08141f76b795584d0dabffa6d563a807da42c951452992ac8cc2676a41c1afcfec6d95f0ec12d378581bff41f802335f6a8e6911cd0a0
```

10000000 participants, 1000 winners Groth16）

```
Verification Key: 0x007502a17bd054ca2d7bfdd7a0a9a7066b41d142eb76c6c2b296880305f8c7c4
Public Values: 0x000000000000000000000000000000000000000000000000000000000098968000000000000000000000000000000000000000000000000000000000000003e8000000000000000000000000000000000000000000000000000000000000303962383a7facecc690676ae8f078d164a26f8c75c1f58d66eae2602740d500345d
Proof Bytes: 0x6a2906ac2bdc11c34385d676b280cbaa646d584c2f65f88341b11405e12fb18a750db8aa0e2d0a73536b05d5a67fbac543df23ff862d1e61f8c4bb49acc2aa9ad5b4edee1ed8af0233d985f535aa9518f533d1a9ab06f1e1680fd12e567d495ea546c0fd0a67b50e220ec2c9576666b69084f1853c6ec0d64eda22e76a30a2cd111eb12e06dcc8dd1fc8cd8f533bb35092d0f59591c54e4ef100919b6e53bd824473b56f0f63cc18213c64404a224cf4b98f320e286ce024b9db9dc7dce3648fb04ced733056bcd3e70ed46e9417853fc41ff6e45be5a48d5ecb374e57eba8994b54926103846766c514836ac41b58d4dba5ae591e4ec8a6436c4e7c5ac536a5b80e998f
```
