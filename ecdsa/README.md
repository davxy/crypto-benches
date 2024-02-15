# ECDSA Benches

## Curve secp-256k1

Comparison between:
- [rust-crypto-k256](https://crates.io/crates/k256) (0.13.3)
- [libsecp256k1](https://crates.io/crates/libsecp256k1) (0.7.1)
- [secp256k1](https://crates.io/crates/secp256k1) (0.28.2)

### Sign Prehashed

```
* rust-crypto   time:   [41.314 µs 41.320 µs 41.329 µs]
* libsecp256k1  time:   [95.844 µs 95.853 µs 95.864 µs]
* secp256k1     time:   [27.813 µs 27.829 µs 27.846 µs]
```

### Verify Prehashed

```
* rust-crypto     time: [88.997 µs 89.014 µs 89.032 µs]    
* libsecp256k1    time: [137.86 µs 137.93 µs 137.99 µs]
* secp256k1       time: [31.669 µs 31.697 µs 31.747 µs]
```
