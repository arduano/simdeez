#[cfg(target_arch = "aarch64")]
use core::arch::aarch64::*;
#[cfg(target_arch = "wasm32")]
use core::arch::wasm32::v128;
#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

macro_rules! make_simd_transmute {
    ($name:ident, $scalar:ident, $sse:ident, $avx:ident, $neon:ident, $wasm:ident) => {
        pub trait $name: Sized {
            /// Tries to transmute the value into its underlying scalar type. Panics if the value is not a scalar.
            fn try_transmute_scalar(&self) -> $scalar {
                panic!("Invalid transmute: tried to transmute non-scalar into scalar");
            }

            /// Tries to create the value from its underlying scalar type. Panics if the value is not a scalar.
            fn try_transmute_from_scalar(_scalar: $scalar) -> Self {
                panic!("Invalid transmute: tried to transmute non-scalar into scalar");
            }

            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            /// Tries to transmute the value into its underlying Sse2 type. Panics if the value is not a Sse2.
            fn try_transmute_sse2(&self) -> $sse {
                panic!("Invalid transmute: tried to transmute non-sse2 into sse2");
            }

            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            /// Tries to create the value from its underlying Sse2 type. Panics if the value is not a Sse2.
            fn try_transmute_from_sse2(_sse2: $sse) -> Self {
                panic!("Invalid transmute: tried to transmute non-sse2 into sse2");
            }

            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            /// Tries to transmute the value into its underlying Sse4.1 type. Panics if the value is not a Sse4.1.
            fn try_transmute_sse41(&self) -> $sse {
                panic!("Invalid transmute: tried to transmute non-sse41 into sse41");
            }

            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            /// Tries to create the value from its underlying Sse4.1 type. Panics if the value is not a Sse4.1.
            fn try_transmute_from_sse41(_sse41: $sse) -> Self {
                panic!("Invalid transmute: tried to transmute non-sse41 into sse41");
            }

            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            /// Tries to transmute the value into its underlying Avx2 type. Panics if the value is not a Avx2.
            fn try_transmute_avx2(&self) -> $avx {
                panic!("Invalid transmute: tried to transmute non-avx2 into avx2");
            }

            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            /// Tries to create the value from its underlying Avx2 type. Panics if the value is not a Avx2.
            fn try_transmute_from_avx2(_avx2: $avx) -> Self {
                panic!("Invalid transmute: tried to transmute non-avx2 into avx2");
            }

            #[cfg(target_arch = "aarch64")]
            /// Tries to transmute the value into its underlying Neon type. Panics if the value is not a Neon.
            fn try_transmute_neon(&self) -> $neon {
                panic!("Invalid transmute: tried to transmute non-neon into neon");
            }

            #[cfg(target_arch = "aarch64")]
            /// Tries to create the value from its underlying Neon type. Panics if the value is not a Neon.
            fn try_transmute_from_neon(_neon: $neon) -> Self {
                panic!("Invalid transmute: tried to transmute non-neon into neon");
            }

            #[cfg(target_arch = "wasm32")]
            /// Tries to transmute the value into its underlying Wasm type. Panics if the value is not a Wasm.
            fn try_transmute_wasm(&self) -> $wasm {
                panic!("Invalid transmute: tried to transmute non-wasm into wasm");
            }

            #[cfg(target_arch = "wasm32")]
            /// Tries to create the value from its underlying Wasm type. Panics if the value is not a Wasm.
            fn try_transmute_from_wasm(_wasm: $wasm) -> Self {
                panic!("Invalid transmute: tried to transmute non-wasm into wasm");
            }
        }
    };
}

make_simd_transmute!(SimdTransmuteF32, f32, __m128, __m256, float32x4_t, v128);
make_simd_transmute!(SimdTransmuteF64, f64, __m128d, __m256d, float64x2_t, v128);
make_simd_transmute!(SimdTransmuteI8, i8, __m128i, __m256i, int8x16_t, v128);
make_simd_transmute!(SimdTransmuteI16, i16, __m128i, __m256i, int16x8_t, v128);
make_simd_transmute!(SimdTransmuteI32, i32, __m128i, __m256i, int32x4_t, v128);
make_simd_transmute!(SimdTransmuteI64, i64, __m128i, __m256i, int64x2_t, v128);
