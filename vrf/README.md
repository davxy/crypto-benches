# VRFs Benches

Comparison between:

- [ark-vrf](https://crates.io/crates/ark-vrf) (0.5.3)
- [schnorrkel](https://crates.io/crates/schnorrkel) (0.11.5)

Serial results use the `full` and `asm` features. Parallel results add
`parallel`.

A single VRF operation is too small for `parallel` to pay off. The rayon
overhead dominates, and verify costs about three times more. schnorrkel does not
use these features. Its two rows show the spread between the two runs.

## VRF Prove

**Serial**
```
prove/schnorrkel                       time:   [104.94 µs 105.68 µs 106.40 µs]
prove/ark-vrf-bandersnatch-sha512-ed   time:   [162.76 µs 162.93 µs 163.10 µs]
prove/ark-vrf-ed25519                  time:   [163.26 µs 164.32 µs 165.33 µs]
prove/ark-vrf-bandersnatch-blake2-ed   time:   [233.57 µs 235.40 µs 237.04 µs]
prove/ark-vrf-bandersnatch-sha512-ws   time:   [233.81 µs 236.57 µs 239.17 µs]
```

**Parallel**
```
prove/schnorrkel                       time:   [99.648 µs 100.31 µs 101.12 µs]
prove/ark-vrf-ed25519                  time:   [198.69 µs 200.33 µs 202.33 µs]
prove/ark-vrf-bandersnatch-sha512-ed   time:   [218.04 µs 219.41 µs 220.74 µs]
prove/ark-vrf-bandersnatch-blake2-ed   time:   [279.08 µs 280.68 µs 282.18 µs]
prove/ark-vrf-bandersnatch-sha512-ws   time:   [281.49 µs 282.97 µs 284.42 µs]
```

## VRF Verify

**Serial**
```
verify/schnorrkel                       time:   [92.941 µs 93.271 µs 93.535 µs]
verify/ark-vrf-ed25519                  time:   [168.82 µs 170.68 µs 172.52 µs]
verify/ark-vrf-bandersnatch-sha512-ed   time:   [186.86 µs 187.84 µs 188.65 µs]
verify/ark-vrf-bandersnatch-blake2-ed   time:   [228.02 µs 228.78 µs 229.86 µs]
verify/ark-vrf-bandersnatch-sha512-ws   time:   [241.92 µs 244.48 µs 246.61 µs]
```

**Parallel**
```
verify/schnorrkel                       time:   [89.183 µs 89.913 µs 90.773 µs]
verify/ark-vrf-ed25519                  time:   [515.40 µs 523.02 µs 530.86 µs]
verify/ark-vrf-bandersnatch-sha512-ed   time:   [527.75 µs 535.35 µs 542.78 µs]
verify/ark-vrf-bandersnatch-sha512-ws   time:   [575.16 µs 582.21 µs 589.73 µs]
verify/ark-vrf-bandersnatch-blake2-ed   time:   [581.93 µs 588.56 µs 595.38 µs]
```

# Ring-VRFs Benches (ring size: 1023; domain size: 2048)

- [ark-vrf](https://github.com/davxy/ark-vrf) (0.5.3)

Serial results use the `full` and `asm` features. Parallel results add
`parallel`.

`parallel` gives a large gain on the key and proof construction. It still
costs time on the small operations.

## Params Deserialization

**Serial**
```
deserialize-params/ark-vrf-bandersnatch-ed-uncompressed   time:   [4.5747 ms 4.6128 ms 4.6506 ms]
deserialize-params/ark-vrf-bandersnatch-ed-compressed     time:   [150.56 ms 151.68 ms 152.84 ms]
```

**Parallel**
```
deserialize-params/ark-vrf-bandersnatch-ed-uncompressed   time:   [8.0738 ms 8.1546 ms 8.2425 ms]
deserialize-params/ark-vrf-bandersnatch-ed-compressed     time:   [186.66 ms 186.87 ms 187.08 ms]
```

## Prover Key Construction

**Serial**
```
make-prover-key/ark-vrf-bandersnatch-ed   time:   [110.51 ms 111.27 ms 112.09 ms]
make-prover-key/ark-vrf-bandersnatch-ws   time:   [134.02 ms 135.16 ms 136.30 ms]
```

**Parallel**
```
make-prover-key/ark-vrf-bandersnatch-ed   time:   [30.905 ms 31.104 ms 31.304 ms]
make-prover-key/ark-vrf-bandersnatch-ws   time:   [32.357 ms 32.553 ms 32.759 ms]
```

## Prover Construction

**Serial**
```
make-prover/ark-vrf-bandersnatch-ed   time:   [114.44 ms 115.36 ms 116.26 ms]
make-prover/ark-vrf-bandersnatch-ws   time:   [132.10 ms 133.20 ms 134.29 ms]
```

**Parallel**
```
make-prover/ark-vrf-bandersnatch-ed   time:   [31.158 ms 31.353 ms 31.551 ms]
make-prover/ark-vrf-bandersnatch-ws   time:   [32.364 ms 32.559 ms 32.756 ms]
```

## Prove

**Serial**
```
prove/ark-vrf-bandersnatch-ed   time:   [397.12 ms 400.16 ms 403.23 ms]
prove/ark-vrf-bandersnatch-ws   time:   [403.52 ms 406.34 ms 409.10 ms]
```

**Parallel**
```
prove/ark-vrf-bandersnatch-ed   time:   [125.31 ms 125.90 ms 126.49 ms]
prove/ark-vrf-bandersnatch-ws   time:   [125.70 ms 126.31 ms 126.92 ms]
```

## Verifier Key Construction

**Serial**
```
make-verifier-key/ark-vrf-bandersnatch-ed   time:   [116.43 ms 117.21 ms 117.95 ms]
make-verifier-key/ark-vrf-bandersnatch-ws   time:   [133.70 ms 134.77 ms 135.84 ms]
```

**Parallel**
```
make-verifier-key/ark-vrf-bandersnatch-ed   time:   [31.068 ms 31.252 ms 31.436 ms]
make-verifier-key/ark-vrf-bandersnatch-ws   time:   [32.376 ms 32.602 ms 32.857 ms]
```

## Verifier Construction

**Serial**
```
make-verifier/ark-vrf-bandersnatch-ws   time:   [255.64 µs 258.78 µs 261.69 µs]
make-verifier/ark-vrf-bandersnatch-ed   time:   [261.56 µs 264.03 µs 266.27 µs]
```

**Parallel**
```
make-verifier/ark-vrf-bandersnatch-ed   time:   [258.12 µs 258.73 µs 259.56 µs]
make-verifier/ark-vrf-bandersnatch-ws   time:   [259.09 µs 260.57 µs 262.41 µs]
```

## Verifier Construction (ring context included)

**Serial**
```
make-verifier-and-context/ark-vrf-bandersnatch-ed   time:   [3.8969 ms 3.9312 ms 3.9661 ms]
make-verifier-and-context/ark-vrf-bandersnatch-ws   time:   [3.9802 ms 4.0129 ms 4.0449 ms]
```

**Parallel**
```
make-verifier-and-context/ark-vrf-bandersnatch-ed   time:   [7.1505 ms 7.2036 ms 7.2570 ms]
make-verifier-and-context/ark-vrf-bandersnatch-ws   time:   [7.3033 ms 7.3524 ms 7.4049 ms]
```

## Verify

**Serial**
```
verify/ark-vrf-bandersnatch-ed   time:   [3.0825 ms 3.1075 ms 3.1322 ms]
verify/ark-vrf-bandersnatch-ws   time:   [3.2190 ms 3.2422 ms 3.2638 ms]
```

**Parallel**
```
verify/ark-vrf-bandersnatch-ed   time:   [4.1511 ms 4.1633 ms 4.1755 ms]
verify/ark-vrf-bandersnatch-ws   time:   [4.2651 ms 4.2761 ms 4.2873 ms]
```

