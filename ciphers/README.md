# Ciphers Benches

## AES

Comparison between:
- [rust-crypto-aes](https://crates.io/crates/aes) (0.8.4)
- [cry-rs](https://github.com/davxy/cry) (0.1.3)

### Encrypt

```
* rust-crypto   time:   [1.7451 µs 1.7455 µs 1.7460 µs]
* cry-rs        time:   [40.536 µs 40.609 µs 40.717 µs]
```

### Decrypt

```
* rust-crypto   time:   [1.8126 µs 1.8153 µs 1.8180 µs]
* cry-rs        time:   [43.092 µs 43.148 µs 43.226 µs]
```

## DES

Comparison between:
- [rust-crypto-aes](https://crates.io/crates/des) (0.8.1)
- [cry-rs](https://github.com/davxy/cry) (0.1.3)

### Encrypt

```
* rust-crypto   time:   [184.82 µs 184.92 µs 185.01 µs]
* cry-rs        time:   [2.5965 ms 2.5981 ms 2.5997 ms]  
```

### Decrypt

```
* rust-crypto   time:   [180.31 µs 180.40 µs 180.50 µs]
* cry-rs        time:   [2.6102 ms 2.6136 ms 2.6171 ms]
```
