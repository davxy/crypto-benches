# Ciphers Benches

## AES

Comparison between:
- [rust-crypto-aes](https://crates.io/crates/aes)
- [cry-rs](https://github.com/davxy/cry)

### Encrypt

```
* rust-crypto   time:   [1.7700 µs 1.7702 µs 1.7703 µs]
* cry-rs        time:   [41.297 µs 41.310 µs 41.325 µs]
```

### Decrypt

```
* rust-crypto   time:   [1.8068 µs 1.8074 µs 1.8081 µs]
* cry-rs        time:   [41.120 µs 41.132 µs 41.143 µs]
```
