//! This module exists because we are unable to generic-implement these overload traits
//! for all structs that implement the `SimdBase` trait. Rust doesn't allow generic-implementing
//! traits from outside the current module.

macro_rules! impl_simd_base_overloads {
    ($s:ident) => {
        impl Add<Self> for $s {
            type Output = Self;

            #[inline(always)]
            fn add(self, rhs: Self) -> Self {
                SimdBase::add(self, rhs)
            }
        }

        impl AddAssign<Self> for $s {
            #[inline(always)]
            fn add_assign(&mut self, rhs: Self) {
                *self = SimdBase::add(*self, rhs);
            }
        }

        impl Add<<Self as SimdBase>::Scalar> for $s {
            type Output = Self;

            #[inline(always)]
            fn add(self, rhs: <Self as SimdBase>::Scalar) -> Self {
                unsafe { SimdBase::add(self, Self::set1(rhs)) }
            }
        }

        impl Add<$s> for <$s as SimdBase>::Scalar {
            type Output = $s;

            #[inline(always)]
            fn add(self, rhs: $s) -> $s {
                unsafe { SimdBase::add($s::set1(self), rhs) }
            }
        }

        impl AddAssign<<Self as SimdBase>::Scalar> for $s {
            #[inline(always)]
            fn add_assign(&mut self, rhs: <Self as SimdBase>::Scalar) {
                unsafe {
                    *self = SimdBase::add(*self, Self::set1(rhs));
                }
            }
        }

        impl Sub<Self> for $s {
            type Output = Self;

            #[inline(always)]
            fn sub(self, rhs: Self) -> Self {
                SimdBase::sub(self, rhs)
            }
        }

        impl SubAssign<Self> for $s {
            #[inline(always)]
            fn sub_assign(&mut self, rhs: Self) {
                *self = SimdBase::sub(*self, rhs);
            }
        }

        impl Sub<<Self as SimdBase>::Scalar> for $s {
            type Output = Self;

            #[inline(always)]
            fn sub(self, rhs: <Self as SimdBase>::Scalar) -> Self {
                unsafe { SimdBase::sub(self, Self::set1(rhs)) }
            }
        }

        impl Sub<$s> for <$s as SimdBase>::Scalar {
            type Output = $s;

            #[inline(always)]
            fn sub(self, rhs: $s) -> $s {
                unsafe { SimdBase::sub($s::set1(self), rhs) }
            }
        }

        impl SubAssign<<Self as SimdBase>::Scalar> for $s {
            #[inline(always)]
            fn sub_assign(&mut self, rhs: <Self as SimdBase>::Scalar) {
                unsafe {
                    *self = SimdBase::sub(*self, Self::set1(rhs));
                }
            }
        }

        impl Mul<Self> for $s {
            type Output = Self;

            #[inline(always)]
            fn mul(self, rhs: Self) -> Self {
                SimdBase::mul(self, rhs)
            }
        }

        impl MulAssign<Self> for $s {
            #[inline(always)]
            fn mul_assign(&mut self, rhs: Self) {
                *self = SimdBase::mul(*self, rhs);
            }
        }

        impl Mul<<Self as SimdBase>::Scalar> for $s {
            type Output = Self;

            #[inline(always)]
            fn mul(self, rhs: <Self as SimdBase>::Scalar) -> Self {
                unsafe { SimdBase::mul(self, Self::set1(rhs)) }
            }
        }

        impl MulAssign<<Self as SimdBase>::Scalar> for $s {
            #[inline(always)]
            fn mul_assign(&mut self, rhs: <Self as SimdBase>::Scalar) {
                unsafe {
                    *self = SimdBase::mul(*self, Self::set1(rhs));
                }
            }
        }

        impl Mul<$s> for <$s as SimdBase>::Scalar {
            type Output = $s;

            #[inline(always)]
            fn mul(self, rhs: $s) -> $s {
                unsafe { SimdBase::mul($s::set1(self), rhs) }
            }
        }

        impl BitAnd<Self> for $s {
            type Output = Self;

            #[inline(always)]
            fn bitand(self, rhs: Self) -> Self {
                SimdBase::bit_and(self, rhs)
            }
        }

        impl BitAndAssign<Self> for $s {
            #[inline(always)]
            fn bitand_assign(&mut self, rhs: Self) {
                *self = SimdBase::bit_and(*self, rhs);
            }
        }

        impl BitAnd<<Self as SimdBase>::Scalar> for $s {
            type Output = Self;

            #[inline(always)]
            fn bitand(self, rhs: <Self as SimdBase>::Scalar) -> Self {
                unsafe { SimdBase::bit_and(self, SimdBase::set1(rhs)) }
            }
        }

        impl BitAndAssign<<Self as SimdBase>::Scalar> for $s {
            #[inline(always)]
            fn bitand_assign(&mut self, rhs: <Self as SimdBase>::Scalar) {
                unsafe {
                    *self = SimdBase::bit_and(*self, SimdBase::set1(rhs));
                }
            }
        }

        impl BitOr<Self> for $s {
            type Output = Self;

            #[inline(always)]
            fn bitor(self, rhs: Self) -> Self {
                SimdBase::bit_or(self, rhs)
            }
        }

        impl BitOrAssign<Self> for $s {
            #[inline(always)]
            fn bitor_assign(&mut self, rhs: Self) {
                *self = SimdBase::bit_or(*self, rhs);
            }
        }

        impl BitOr<<Self as SimdBase>::Scalar> for $s {
            type Output = Self;

            #[inline(always)]
            fn bitor(self, rhs: <Self as SimdBase>::Scalar) -> Self {
                unsafe { SimdBase::bit_or(self, SimdBase::set1(rhs)) }
            }
        }

        impl BitOrAssign<<Self as SimdBase>::Scalar> for $s {
            #[inline(always)]
            fn bitor_assign(&mut self, rhs: <Self as SimdBase>::Scalar) {
                unsafe {
                    *self = SimdBase::bit_or(*self, SimdBase::set1(rhs));
                }
            }
        }

        impl BitXor<Self> for $s {
            type Output = Self;

            #[inline(always)]
            fn bitxor(self, rhs: Self) -> Self {
                SimdBase::bit_xor(self, rhs)
            }
        }

        impl BitXorAssign<Self> for $s {
            #[inline(always)]
            fn bitxor_assign(&mut self, rhs: Self) {
                *self = SimdBase::bit_xor(*self, rhs);
            }
        }

        impl BitXor<<Self as SimdBase>::Scalar> for $s {
            type Output = Self;

            #[inline(always)]
            fn bitxor(self, rhs: <Self as SimdBase>::Scalar) -> Self {
                unsafe { SimdBase::bit_xor(self, SimdBase::set1(rhs)) }
            }
        }

        impl BitXorAssign<<Self as SimdBase>::Scalar> for $s {
            #[inline(always)]
            fn bitxor_assign(&mut self, rhs: <Self as SimdBase>::Scalar) {
                unsafe {
                    *self = SimdBase::bit_xor(*self, SimdBase::set1(rhs));
                }
            }
        }

        impl Not for $s {
            type Output = Self;

            #[inline(always)]
            fn not(self) -> Self {
                SimdBase::bit_not(self)
            }
        }

        impl Neg for $s {
            type Output = Self;

            #[inline(always)]
            fn neg(self) -> Self {
                unsafe { Self::zeroes() - self }
            }
        }

        impl Index<usize> for $s {
            type Output = <Self as SimdBase>::Scalar;

            fn index(&self, index: usize) -> &Self::Output {
                unsafe { &(*self.transmute_into_array_ref())[index] }
            }
        }

        impl IndexMut<usize> for $s {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                unsafe { &mut (*self.transmute_into_array_mut())[index] }
            }
        }

        impl core::fmt::Debug for $s {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                unsafe {
                    let array = self.transmute_into_array_ref();
                    write!(f, "{}([{:?}])", stringify!($s), array)
                }
            }
        }
    };
}

