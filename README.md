A library that abstracts over SIMD instruction sets, including ones with differing widths.
SIMDeez is designed to allow you to write a function one time and produce SSE2, SSE41, and AVX2 versions of the function.
You can either have the version you want chosen at compile time with `cfg` attributes, or at runtime with
`target_feature` attributes and using the built in `is_x86_feature_detected!' macro.

SIMDeez is currently in Beta, if there are intrinsics you need that are not currently implemented, create an issue
and I'll add them. PRs to add more intrinsics are welcome. Currently things are well fleshed out for i32, i64, f32, and f64 types.


# Features

* Can use used with compile time or run time selection
* No runtime overhead
* Uses familiar intel intrinsic naming conventions, easy to port. 
  * `_mm_add_ps(a,b)` becomes `add_ps(a,b)`
* Fills in missing intrinsics in older APIs with fast SIMD workarounds. 
  * ceil, floor, round, etc
* Can be used by `#[no_std]` projects
* Operator overloading: `let sum = va + vb` or `s *= s`
* Extract or set a single lane with the index operator: `let v1 = v[1];`

# Compared to stdsimd

* SIMDeez Can abstract over differing simd widths. stdsimd does not
* SIMDeez builds on stable rust now, stdsimd does not

# Compared to faster

* SIMDeez can be used with runtime selection, Faster cannot.
* SIMDeez has faster fallbacks for some functions
* SIMDeez does not currently work with iterators, faster does.
* SIMDeez uses more idiomatic intrinsic syntax while Faster uses more idomatic Rust syntax
* SIMDeez can be used by `#[no_std]` projects
* SIMDeez builds on stable rust now, Faster does not.

All of the above could change! Faster seems to generally have the same
performance as long as you don't run into some of the slower fallback functions.


# Example

```rust
    // If using runtime feature detection, you will want to be sure this inlines
    // so you can leverage target_feature attributes
    #[inline(always)]
    unsafe fn distance<S: Simd>(x1: &[f32], y1: &[f32], x2: &[f32], y2: &[f32]) -> Vec<f32> {
        let mut result: Vec<f32> = Vec::with_capacity(x1.len());
        result.set_len(x1.len()); // for efficiency

        // Operations have to be done in terms of the vector width
        // so that it will work with any size vector.
        // the width of a vector type is provided as a constant
        // so the compiler is free to optimize it more.
        let mut i = 0;
        //S::VF32_WIDTH is a constant, 4 when using SSE, 8 when using AVX2, etc
        while i < x1.len() {
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
            // Increment i by the vector width
            i += S::VF32_WIDTH
        }
        result
    }

    //Call distance as an SSE2 function
    #[target_feature(enable = "sse2")]
    unsafe fn distance_sse2(x1: &[f32], y1: &[f32], x2: &[f32], y2: &[f32]) -> Vec<f32> {
        distance::<Sse2>(x1, y1, x2, y2)
    }
    //Call distance as an SSE41 function
    #[target_feature(enable = "sse4.1")]
    unsafe fn distance_sse41(x1: &[f32], y1: &[f32], x2: &[f32], y2: &[f32]) -> Vec<f32> {
        distance::<Sse41>(x1, y1, x2, y2)
    }
    //Call distance as an AVX2 function
    #[target_feature(enable = "avx2")]
    unsafe fn distance_avx2(x1: &[f32], y1: &[f32], x2: &[f32], y2: &[f32]) -> Vec<f32> {
        distance::<Avx2>(x1, y1, x2, y2)
    }

```
