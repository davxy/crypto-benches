# Crypto-Benches

Cryptographic libraries benchmarks.

Benchmarking has been performed using [criterion](https://crates.io/crates/criterion)
on a *Intel i7-1185G7* machine running Linux.

## ECDSA Secp256k1

Comparison between:
- [k256](https://crates.io/crates/k256)
- [libsecp256k1](https://crates.io/crates/libsecp256k1)
- [secp256k1](https://crates.io/crates/secp256k1)

### Sign Prehashed

```
k256/sign               time:   [98.505 us 100.47 us 102.88 us]
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe

libsecp256k1/sign       time:   [131.23 us 133.43 us 136.18 us]
Found 19 outliers among 100 measurements (19.00%)
  4 (4.00%) high mild
  15 (15.00%) high severe

secp256k1/sign          time:   [40.374 us 40.890 us 41.581 us]
Found 13 outliers among 100 measurements (13.00%)
  4 (4.00%) high mild
  9 (9.00%) high severe
```

### Verify Prehashed

```
k256/verify             time:   [128.04 us 130.47 us 133.37 us]
Found 13 outliers among 100 measurements (13.00%)
  1 (1.00%) high mild
  12 (12.00%) high severe

libsecp256k1/verify     time:   [185.65 us 186.62 us 187.99 us]
Found 13 outliers among 100 measurements (13.00%)
  6 (6.00%) high mild
  7 (7.00%) high severe

secp256k1/verify        time:   [42.726 us 43.454 us 44.534 us]
Found 10 outliers among 100 measurements (10.00%)
  3 (3.00%) high mild
  7 (7.00%) high severe
```
