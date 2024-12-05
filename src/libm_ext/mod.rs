#![allow(dead_code)]
#[cfg(feature = "no_std")]
mod no_std;

#[cfg(not(feature = "no_std"))]
mod with_std;

/// Math support for `f32`
pub trait FloatExt: private::Sealed + Sized {
    fn m_floor(self) -> Self;

    fn m_ceil(self) -> Self;

    fn m_round(self) -> Self;

    fn m_trunc(self) -> Self;

    fn m_fract(self) -> Self;

    fn m_abs(self) -> Self;

    fn m_mul_add(self, a: Self, b: Self) -> Self;

    fn m_powf(self, n: Self) -> Self;

    fn m_sqrt(self) -> Self;

    fn m_exp(self) -> Self;

    fn m_exp2(self) -> Self;

    fn m_ln(self) -> Self;

    fn m_log(self, base: Self) -> Self;

    fn m_log2(self) -> Self;

    fn m_log10(self) -> Self;

    fn m_cbrt(self) -> Self;

    fn m_hypot(self, other: Self) -> Self;

    fn m_sin(self) -> Self;

    fn m_cos(self) -> Self;

    fn m_tan(self) -> Self;

    fn m_asin(self) -> Self;

    fn m_acos(self) -> Self;

    fn m_atan(self) -> Self;

    fn m_atan2(self, other: Self) -> Self;

    #[inline]
    fn m_sin_cos(self) -> (Self, Self)
    where
        Self: Copy,
    {
        (self.m_sin(), self.m_cos())
    }

    fn m_exp_m1(self) -> Self;

    fn m_ln_1p(self) -> Self;

    fn m_sinh(self) -> Self;

    fn m_cosh(self) -> Self;

    fn m_tanh(self) -> Self;

    fn m_asinh(self) -> Self;

    fn m_acosh(self) -> Self;

    fn m_atanh(self) -> Self;
}

mod private {
    pub trait Sealed {}

    impl Sealed for f32 {}
    impl Sealed for f64 {}
}
