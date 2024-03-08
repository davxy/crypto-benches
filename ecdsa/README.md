# ECDSA Benches

## Curve secp-256k1

Comparison between:
- [rust-crypto-k256](https://crates.io/crates/k256) (0.13.3)
- [libsecp256k1](https://crates.io/crates/libsecp256k1) (0.7.1)
- [secp256k1](https://crates.io/crates/secp256k1) (0.28.2)

### Sign Pre-Hashed

```
* secp256k1     time:   [27.813 µs 27.829 µs 27.846 µs]
* rust-crypto   time:   [41.314 µs 41.320 µs 41.329 µs]
* libsecp256k1  time:   [95.844 µs 95.853 µs 95.864 µs]
```

### Verify Pre-Hashed

```
* secp256k1       time: [31.669 µs 31.697 µs 31.747 µs]
* rust-crypto     time: [86.744 µs 86.786 µs 86.827 µs]
* libsecp256k1    time: [137.86 µs 137.93 µs 137.99 µs]
```

### Verify Pre-Hashed Recoverable

```
* secp256k1       time: [32.765 µs 32.790 µs 32.816 µs]
* rust-crypto     time: [193.63 µs 193.83 µs 194.15 µs]
```
