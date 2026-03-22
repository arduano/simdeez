mod binary_misc;
mod core;
mod hyperbolic;
mod inverse_hyperbolic;
mod inverse_trig;

use crate::{SimdFloat32, SimdFloat64};

pub use binary_misc::{SimdMathF32BinaryMisc, SimdMathF64BinaryMisc};
pub use core::{SimdMathF32Core, SimdMathF64Core};
pub use hyperbolic::{SimdMathF32Hyperbolic, SimdMathF64Hyperbolic};
pub use inverse_hyperbolic::{SimdMathF32InverseHyperbolic, SimdMathF64InverseHyperbolic};
pub use inverse_trig::{SimdMathF32InverseTrig, SimdMathF64InverseTrig};

/// Full SIMD math surface grouped into family-specific extension traits.
pub trait SimdMathF32:
    SimdFloat32
    + SimdMathF32Core
    + SimdMathF32InverseTrig
    + SimdMathF32Hyperbolic
    + SimdMathF32InverseHyperbolic
    + SimdMathF32BinaryMisc
{
}

impl<T> SimdMathF32 for T where
    T: SimdFloat32
        + SimdMathF32Core
        + SimdMathF32InverseTrig
        + SimdMathF32Hyperbolic
        + SimdMathF32InverseHyperbolic
        + SimdMathF32BinaryMisc
{
}

/// Full SIMD math surface grouped into family-specific extension traits.
pub trait SimdMathF64:
    SimdFloat64
    + SimdMathF64Core
    + SimdMathF64InverseTrig
    + SimdMathF64Hyperbolic
    + SimdMathF64InverseHyperbolic
    + SimdMathF64BinaryMisc
{
}

impl<T> SimdMathF64 for T where
    T: SimdFloat64
        + SimdMathF64Core
        + SimdMathF64InverseTrig
        + SimdMathF64Hyperbolic
        + SimdMathF64InverseHyperbolic
        + SimdMathF64BinaryMisc
{
}
