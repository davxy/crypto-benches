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
