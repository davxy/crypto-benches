# Hash Benches

## SHA-256

Comparison between:
- [rust-crypto](https://crates.io/crates/sha2)
- [ring](https://crates.io/crates/ring)
- [cry-rs](https://github.com/davxy/cry)

### Hash

```
* rust-crypto   time:   [1.9360 µs 1.9366 µs 1.9372 µs]
* ring          time:   [8.0719 µs 8.0739 µs 8.0759 µs]
* cry-rs        time:   [17.019 µs 17.039 µs 17.069 µs]
```

## Blake2

Comparison between:
- [rust-crypto](https://crates.io/crates/sha2)
- [blake2-rfc](https://crates.io/crates/blake2-rfc)

### Hash 256

```
* rust-crypto  time:   [11.061 µs 11.065 µs 11.070 µs]
* blake2-rfc   time:   [11.121 µs 11.124 µs 11.128 µs]
```

### Hash 512

```
* rust-crypto  time:   [11.084 µs 11.089 µs 11.094 µs]
* blake2-rfc   time:   [11.099 µs 11.103 µs 11.108 µs]
```
