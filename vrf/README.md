# VRFs Benches

Comparison between:

- [ark-ec-vrfs](https://github.com/davxy/ark-ec-vrfs) (0.1.0, rev=c924bf6)
- [schnorrkel](https://crates.io/crates/schnorrkel) (0.11.4)
- [bandersnatch-vrfs](https://github.com/w3f/ring-vrfs/bandersnatch_vrfs) (0.0.4, rev=1a7aa56)

## VRF Prove

```
prove/schnorrkel                           time:   [101.11 µs 101.15 µs 101.21 µs]
prove/ark-ec-vrf-ed25519                   time:   [175.70 µs 175.82 µs 175.96 µs]
prove/ark-ec-vrf-bandersnatch-sha512-ed    time:   [187.79 µs 187.89 µs 187.98 µs]
prove/ark-ec-vrf-bandersnatch-blake2-ed    time:   [225.30 µs 225.39 µs 225.49 µs]
prove/ark-ec-vrf-bandersnatch-sha512-ws    time:   [230.34 µs 230.46 µs 230.58 µs]
prove/bandersnatch-vrfs                    time:   [380.76 µs 380.96 µs 381.20 µs]
```

## VRF Verify

```
verify/schnorrkel                          time:   [89.168 µs 89.204 µs 89.245 µs]
verify/ark-ec-vrf-ed25519                  time:   [268.86 µs 269.21 µs 269.65 µs]
verify/ark-ec-vrf-bandersnatch-sha512-ed   time:   [389.57 µs 389.72 µs 389.90 µs]
verify/ark-ec-vrf-bandersnatch-blake2-ed   time:   [483.51 µs 483.68 µs 483.85 µs]
verify/ark-ec-vrf_bandersnatch-sha512-ws   time:   [492.60 µs 493.32 µs 494.16 µs]
verify/bandersnatch-vrfs                   time:   [503.94 µs 504.12 µs 504.32 µs]

```


# Ring-VRFs Benches (ring size: 1023; domain size: 2048)

- [ark-ec-vrfs](https://github.com/davxy/ark-ec-vrfs) (0.1.0, rev=c924bf6)  --- TODO CHANGE REV
- [bandersnatch-vrfs](https://github.com/w3f/ring-vrfs/bandersnatch_vrfs) (0.0.4, rev=1a7aa56)

Both using [ring-proof](https://github.com/w3f/ring-proof) backend for zk-SNARK.


## Prover Key Construction

**Serial**
```
make-prover-key/w3f_bandersnatch-vrfs          time:   [138.00 ms 139.33 ms 140.69 ms]
make-prover-key/ark-ec-vrfs-bandersnatch-ed    time:   [140.95 ms 142.32 ms 143.68 ms]
make-prover-key/ark-ec-vrfs-bandersnatch-ws    time:   [161.88 ms 163.43 ms 164.97 ms] 
```

**Parallel**
```
make-prover-key/w3f-bandersnatch-vrfs          time:   [26.152 ms 26.433 ms 26.713 ms]
make-prover-key/ark-ec-vrfs-bandersnatch-ws    time:   [26.372 ms 26.597 ms 26.832 ms]
make-prover-key/ark-ec-vrfs-bandersnatch-ed    time:   [26.418 ms 26.656 ms 26.897 ms]
```

## Prover Construction

**Serial**
```
make-prover/w3f-bandersnatch-vrfs              time:   [133.75 ms 133.82 ms 133.90 ms]
make-prover/ark-ec-vrfs-bandersnatch-ed        time:   [137.80 ms 139.13 ms 140.50 ms]
make-prover/ark-ec-vrfs-bandersnatch-ws        time:   [165.96 ms 167.63 ms 169.35 ms]
```

**Parallel**
```
make-prover/ark-ec-vrfs-bandersnatch-ws        time:   [24.569 ms 24.739 ms 24.915 ms]
make-prover/ark-ec-vrfs-bandersnatch-ed        time:   [24.702 ms 24.852 ms 25.006 ms]
make-prover/w3f-bandersnatch-vrfs              time:   [26.878 ms 27.128 ms 27.385 ms]
```

## Prove

**Serial**
```
prove/ark-ec-vrfs-bandersnatch-ed              time:   [491.00 ms 495.38 ms 499.68 ms]
prove/ark-ec-vrfs-bandersnatch-ws              time:   [491.60 ms 496.20 ms 500.79 ms]
prove/w3f-bandersnatch-vrfs                    time:   [495.93 ms 500.29 ms 504.69 ms]
```

**Parallel**
```
prove/ark-ec-vrfs-bandersnatch-ed              time:   [113.88 ms 114.92 ms 116.03 ms]
prove/ark-ec-vrfs-bandersnatch-ws              time:   [115.02 ms 115.90 ms 116.76 ms]
prove/w3f-bandersnatch-vrfs                    time:   [116.87 ms 117.93 ms 119.02 ms]
```

## Verifier Key Construction

**Serial**
```
make-verifier-key/w3f-bandersnatch-vrfs        time:   [138.97 ms 140.32 ms 141.70 ms]
make-verifier-key/ark-ec-vrfs-bandersnatch-ed  time:   [138.98 ms 140.33 ms 141.72 ms]
make-verifier-key/ark-ec-vrfs-bandersnatch-ws  time:   [161.65 ms 163.20 ms 164.72 ms]
```

**Parallel**
```
make-verifier-key/ark-ec-vrfs-bandersnatch-ed  time:   [26.169 ms 26.371 ms 26.580 ms]
make-verifier-key/ark-ec-vrfs-bandersnatch-ws  time:   [26.425 ms 26.605 ms 26.784 ms]
make-verifier-key/w3f-bandersnatch-vrfs        time:   [26.413 ms 26.639 ms 26.868 ms]
```

## Verifier Construction

**Serial**
```
make-verifier/ark-ec-vrfs-bandersnatch-ed      time:   [297.97 µs 301.25 µs 304.06 µs]
make-verifier/ark-ec-vrfs-bandersnatch-ws      time:   [293.90 µs 297.63 µs 301.58 µs]
make-verifier/w3f-bandersnatch-vrfs            time:   [300.87 µs 304.68 µs 308.60 µs]
```

**Parallel**
```
make-verifier/ark-ec-vrfs-bandersnatch-ws      time:   [278.11 µs 278.22 µs 278.33 µs]
make-verifier/ark-ec-vrfs-bandersnatch-ed      time:   [278.13 µs 278.46 µs 278.94 µs]
make-verifier/w3f-bandersnatch-vrfs            time:   [282.96 µs 284.43 µs 286.85 µs]
```

## Verify

**Serial**
```
verify/ark-ec-vrfs-bandersnatch-ed             time:   [4.1419 ms 4.1448 ms 4.1483 ms]
verify/ark-ec-vrfs-bandersnatch-ws             time:   [4.5241 ms 4.5615 ms 4.5960 ms]
verify/w3f-bandersnatch-vrfs                   time:   [4.4622 ms 4.4978 ms 4.5315 ms]
```

**Parallel**
```
verify/ark-ec-vrfs-bandersnatch-ed             time:   [4.0692 ms 4.0727 ms 4.0771 ms]
verify/ark-ec-vrfs-bandersnatch-ws             time:   [4.2059 ms 4.2074 ms 4.2090 ms] 
verify/w3f-bandersnatch-vrfs                   time:   [4.3726 ms 4.3762 ms 4.3802 ms]
```
