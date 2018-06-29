A library that abstracts over SIMD instruction sets, including ones with differing widths.
Implemented such that you can use it with runtime feature detection, or not!

Currently in alpha, I'm implementing just the functions I need for my own projects.  If a fully featured version would be useful to you
or your company I'd be willing to flesh this out with some kind of corporate sponsorship. PRs adding more of the intrinsics are welcome, of course.

Currently supports: AVX2, SSE41, SSE2

# Examples

```rust
// If using runtime feature detection, you will want to be sure this inlines
#[inline(always)]
unsafe fn sample<S: Simd>() -> f32 {
    // function names mirror the intel intrinsics, minus the _mm_ part, call them as usual 
    let a = S::set1_ps(1.5);
    let b = S::set1_ps(2.5);
    let mut c = S::add_ps(a,b);
    // If your SIMD instruction set doesn't have floor, round, gather etc,  SIMDeez handles it for you
    c = S::floor_ps(c);
    // You can get the width (as a const!)  of the instruction set you are working with
    let width = S::WIDTH_BYTES;    
    // And set or get individual lanes with ease
    let first = S::get_lane(c,0);
    let last = S::get_lane(c,(width/4)-1);
    first+last
    
}

// Make an sse2 version of sample 
#[target_feature(enable="sse2")]
unsafe fn sample_sse2() -> f32 {
    sample::<Sse2>()
}

// Make an avx2 version of sample
#[target_feature(enable="avx2")]
unsafe fn sample_avx2() -> f32{
 sample::<Avx2>()
}


// The target_feature attributes ensure that the compiler emits the appropriate instructions on
// a per function basis.


```
