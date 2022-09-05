# Misc Benches

## Base64 codecs

Comparison between:
- [base64](https://crates.io/crates/base64)
- [cry-rs](https://github.com/davxy/cry)
- [smile](https://github.com/davxy/smile)

### Encode

```
* base64    time:   [48.951 ns 49.155 ns 49.375 ns]
* cry-rs    time:   [52.054 ns 52.077 ns 52.099 ns]
* smile     time:   [54.771 ns 54.825 ns 54.876 ns]
```

### Decode

```
* base64    time:   [50.621 ns 50.704 ns 50.802 ns]
* cry-rs    time:   [42.769 ns 42.793 ns 42.821 ns]
* smile     time:   [110.81 ns 111.34 ns 111.85 ns]
```
