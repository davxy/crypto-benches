# ECDSA Benches

## Curve secp-256k1

Comparison between:
- [rust-crypto-k256](https://crates.io/crates/k256)
- [libsecp256k1](https://crates.io/crates/libsecp256k1)
- [secp256k1](https://crates.io/crates/secp256k1)

### Sign Prehashed

```
* rust-crypto   time:   [74.929 µs 74.966 µs 75.011 µs]
* libsecp256k1  time:   [95.844 µs 95.853 µs 95.864 µs]
* secp256k1     time:   [27.813 µs 27.829 µs 27.846 µs]
```

### Verify Prehashed

```
* rust-crypto     time:   [99.389 µs 99.421 µs 99.459 µs]
* libsecp256k1    time:   [137.86 µs 137.93 µs 137.99 µs]
* secp256k1       time:   [31.669 µs 31.697 µs 31.747 µs]
```
