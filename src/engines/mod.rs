pub mod scalar;

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
pub mod avx2;
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
pub mod sse2;
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
pub mod sse41;

#[cfg(target_arch = "aarch64")]
pub mod neon;

#[cfg(target_arch = "wasm32")]
pub mod wasm32;
