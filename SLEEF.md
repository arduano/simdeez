# SLEEF Math Status

SIMDeez includes a native, pure-Rust SIMD math surface modeled after the historical SLEEF family layout.

The public surface currently covers:
- core log/exp: `log2_u35`, `exp2_u35`, `ln_u35`, `exp_u35`
- trig and inverse trig: `sin_u35`, `cos_u35`, `tan_u35`, `asin_u35`, `acos_u35`, `atan_u35`, `atan2_u35`
- hyperbolic and inverse hyperbolic: `sinh_u35`, `cosh_u35`, `tanh_u35`, `asinh_u35`, `acosh_u35`, `atanh_u35`
- binary misc: `log10_u35`, `hypot_u35`, `fmod`

These APIs are exposed through `simdeez::math` and re-exported by `simdeez::prelude`.

## Current Shape

- Most `f32` families use portable SIMD by default.
- `f32 log2_u35` keeps an AVX2 override where local benchmarks justify it.
- Revived `f64` log/exp, inverse trig, and binary-misc families keep SIMD defaults.
- Some `f64` families intentionally remain mixed or scalar-reference where the local rescue passes did not justify a SIMD default.
- The old `sleef-sys` feature remains deprecated and is not the primary implementation path.

## Implementation Pattern

The maintained pattern is:
1. start with portable SIMD kernels
2. add dispatch glue without changing the public API
3. add backend-specific overrides only where profiling justifies them
4. keep scalar-lane patching centralized for exceptional semantics

The restored `f32` path in `src/math/f32/` is the clearest reference implementation of that layering.

## Benchmarks

Criterion targets for the restored math surface:

```bash
cargo bench --bench simd_math
cargo bench --bench simd_math_remaining_baseline
```

These benches report:
- native scalar loop baselines
- the simdeez runtime-selected path
- forced backend variants such as `scalar`, `sse2`, `sse41`, `avx2`, and `avx512` when available

Current expectation:
- `log2_u35` and `exp2_u35` should show clear speedups on SIMD-capable backends
- revived `f64` log/exp and much of the inverse-trig and hyperbolic surface should remain worthwhile on the runtime-selected path
- documented scalar-reference holdouts should stay correctness-first until a later rescue pass produces stronger evidence
