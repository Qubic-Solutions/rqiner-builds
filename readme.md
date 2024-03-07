# Archive for optimized rqiner algo source code (SEP 23 -> MAR 24)


## keccak-p-x

Macros were stripped from https://github.com/codahale/keccak-p.
To make use of SIMD it is required to enable the required target-features through `RUSTFLAGS`, e.g.

```sh
export RUSTFLAGS="-C target-cpu=znver4"
```
