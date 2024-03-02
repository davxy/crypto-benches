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

## Proving

### Ring Size 512

```
bandersnatch-ring-vrf-512/make-prover-key     time:   [42.953 ms 42.956 ms 42.959 ms]
bandersnatch-ring-vrf-512/make-prover         time:   [43.005 ms 43.019 ms 43.038 ms]
bandersnatch-ring-vrf-512/sign                time:   [143.81 ms 144.08 ms 144.35 ms]
```

### Ring Size 1024

```
bandersnatch-ring-vrf-1024/make-prover-key    time:   [72.713 ms 72.731 ms 72.747 ms]
bandersnatch-ring-vrf-1024/make-prover        time:   [72.902 ms 72.918 ms 72.933 ms]
bandersnatch-ring-vrf-1024/sign               time:   [253.69 ms 253.75 ms 253.80 ms]
```

### Ring Size 2048

```
bandersnatch-ring-vrf-2048/make-prover-key    time:   [128.57 ms 128.60 ms 128.64 ms]
bandersnatch-ring-vrf-2048/make-prover        time:   [132.16 ms 132.21 ms 132.30 ms]
bandersnatch-ring-vrf-2048/sign               time:   [456.75 ms 456.83 ms 456.91 ms]
```

## Verifying

### Ring Size 512

```
bandersnatch-ring-vrf-512/make-verifier-key   time:   [41.763 ms 41.768 ms 41.774 ms]
bandersnatch-ring-vrf-512/make-verifier       time:   [41.965 ms 41.973 ms 41.980 ms]
bandersnatch-ring-vrf-512/verify              time:   [3.9925 ms 3.9928 ms 3.9931 ms]
```

### Ring Size 1024

```
bandersnatch-ring-vrf-1024/make-verifier-key  time:   [72.985 ms 73.000 ms 73.013 ms]
bandersnatch-ring-vrf-1024/make-verifier      time:   [73.107 ms 73.113 ms 73.120 ms]
bandersnatch-ring-vrf-1024/verify             time:   [4.1118 ms 4.1123 ms 4.1127 ms]
```

### Ring Size 2048

```
bandersnatch-ring-vrf-2048/make-verifier-key  time:   [132.44 ms 132.48 ms 132.53 ms]
bandersnatch-ring-vrf-2048/make-verifier      time:   [132.45 ms 132.49 ms 132.52 ms]
bandersnatch-ring-vrf-2048/verify             time:   [4.1161 ms 4.1166 ms 4.1172 ms]
```
