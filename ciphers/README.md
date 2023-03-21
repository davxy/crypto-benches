# Ciphers Benches

## AES

Comparison between:
- [rust-crypto-aes](https://crates.io/crates/aes) (0.8.2)
- [cry-rs](https://github.com/davxy/cry) (0.1.3)

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

## DES

Comparison between:
- [rust-crypto-aes](https://crates.io/crates/des) 0.8.1
- [cry-rs](https://github.com/davxy/cry) 0.1.3

### Encrypt

```
* rust-crypto   time:   [184.82 µs 184.92 µs 185.01 µs]
* cry-rs        time:   [2.5965 ms 2.5981 ms 2.5997 ms]  
```

### Decrypt

```
* rust-crypto   time:   [185.44 µs 186.00 µs 186.71 µs]
* cry-rs        time:   [2.6102 ms 2.6136 ms 2.6171 ms]
```
