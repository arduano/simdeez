//! f32 SIMD math kernel layering:
//! - `portable`: backend-agnostic reduction/polynomial kernels + scalar lane patching.
//! - `x86_avx2`: optional hand-optimized override(s) for specific functions.
//! - this module: dispatch glue selecting overrides without changing the public API.

mod hyperbolic;
mod portable;

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
mod x86_avx2;

use crate::{Simd, SimdFloat32};

#[inline(always)]
pub(crate) fn log2_u35<V>(input: V) -> V
where
    V: SimdFloat32,
    V::Engine: Simd<Vf32 = V>,
{
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    {
        if is_avx2_engine::<V::Engine>() {
            return unsafe { x86_avx2::log2_u35(input) };
        }
    }

    portable::log2_u35(input)
}

#[inline(always)]
pub(crate) fn exp2_u35<V>(input: V) -> V
where
    V: SimdFloat32,
    V::Engine: Simd<Vf32 = V>,
{
    portable::exp2_u35(input)
}

#[inline(always)]
pub(crate) fn sin_u35<V>(input: V) -> V
where
    V: SimdFloat32,
    V::Engine: Simd<Vf32 = V>,
{
    portable::sin_u35(input)
}

#[inline(always)]
pub(crate) fn cos_u35<V>(input: V) -> V
where
    V: SimdFloat32,
    V::Engine: Simd<Vf32 = V>,
{
    portable::cos_u35(input)
}

#[inline(always)]
pub(crate) fn tan_u35<V>(input: V) -> V
where
    V: SimdFloat32,
    V::Engine: Simd<Vf32 = V>,
{
    portable::tan_u35(input)
}

#[inline(always)]
pub(crate) fn sinh_u35<V>(input: V) -> V
where
    V: SimdFloat32,
    V::Engine: Simd<Vf32 = V>,
{
    hyperbolic::sinh_u35(input)
}

#[inline(always)]
pub(crate) fn cosh_u35<V>(input: V) -> V
where
    V: SimdFloat32,
    V::Engine: Simd<Vf32 = V>,
{
    hyperbolic::cosh_u35(input)
}

#[inline(always)]
pub(crate) fn tanh_u35<V>(input: V) -> V
where
    V: SimdFloat32,
    V::Engine: Simd<Vf32 = V>,
{
    hyperbolic::tanh_u35(input)
}

#[inline(always)]
pub(crate) fn asinh_u35<V>(input: V) -> V
where
    V: SimdFloat32,
    V::Engine: Simd<Vf32 = V>,
{
    portable::asinh_u35(input)
}

#[inline(always)]
pub(crate) fn acosh_u35<V>(input: V) -> V
where
    V: SimdFloat32,
    V::Engine: Simd<Vf32 = V>,
{
    portable::acosh_u35(input)
}

#[inline(always)]
pub(crate) fn atanh_u35<V>(input: V) -> V
where
    V: SimdFloat32,
    V::Engine: Simd<Vf32 = V>,
{
    portable::atanh_u35(input)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[inline(always)]
fn is_avx2_engine<E: Simd>() -> bool {
    core::any::TypeId::of::<E>() == core::any::TypeId::of::<crate::engines::avx2::Avx2>()
}

#[cfg(test)]
mod tests {
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    #[test]
    fn avx2_dispatch_gate_matches_only_avx2_engine() {
        use crate::engines::{avx2::Avx2, sse2::Sse2};

        assert!(super::is_avx2_engine::<Avx2>());
        assert!(!super::is_avx2_engine::<Sse2>());
    }
}
