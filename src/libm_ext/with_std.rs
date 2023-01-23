use super::FloatExt;

impl FloatExt for f32 {
    #[inline]
    fn m_floor(self) -> Self {
        f32::floor(self)
    }

    #[inline]
    fn m_ceil(self) -> Self {
        f32::ceil(self)
    }

    #[inline]
    fn m_round(self) -> Self {
        f32::round(self)
    }

    #[inline]
    fn m_trunc(self) -> Self {
        f32::trunc(self)
    }

    #[inline]
    fn m_fract(self) -> Self {
        self - self.trunc()
    }

    #[inline]
    fn m_abs(self) -> Self {
        f32::abs(self)
    }

    #[inline]
    fn m_mul_add(self, a: Self, b: Self) -> Self {
        f32::mul_add(self, a, b)
    }

    #[inline]
    fn m_powf(self, n: Self) -> Self {
        f32::powf(self, n)
    }

    #[inline]
    fn m_sqrt(self) -> Self {
        f32::sqrt(self)
    }

    #[inline]
    fn m_exp(self) -> Self {
        f32::exp(self)
    }

    #[inline]
    fn m_exp2(self) -> Self {
        f32::exp2(self)
    }

    #[inline]
    fn m_ln(self) -> Self {
        f32::ln(self)
    }

    #[inline]
    fn m_log(self, base: Self) -> Self {
        f32::log(self, base)
    }

    #[inline]
    fn m_log2(self) -> Self {
        f32::log2(self)
    }

    #[inline]
    fn m_log10(self) -> Self {
        f32::log10(self)
    }

    #[inline]
    fn m_cbrt(self) -> Self {
        f32::cbrt(self)
    }

    #[inline]
    fn m_hypot(self, other: Self) -> Self {
        f32::hypot(self, other)
    }

    #[inline]
    fn m_sin(self) -> Self {
        f32::sin(self)
    }

    #[inline]
    fn m_cos(self) -> Self {
        f32::cos(self)
    }

    #[inline]
    fn m_tan(self) -> Self {
        f32::tan(self)
    }

    #[inline]
    fn m_asin(self) -> Self {
        f32::asin(self)
    }

    #[inline]
    fn m_acos(self) -> Self {
        f32::acos(self)
    }

    #[inline]
    fn m_atan(self) -> Self {
        f32::atan(self)
    }

    #[inline]
    fn m_atan2(self, other: Self) -> Self {
        f32::atan2(self, other)
    }

    #[inline]
    fn m_exp_m1(self) -> Self {
        f32::exp_m1(self)
    }

    #[inline]
    fn m_ln_1p(self) -> Self {
        f32::ln_1p(self)
    }

    #[inline]
    fn m_sinh(self) -> Self {
        f32::sinh(self)
    }

    #[inline]
    fn m_cosh(self) -> Self {
        f32::cosh(self)
    }

    #[inline]
    fn m_tanh(self) -> Self {
        f32::tanh(self)
    }

    #[inline]
    fn m_asinh(self) -> Self {
        f32::asinh(self)
    }

    #[inline]
    fn m_acosh(self) -> Self {
        f32::acosh(self)
    }

    #[inline]
    fn m_atanh(self) -> Self {
        f32::atanh(self)
    }
}

impl FloatExt for f64 {
    #[inline]
    fn m_floor(self) -> Self {
        f64::floor(self)
    }

    #[inline]
    fn m_ceil(self) -> Self {
        f64::ceil(self)
    }

    #[inline]
    fn m_round(self) -> Self {
        f64::round(self)
    }

    #[inline]
    fn m_trunc(self) -> Self {
        f64::trunc(self)
    }

    #[inline]
    fn m_fract(self) -> Self {
        self - self.trunc()
    }

    #[inline]
    fn m_abs(self) -> Self {
        f64::abs(self)
    }

    #[inline]
    fn m_mul_add(self, a: Self, b: Self) -> Self {
        f64::mul_add(self, a, b)
    }

    #[inline]
    fn m_powf(self, n: Self) -> Self {
        f64::powf(self, n)
    }

    #[inline]
    fn m_sqrt(self) -> Self {
        f64::sqrt(self)
    }

    #[inline]
    fn m_exp(self) -> Self {
        f64::exp(self)
    }

    #[inline]
    fn m_exp2(self) -> Self {
        f64::exp2(self)
    }

    #[inline]
    fn m_ln(self) -> Self {
        f64::ln(self)
    }

    #[inline]
    fn m_log(self, base: Self) -> Self {
        f64::log(self, base)
    }

    #[inline]
    fn m_log2(self) -> Self {
        f64::log2(self)
    }

    #[inline]
    fn m_log10(self) -> Self {
        f64::log10(self)
    }

    #[inline]
    fn m_cbrt(self) -> Self {
        f64::cbrt(self)
    }

    #[inline]
    fn m_hypot(self, other: Self) -> Self {
        f64::hypot(self, other)
    }

    #[inline]
    fn m_sin(self) -> Self {
        f64::sin(self)
    }

    #[inline]
    fn m_cos(self) -> Self {
        f64::cos(self)
    }

    #[inline]
    fn m_tan(self) -> Self {
        f64::tan(self)
    }

    #[inline]
    fn m_asin(self) -> Self {
        f64::asin(self)
    }

    #[inline]
    fn m_acos(self) -> Self {
        f64::acos(self)
    }

    #[inline]
    fn m_atan(self) -> Self {
        f64::atan(self)
    }

    #[inline]
    fn m_atan2(self, other: Self) -> Self {
        f64::atan2(self, other)
    }

    #[inline]
    fn m_exp_m1(self) -> Self {
        f64::exp_m1(self)
    }

    #[inline]
    fn m_ln_1p(self) -> Self {
        f64::ln_1p(self)
    }

    #[inline]
    fn m_sinh(self) -> Self {
        f64::sinh(self)
    }

    #[inline]
    fn m_cosh(self) -> Self {
        f64::cosh(self)
    }

    #[inline]
    fn m_tanh(self) -> Self {
        f64::tanh(self)
    }

    #[inline]
    fn m_asinh(self) -> Self {
        f64::asinh(self)
    }

    #[inline]
    fn m_acosh(self) -> Self {
        f64::acosh(self)
    }

    #[inline]
    fn m_atanh(self) -> Self {
        f64::atanh(self)
    }
}
