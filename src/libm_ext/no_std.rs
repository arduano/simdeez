use core::{f32, f64};

pub use libm::*;

use super::FloatExt;

impl FloatExt for f32 {
    #[inline]
    fn m_floor(self) -> Self {
        floorf(self)
    }

    #[inline]
    fn m_ceil(self) -> Self {
        ceilf(self)
    }

    #[inline]
    fn m_round(self) -> Self {
        roundf(self)
    }

    #[inline]
    fn m_trunc(self) -> Self {
        truncf(self)
    }

    #[inline]
    fn m_fract(self) -> Self {
        self - self.m_trunc()
    }

    #[inline]
    fn m_abs(self) -> Self {
        fabsf(self)
    }

    #[inline]
    fn m_mul_add(self, a: Self, b: Self) -> Self {
        fmaf(self, a, b)
    }

    #[inline]
    fn m_powf(self, n: Self) -> Self {
        powf(self, n)
    }

    #[inline]
    fn m_sqrt(self) -> Self {
        sqrtf(self)
    }

    #[inline]
    fn m_exp(self) -> Self {
        expf(self)
    }

    #[inline]
    fn m_exp2(self) -> Self {
        exp2f(self)
    }

    #[inline]
    fn m_ln(self) -> Self {
        logf(self)
    }

    #[inline]
    fn m_log(self, base: Self) -> Self {
        self.m_ln() / base.m_ln()
    }

    #[inline]
    fn m_log2(self) -> Self {
        log2f(self)
    }

    #[inline]
    fn m_log10(self) -> Self {
        log10f(self)
    }

    #[inline]
    fn m_cbrt(self) -> Self {
        cbrtf(self)
    }

    #[inline]
    fn m_hypot(self, other: Self) -> Self {
        hypotf(self, other)
    }

    #[inline]
    fn m_sin(self) -> Self {
        sinf(self)
    }

    #[inline]
    fn m_cos(self) -> Self {
        cosf(self)
    }

    #[inline]
    fn m_tan(self) -> Self {
        tanf(self)
    }

    #[inline]
    fn m_asin(self) -> Self {
        asinf(self)
    }

    #[inline]
    fn m_acos(self) -> Self {
        acosf(self)
    }

    #[inline]
    fn m_atan(self) -> Self {
        atanf(self)
    }

    #[inline]
    fn m_atan2(self, other: Self) -> Self {
        atan2f(self, other)
    }

    #[inline]
    fn m_exp_m1(self) -> Self {
        expm1f(self)
    }

    #[inline]
    fn m_ln_1p(self) -> Self {
        log1pf(self)
    }

    #[inline]
    fn m_sinh(self) -> Self {
        sinhf(self)
    }

    #[inline]
    fn m_cosh(self) -> Self {
        coshf(self)
    }

    #[inline]
    fn m_tanh(self) -> Self {
        tanhf(self)
    }

    #[inline]
    fn m_asinh(self) -> Self {
        if self == f32::NEG_INFINITY {
            f32::NEG_INFINITY
        } else {
            (self + ((self * self) + 1.0).m_sqrt()).m_ln()
        }
    }

    #[inline]
    fn m_acosh(self) -> Self {
        match self {
            x if x < 1.0 => f32::NAN,
            x => (x + ((x * x) - 1.0).m_sqrt()).m_ln(),
        }
    }

    #[inline]
    fn m_atanh(self) -> Self {
        0.5 * ((2.0 * self) / (1.0 - self)).m_ln_1p()
    }
}

impl FloatExt for f64 {
    #[inline]
    fn m_floor(self) -> Self {
        floor(self)
    }

    #[inline]
    fn m_ceil(self) -> Self {
        ceil(self)
    }

    #[inline]
    fn m_round(self) -> Self {
        round(self)
    }

    #[inline]
    fn m_trunc(self) -> Self {
        trunc(self)
    }

    #[inline]
    fn m_fract(self) -> Self {
        self - self.m_trunc()
    }

    #[inline]
    fn m_abs(self) -> Self {
        fabs(self)
    }

    #[inline]
    fn m_mul_add(self, a: Self, b: Self) -> Self {
        fma(self, a, b)
    }

    #[inline]
    fn m_powf(self, n: Self) -> Self {
        pow(self, n)
    }

    #[inline]
    fn m_sqrt(self) -> Self {
        sqrt(self)
    }

    #[inline]
    fn m_exp(self) -> Self {
        exp(self)
    }

    #[inline]
    fn m_exp2(self) -> Self {
        exp2(self)
    }

    #[inline]
    fn m_ln(self) -> Self {
        log(self)
    }

    #[inline]
    fn m_log(self, base: Self) -> Self {
        self.m_ln() / base.m_ln()
    }

    #[inline]
    fn m_log2(self) -> Self {
        log2(self)
    }

    #[inline]
    fn m_log10(self) -> Self {
        log10(self)
    }

    #[inline]
    fn m_cbrt(self) -> Self {
        cbrt(self)
    }

    #[inline]
    fn m_hypot(self, other: Self) -> Self {
        hypot(self, other)
    }

    #[inline]
    fn m_sin(self) -> Self {
        sin(self)
    }

    #[inline]
    fn m_cos(self) -> Self {
        cos(self)
    }

    #[inline]
    fn m_tan(self) -> Self {
        tan(self)
    }

    #[inline]
    fn m_asin(self) -> Self {
        asin(self)
    }

    #[inline]
    fn m_acos(self) -> Self {
        acos(self)
    }

    #[inline]
    fn m_atan(self) -> Self {
        atan(self)
    }

    #[inline]
    fn m_atan2(self, other: Self) -> Self {
        atan2(self, other)
    }

    #[inline]
    fn m_exp_m1(self) -> Self {
        expm1(self)
    }

    #[inline]
    fn m_ln_1p(self) -> Self {
        log1p(self)
    }

    #[inline]
    fn m_sinh(self) -> Self {
        sinh(self)
    }

    #[inline]
    fn m_cosh(self) -> Self {
        cosh(self)
    }

    #[inline]
    fn m_tanh(self) -> Self {
        tanh(self)
    }

    #[inline]
    fn m_asinh(self) -> Self {
        if self == f64::NEG_INFINITY {
            f64::NEG_INFINITY
        } else {
            (self + ((self * self) + 1.0).m_sqrt()).m_ln()
        }
    }

    #[inline]
    fn m_acosh(self) -> Self {
        match self {
            x if x < 1.0 => f64::NAN,
            x => (x + ((x * x) - 1.0).m_sqrt()).m_ln(),
        }
    }

    #[inline]
    fn m_atanh(self) -> Self {
        0.5 * ((2.0 * self) / (1.0 - self)).m_ln_1p()
    }
}
