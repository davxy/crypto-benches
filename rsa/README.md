# RSA

Comparison between:
- [rust-crypto-rsa](https://crates.io/crates/rsa) (0.9.6)

### PKCS#1 v1.5 Sign

```
* rust-crypto   time:   [1.3856 ms 1.3867 ms 1.3883 ms]
```

### PKCS#1 v1.5 Verify

```
* rust-crypto   time:   [172.02 µs 172.07 µs 172.12 µs]
```

### PKCS#1 v1.5 Encrypt

```
* rust-crypto   time:   [172.29 µs 172.38 µs 172.48 µs]
```

### PKCS#1 v1.5 Decrypt

```
* rust-crypto   time:   [1.4260 ms 1.4280 ms 1.4307 ms]
```
