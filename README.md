A library that abstracts over SIMD instruction sets, including ones with differing widths.
SIMDeez is designed to allow you to write a function one time and produce SSE2, SSE41, and AVX2 versions of the function.
You can either have the version you want chosen at compile time or automatically at runtime.

If there are intrinsics you need that are not currently implemented, create an issue
and I'll add them. PRs to add more intrinsics are welcome. Currently things are well fleshed out for i32, i64, f32, and f64 types.

As Rust stabilizes support for Neon and AVX-512 I plan to add those as well.

Refer to the excellent [Intel Intrinsics Guide](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#) for documentation on these functions:

# Features

* SSE2, SSE41, and AVX2, and scalar fallback
* Can be used with compile time or run time selection
* No runtime overhead
* Uses familiar intel intrinsic naming conventions, easy to port. 
  * `_mm_add_ps(a,b)` becomes `add_ps(a,b)`
* Fills in missing intrinsics in older APIs with fast SIMD workarounds. 
  * ceil, floor, round,blend etc
* Can be used by `#[no_std]` projects
* Operator overloading: `let sum = va + vb` or `s *= s`
* Extract or set a single lane with the index operator: `let v1 = v[1];`
* Falls all the way back to scalar code for platforms with no SIMD or unsupported SIMD

# Trig Functions via Sleef-sys
A number of trigonometric and other common math functions are provided
in vectorized form via the Sleef-sys crate. This is an optional feature `sleef` that you can enable.
Doing so currently requires nightly, as well as having CMake and Clang installed.

# Compared to packed_simd

* SIMDeez can abstract over differing simd widths. packed_simd does not
* SIMDeez builds on stable rust now, packed_simd does not

# Compared to Faster

* SIMDeez can be used with runtime selection, Faster cannot.
* SIMDeez has faster fallbacks for some functions
* SIMDeez does not currently work with iterators, Faster does.
* SIMDeez uses more idiomatic intrinsic syntax while Faster uses more idomatic Rust syntax
* SIMDeez builds on stable rust now, Faster does not.

All of the above could change! Faster seems to generally have the same
performance as long as you don't run into some of the slower fallback functions.


# Example

```rust
use simdeez::*;
    use simdeez::scalar::*;
    use simdeez::sse2::*;
    use simdeez::sse41::*;
    use simdeez::avx2::*;
    // If you want your SIMD function to use use runtime feature detection to call
    // the fastest available version, use the simd_runtime_generate macro:
    simd_runtime_generate!(
    fn distance(
        x1: &[f32],
        y1: &[f32],
        x2: &[f32],
        y2: &[f32]) -> Vec<f32> {

        let mut result: Vec<f32> = Vec::with_capacity(x1.len());
        result.set_len(x1.len()); // for efficiency

        // Operations have to be done in terms of the vector width
        // so that it will work with any size vector.
        // the width of a vector type is provided as a constant
        // so the compiler is free to optimize it more.
        // S::VF32_WIDTH is a constant, 4 when using SSE, 8 when using AVX2, etc
        for i in (0..x1.len()).step_by(S::VF32_WIDTH) {
            //load data from your vec into a SIMD value
            let xv1 = S::loadu_ps(&x1[i]);
            let yv1 = S::loadu_ps(&y1[i]);
            let xv2 = S::loadu_ps(&x2[i]);
            let yv2 = S::loadu_ps(&y2[i]);

            // Use the usual intrinsic syntax if you prefer
            let mut xdiff = S::sub_ps(xv1, xv2);
            // Or use operater overloading if you like
            let mut ydiff = yv1 - yv2;
            xdiff *= xdiff;
            ydiff *= ydiff;
            let distance = S::sqrt_ps(xdiff + ydiff);
            // Store the SIMD value into the result vec
            S::storeu_ps(&mut result[i], distance);
        }
        result
    });
fn main() {
}
```
This will generate 5 functions for you:
* `distance<S:Simd>` the generic version of your function
* `distance_scalar`  a scalar fallback
* `distance_sse2`    SSE2 version
* `distance_sse41`   SSE41 version
* `distance_avx2`    AVX2 version
* `distance_runtime_select`  // picks the fastest of the above at runtime

You can use any of these you wish, though typically you would use the runtime_select version
unless you want to force an older instruction set to avoid throttling or for other arcane
reasons.
Optionally you can use the `simd_compiletime_generate!` macro in the same way.  This will
produce 2 active functions via the `cfg` attribute feature:

* `distance<S:Simd>`      the generic version of your function
* `distance_compiletime`  the fastest instruction set availble for the given compile time
feature set

You may also forgoe the macros if you know what you are doing, just keep in mind there are lots
of arcane subtleties with inlining and target_features that must be managed. See how the macros
expand for more detail.
