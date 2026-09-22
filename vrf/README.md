# VRFs Benches

Comparison between:

- [ark-vrf](https://github.com/davxy/ark-vrf) (main, commit `2b752e6`)
- [schnorrkel](https://crates.io/crates/schnorrkel) (0.11.5)

The bench profile builds the crate in one codegen unit. With the default
partitioning a result depends on the other code in the same bench binary.

Serial results use the `full` and `asm` features. Parallel results add
`parallel`.

A single VRF operation is too small for `parallel` to pay off. The rayon
overhead dominates, and verify costs about four times as much. schnorrkel does
not use these features. Its two rows show the spread between the two runs.

## VRF Prove

**Serial**
```
prove/schnorrkel                       time:   [90.601 µs 90.906 µs 91.272 µs]
prove/ark-vrf-ed25519                  time:   [111.11 µs 112.54 µs 113.92 µs]
prove/ark-vrf-bandersnatch-sha512-ed   time:   [119.14 µs 120.44 µs 121.69 µs]
prove/ark-vrf-bandersnatch-sha512-ws   time:   [158.27 µs 158.33 µs 158.40 µs]
prove/ark-vrf-bandersnatch-blake2-ed   time:   [161.86 µs 163.85 µs 165.68 µs]
```

**Parallel**
```
prove/schnorrkel                       time:   [92.932 µs 93.894 µs 94.875 µs]
prove/ark-vrf-ed25519                  time:   [107.03 µs 107.45 µs 108.01 µs]
prove/ark-vrf-bandersnatch-sha512-ed   time:   [115.00 µs 116.47 µs 117.87 µs]
prove/ark-vrf-bandersnatch-blake2-ed   time:   [154.59 µs 155.07 µs 155.74 µs]
prove/ark-vrf-bandersnatch-sha512-ws   time:   [156.99 µs 157.21 µs 157.50 µs]
```

## VRF Verify

**Serial**
```
verify/schnorrkel                       time:   [87.066 µs 87.794 µs 88.667 µs]
verify/ark-vrf-ed25519                  time:   [106.84 µs 107.63 µs 108.60 µs]
verify/ark-vrf-bandersnatch-sha512-ed   time:   [112.74 µs 113.02 µs 113.38 µs]
verify/ark-vrf-bandersnatch-blake2-ed   time:   [136.14 µs 137.12 µs 138.54 µs]
verify/ark-vrf-bandersnatch-sha512-ws   time:   [142.28 µs 143.78 µs 145.22 µs]
```

**Parallel**
```
verify/schnorrkel                       time:   [82.594 µs 82.618 µs 82.645 µs]
verify/ark-vrf-ed25519                  time:   [435.63 µs 443.67 µs 451.20 µs]
verify/ark-vrf-bandersnatch-sha512-ed   time:   [445.34 µs 452.40 µs 459.37 µs]
verify/ark-vrf-bandersnatch-blake2-ed   time:   [478.93 µs 485.65 µs 492.26 µs]
verify/ark-vrf-bandersnatch-sha512-ws   time:   [488.04 µs 494.79 µs 501.38 µs]
```

# Ring-VRFs Benches (ring size: 1023; domain size: 2048)

- [ark-vrf](https://github.com/davxy/ark-vrf) (main, commit `2b752e6`)

Serial results use the `full` and `asm` features. Parallel results add
`parallel`.

`parallel` gives a large gain on the key and proof construction. It still
costs time on the small operations.

## Params Deserialization

**Serial**
```
deserialize-params/ark-vrf-bandersnatch-ed-uncompressed   time:   [4.4255 ms 4.4605 ms 4.4969 ms]
deserialize-params/ark-vrf-bandersnatch-ed-compressed     time:   [149.53 ms 150.80 ms 152.09 ms]
```

**Parallel**
```
deserialize-params/ark-vrf-bandersnatch-ed-uncompressed   time:   [7.9348 ms 8.0122 ms 8.0912 ms]
deserialize-params/ark-vrf-bandersnatch-ed-compressed     time:   [162.98 ms 163.14 ms 163.31 ms]
```

## Prover Key Construction

**Serial**
```
make-prover-key/ark-vrf-bandersnatch-ed   time:   [105.91 ms 106.58 ms 107.29 ms]
make-prover-key/ark-vrf-bandersnatch-ws   time:   [124.31 ms 125.03 ms 125.84 ms]
```

**Parallel**
```
make-prover-key/ark-vrf-bandersnatch-ed   time:   [29.820 ms 30.008 ms 30.202 ms]
make-prover-key/ark-vrf-bandersnatch-ws   time:   [31.629 ms 31.819 ms 32.013 ms]
```

## Prover Construction

**Serial**
```
make-prover/ark-vrf-bandersnatch-ed   time:   [106.24 ms 106.94 ms 107.70 ms]
make-prover/ark-vrf-bandersnatch-ws   time:   [125.58 ms 126.51 ms 127.48 ms]
```

**Parallel**
```
make-prover/ark-vrf-bandersnatch-ed   time:   [30.063 ms 30.226 ms 30.393 ms]
make-prover/ark-vrf-bandersnatch-ws   time:   [31.465 ms 31.649 ms 31.834 ms]
```

## Prove

**Serial**
```
prove/ark-vrf-bandersnatch-ws   time:   [379.90 ms 382.91 ms 385.95 ms]
prove/ark-vrf-bandersnatch-ed   time:   [382.70 ms 385.79 ms 388.93 ms]
```

**Parallel**
```
prove/ark-vrf-bandersnatch-ed   time:   [122.57 ms 123.05 ms 123.55 ms]
prove/ark-vrf-bandersnatch-ws   time:   [123.81 ms 124.40 ms 124.98 ms]
```

## Verifier Key Construction

**Serial**
```
make-verifier-key/ark-vrf-bandersnatch-ed   time:   [105.21 ms 105.40 ms 105.65 ms]
make-verifier-key/ark-vrf-bandersnatch-ws   time:   [124.54 ms 125.35 ms 126.20 ms]
```

**Parallel**
```
make-verifier-key/ark-vrf-bandersnatch-ed   time:   [30.577 ms 30.791 ms 31.009 ms]
make-verifier-key/ark-vrf-bandersnatch-ws   time:   [31.887 ms 32.077 ms 32.267 ms]
```

## Verifier Construction

**Serial**
```
make-verifier/ark-vrf-bandersnatch-ws   time:   [247.90 µs 248.94 µs 250.30 µs]
make-verifier/ark-vrf-bandersnatch-ed   time:   [255.53 µs 258.43 µs 261.26 µs]
```

**Parallel**
```
make-verifier/ark-vrf-bandersnatch-ws   time:   [252.52 µs 255.23 µs 257.85 µs]
make-verifier/ark-vrf-bandersnatch-ed   time:   [255.54 µs 257.52 µs 259.92 µs]
```

## Verifier Construction (ring context included)

**Serial**
```
make-verifier-and-context/ark-vrf-bandersnatch-ed   time:   [3.7667 ms 3.7685 ms 3.7704 ms]
make-verifier-and-context/ark-vrf-bandersnatch-ws   time:   [3.8085 ms 3.8154 ms 3.8258 ms]
```

**Parallel**
```
make-verifier-and-context/ark-vrf-bandersnatch-ws   time:   [7.1099 ms 7.1713 ms 7.2368 ms]
make-verifier-and-context/ark-vrf-bandersnatch-ed   time:   [7.1312 ms 7.1961 ms 7.2642 ms]
```

## Verify

**Serial**
```
verify/ark-vrf-bandersnatch-ed   time:   [2.9912 ms 3.0124 ms 3.0355 ms]
verify/ark-vrf-bandersnatch-ws   time:   [3.1171 ms 3.1448 ms 3.1724 ms]
```

**Parallel**
```
verify/ark-vrf-bandersnatch-ed   time:   [4.2084 ms 4.2203 ms 4.2322 ms]
verify/ark-vrf-bandersnatch-ws   time:   [4.2479 ms 4.2595 ms 4.2712 ms]
```