macro_rules! impl_simd_int_overloads {
    ($s:ident) => {
        impl Shl<i32> for $s {
            type Output = Self;

            #[inline(always)]
            fn shl(self, rhs: i32) -> Self {
                SimdInt::shl(self, rhs)
            }
        }

        impl ShlAssign<i32> for $s {
            #[inline(always)]
            fn shl_assign(&mut self, rhs: i32) {
                *self = SimdInt::shl(*self, rhs);
            }
        }

        impl Shr<i32> for $s {
            type Output = Self;

            #[inline(always)]
            fn shr(self, rhs: i32) -> Self {
                SimdInt::shr(self, rhs)
            }
        }

        impl ShrAssign<i32> for $s {
            #[inline(always)]
            fn shr_assign(&mut self, rhs: i32) {
                *self = SimdInt::shr(*self, rhs);
            }
        }
    };
}

macro_rules! impl_simd_float_overloads {
    ($s:ident) => {
        impl Div<Self> for $s {
            type Output = Self;

            #[inline(always)]
            fn div(self, rhs: Self) -> Self {
                SimdFloat::div(self, rhs)
            }
        }

        impl DivAssign<Self> for $s {
            #[inline(always)]
            fn div_assign(&mut self, rhs: Self) {
                *self = SimdFloat::div(*self, rhs);
            }
        }

        impl Div<<Self as SimdBase>::Scalar> for $s {
            type Output = Self;

            #[inline(always)]
            fn div(self, rhs: <Self as SimdBase>::Scalar) -> Self {
                unsafe { SimdFloat::div(self, Self::set1(rhs)) }
            }
        }

        impl DivAssign<<Self as SimdBase>::Scalar> for $s {
            #[inline(always)]
            fn div_assign(&mut self, rhs: <Self as SimdBase>::Scalar) {
                unsafe {
                    *self = SimdFloat::div(*self, Self::set1(rhs));
                }
            }
        }
    };
}
