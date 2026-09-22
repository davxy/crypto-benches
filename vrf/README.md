# VRFs Benches

Comparison between:

- [ark-vrf](https://github.com/davxy/ark-vrf) (main, commit `2b752e6`)
- [schnorrkel](https://crates.io/crates/schnorrkel) (0.11.5)

Serial results use the `full` and `asm` features. Parallel results add
`parallel`.

A single VRF operation is too small for `parallel` to pay off. The rayon
overhead dominates, and verify costs more than three times as much. schnorrkel
does not use these features. Its two rows show the spread between the two runs.

## VRF Prove

**Serial**
```
prove/schnorrkel                       time:   [106.25 µs 106.56 µs 106.82 µs]
prove/ark-vrf-ed25519                  time:   [116.37 µs 117.52 µs 118.60 µs]
prove/ark-vrf-bandersnatch-sha512-ed   time:   [127.10 µs 128.57 µs 129.92 µs]
prove/ark-vrf-bandersnatch-blake2-ed   time:   [160.41 µs 162.00 µs 164.14 µs]
prove/ark-vrf-bandersnatch-sha512-ws   time:   [165.81 µs 167.34 µs 169.24 µs]
```

**Parallel**
```
prove/schnorrkel                       time:   [102.88 µs 103.49 µs 104.24 µs]
prove/ark-vrf-ed25519                  time:   [113.21 µs 114.39 µs 115.50 µs]
prove/ark-vrf-bandersnatch-sha512-ed   time:   [129.05 µs 130.14 µs 131.15 µs]
prove/ark-vrf-bandersnatch-blake2-ed   time:   [157.31 µs 158.19 µs 159.34 µs]
prove/ark-vrf-bandersnatch-sha512-ws   time:   [166.49 µs 168.34 µs 170.04 µs]
```

## VRF Verify

**Serial**
```
verify/schnorrkel                       time:   [85.712 µs 86.107 µs 86.567 µs]
verify/ark-vrf-ed25519                  time:   [108.41 µs 108.83 µs 109.38 µs]
verify/ark-vrf-bandersnatch-sha512-ed   time:   [121.35 µs 122.83 µs 124.25 µs]
verify/ark-vrf-bandersnatch-sha512-ws   time:   [144.19 µs 145.85 µs 147.54 µs]
verify/ark-vrf-bandersnatch-blake2-ed   time:   [151.96 µs 152.93 µs 153.74 µs]
```

**Parallel**
```
verify/schnorrkel                       time:   [86.460 µs 87.415 µs 88.297 µs]
verify/ark-vrf-ed25519                  time:   [426.86 µs 433.08 µs 439.19 µs]
verify/ark-vrf-bandersnatch-sha512-ed   time:   [433.53 µs 439.03 µs 444.49 µs]
verify/ark-vrf-bandersnatch-sha512-ws   time:   [475.34 µs 480.72 µs 486.19 µs]
verify/ark-vrf-bandersnatch-blake2-ed   time:   [482.29 µs 488.22 µs 493.88 µs]
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
deserialize-params/ark-vrf-bandersnatch-ed-uncompressed   time:   [4.5922 ms 4.6331 ms 4.6743 ms]
deserialize-params/ark-vrf-bandersnatch-ed-compressed     time:   [149.86 ms 151.04 ms 152.27 ms]
```

**Parallel**
```
deserialize-params/ark-vrf-bandersnatch-ed-uncompressed   time:   [7.9993 ms 8.0491 ms 8.0999 ms]
deserialize-params/ark-vrf-bandersnatch-ed-compressed     time:   [164.85 ms 165.02 ms 165.20 ms]
```

## Prover Key Construction

**Serial**
```
make-prover-key/ark-vrf-bandersnatch-ed   time:   [132.05 ms 133.13 ms 134.21 ms]
make-prover-key/ark-vrf-bandersnatch-ws   time:   [148.15 ms 149.43 ms 150.73 ms]
```

**Parallel**
```
make-prover-key/ark-vrf-bandersnatch-ed   time:   [30.493 ms 30.675 ms 30.860 ms]
make-prover-key/ark-vrf-bandersnatch-ws   time:   [32.279 ms 32.501 ms 32.728 ms]
```

## Prover Construction

**Serial**
```
make-prover/ark-vrf-bandersnatch-ed   time:   [129.47 ms 130.70 ms 131.97 ms]
make-prover/ark-vrf-bandersnatch-ws   time:   [149.83 ms 151.07 ms 152.29 ms]
```

**Parallel**
```
make-prover/ark-vrf-bandersnatch-ed   time:   [30.857 ms 31.053 ms 31.255 ms]
make-prover/ark-vrf-bandersnatch-ws   time:   [32.420 ms 32.601 ms 32.778 ms]
```

## Prove

**Serial**
```
prove/ark-vrf-bandersnatch-ed   time:   [453.75 ms 457.21 ms 460.61 ms]
prove/ark-vrf-bandersnatch-ws   time:   [459.61 ms 463.09 ms 466.53 ms]
```

**Parallel**
```
prove/ark-vrf-bandersnatch-ed   time:   [124.87 ms 125.44 ms 126.04 ms]
prove/ark-vrf-bandersnatch-ws   time:   [125.57 ms 126.17 ms 126.77 ms]
```

## Verifier Key Construction

**Serial**
```
make-verifier-key/ark-vrf-bandersnatch-ed   time:   [128.91 ms 130.01 ms 131.12 ms]
make-verifier-key/ark-vrf-bandersnatch-ws   time:   [149.07 ms 150.32 ms 151.58 ms]
```

**Parallel**
```
make-verifier-key/ark-vrf-bandersnatch-ed   time:   [30.733 ms 30.891 ms 31.051 ms]
make-verifier-key/ark-vrf-bandersnatch-ws   time:   [32.282 ms 32.481 ms 32.688 ms]
```

## Verifier Construction

**Serial**
```
make-verifier/ark-vrf-bandersnatch-ed   time:   [259.96 µs 263.36 µs 266.80 µs]
make-verifier/ark-vrf-bandersnatch-ws   time:   [268.24 µs 271.20 µs 274.10 µs]
```

**Parallel**
```
make-verifier/ark-vrf-bandersnatch-ed   time:   [252.63 µs 252.80 µs 252.98 µs]
make-verifier/ark-vrf-bandersnatch-ws   time:   [258.54 µs 261.46 µs 264.50 µs]
```

## Verifier Construction (ring context included)

**Serial**
```
make-verifier-and-context/ark-vrf-bandersnatch-ed   time:   [3.9248 ms 3.9560 ms 3.9878 ms]
make-verifier-and-context/ark-vrf-bandersnatch-ws   time:   [3.9610 ms 3.9930 ms 4.0250 ms]
```

**Parallel**
```
make-verifier-and-context/ark-vrf-bandersnatch-ws   time:   [7.2745 ms 7.3182 ms 7.3637 ms]
make-verifier-and-context/ark-vrf-bandersnatch-ed   time:   [7.2703 ms 7.3218 ms 7.3768 ms]
```

## Verify

**Serial**
```
verify/ark-vrf-bandersnatch-ed   time:   [3.0345 ms 3.0593 ms 3.0848 ms]
verify/ark-vrf-bandersnatch-ws   time:   [3.1334 ms 3.1601 ms 3.1874 ms]
```

**Parallel**
```
verify/ark-vrf-bandersnatch-ed   time:   [4.1851 ms 4.1977 ms 4.2107 ms]
verify/ark-vrf-bandersnatch-ws   time:   [4.3220 ms 4.3352 ms 4.3486 ms]
```

