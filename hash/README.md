# Hash Benches

Incremental hash of three 4K buffers.

## SHA-256

Comparison between:
- [rust-crypto](https://crates.io/crates/sha2) (0.10.8)
- [ring](https://crates.io/crates/ring) (0.17.7)
- [cry-rs](https://github.com/davxy/cry)

### Hash

```
* rust-crypto   time:   [5.6890 µs 5.6903 µs 5.6917 µs]
* ring          time:   [23.813 µs 23.865 µs 23.922 µs]
* cry-rs        time:   [49.442 µs 49.454 µs 49.469 µs]
```

## Blake2

Comparison between:
- [rust-crypto](https://crates.io/crates/blake2) (0.10.6)
- [blake2-rfc](https://crates.io/crates/blake2-rfc) (0.2.18)

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

## Blake3

Comparison between:
- [blake3](https://crates.io/crates/blake3) (1.5.0)

### Hash 256

```
* blake3       time:   [4.0565 µs 4.0585 µs 4.0602 µs]
```

### Hash 512

```
* blake3       time:   [4.0572 µs 4.0611 µs 4.0659 µs]
```

