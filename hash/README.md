# Hash Benches

## SHA-256

Comparison between:
- [rust-crypto](https://crates.io/crates/sha2)
- [ring](https://crates.io/crates/ring)
- [cry-rs](https://github.com/davxy/cry)

### Hash

```
* rust-crypto   time:   [1.9313 µs 1.9318 µs 1.9324 µs]
* ring          time:   [8.0144 µs 8.0166 µs 8.0189 µs]
* cry-rs        time:   [17.019 µs 17.039 µs 17.069 µs]
```
