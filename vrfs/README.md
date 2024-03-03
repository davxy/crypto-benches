# VRFs Benches

Comparison between:

- [schnorrkel](https://crates.io/crates/schnorrkel) (0.11.4)
- [bandersnatch-vrfs](https://github.com/w3f/ring-vrfs/bandersnatch_vrfs) (0.0.4)

## VRF Sign

```
sign/schnorrkel         time:   [98.966 µs 98.989 µs 99.018 µs]
sign/bandersnatch       time:   [378.06 µs 378.81 µs 379.63 µs]
```

## VRF Verify

```
verify/schnorrkel       time:   [85.270 µs 85.298 µs 85.325 µs]
verify/bandersnatch     time:   [486.43 µs 486.67 µs 486.95 µs]
```

## VRF Bytes

```
vrf-bytes/schnorrkel    time:   [1.1767 µs 1.1785 µs 1.1803 µs]
vrf-bytes/bandersnatch  time:   [4.9840 µs 4.9862 µs 4.9887 µs]
```

# Ring-VRFs Benches

- [bandersnatch-vrfs](https://github.com/w3f/ring-vrfs/bandersnatch_vrfs) (0.0.4)


## Domain Size 512

```
bandersnatch-ring-vrf-512/make-prover         time:   [43.005 ms 43.019 ms 43.038 ms]
bandersnatch-ring-vrf-512/sign                time:   [143.81 ms 144.08 ms 144.35 ms]

bandersnatch-ring-vrf-512/make-verifier       time:   [41.965 ms 41.973 ms 41.980 ms]
bandersnatch-ring-vrf-512/verify              time:   [4.3327 ms 4.3370 ms 4.3428 ms]
```

## Domain Size 512 (parallel)

```
bandersnatch-ring-vrf-512/make-prover-key     time:   [11.729 ms 11.853 ms 11.985 ms]
bandersnatch-ring-vrf-512/sign                time:   [46.214 ms 46.645 ms 47.100 ms]

bandersnatch-ring-vrf-512/make-verifier-key   time:   [11.486 ms 11.597 ms 11.706 ms]
bandersnatch-ring-vrf-512/verify              time:   [3.9925 ms 3.9928 ms 3.9931 ms]
```

## Domain Size 1024

```
bandersnatch-ring-vrf-1024/make-prover        time:   [72.902 ms 72.918 ms 72.933 ms]
bandersnatch-ring-vrf-1024/sign               time:   [253.69 ms 253.75 ms 253.80 ms]

bandersnatch-ring-vrf-1024/make-verifier      time:   [73.107 ms 73.113 ms 73.120 ms]
bandersnatch-ring-vrf-1024/verify             time:   [4.1118 ms 4.1123 ms 4.1127 ms]
```

## Domain Size 1024 (parallel)

```
bandersnatch-ring-vrf-1024/make-prover        time:   [19.494 ms 19.689 ms 19.888 ms]
bandersnatch-ring-vrf-1024/sign               time:   [86.406 ms 87.555 ms 88.703 ms]

bandersnatch-ring-vrf-1024/make-verifier      time:   [21.814 ms 21.980 ms 22.141 ms]
bandersnatch-ring-vrf-1024/verify             time:   [4.3917 ms 4.3933 ms 4.3955 ms]
```

## Domain Size 2048

```
bandersnatch-ring-vrf-2048/make-prover        time:   [132.16 ms 132.21 ms 132.30 ms]
bandersnatch-ring-vrf-2048/sign               time:   [456.75 ms 456.83 ms 456.91 ms]

bandersnatch-ring-vrf-2048/make-verifier      time:   [132.45 ms 132.49 ms 132.52 ms]
bandersnatch-ring-vrf-2048/verify             time:   [4.3897 ms 4.3920 ms 4.3958 ms]
```

## Domain Size 2048 (parallel)

```
bandersnatch-ring-vrf-2048/make-prover-key    time:   [36.308 ms 36.524 ms 36.736 ms]
bandersnatch-ring-vrf-2048/sign               time:   [153.68 ms 155.26 ms 156.80 ms]

bandersnatch-ring-vrf-2048/make-verifier      time:   [37.187 ms 37.391 ms 37.591 ms]
bandersnatch-ring-vrf-2048/verify             time:   [4.1161 ms 4.1166 ms 4.1172 ms]
```
