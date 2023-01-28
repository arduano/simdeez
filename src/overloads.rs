//! This module exists because we are unable to generic-implement these overload traits
//! for all structs that implement the `SimdBaseOps` trait. Rust doesn't allow generic-implementing
//! traits from outside the current module.

macro_rules! impl_simd_base_overloads {
    ($s:ident) => {
        impl Add<Self> for $s {
            type Output = Self;

            #[inline(always)]
            fn add(self, rhs: Self) -> Self {
                SimdBaseOps::add(self, rhs)
            }
        }

        impl AddAssign<Self> for $s {
            #[inline(always)]
            fn add_assign(&mut self, rhs: Self) {
                *self = SimdBaseOps::add(*self, rhs);
            }
        }

        impl Add<<Self as SimdConsts>::Scalar> for $s {
            type Output = Self;

            #[inline(always)]
            fn add(self, rhs: <Self as SimdConsts>::Scalar) -> Self {
                unsafe { SimdBaseOps::add(self, <Self as InternalSimdBaseIo>::set1(rhs)) }
            }
        }

        impl Add<$s> for <$s as SimdConsts>::Scalar {
            type Output = $s;

            #[inline(always)]
            fn add(self, rhs: $s) -> $s {
                unsafe { SimdBaseOps::add(<$s as InternalSimdBaseIo>::set1(self), rhs) }
            }
        }

        impl AddAssign<<Self as SimdConsts>::Scalar> for $s {
            #[inline(always)]
            fn add_assign(&mut self, rhs: <Self as SimdConsts>::Scalar) {
                unsafe {
                    *self = SimdBaseOps::add(*self, <Self as InternalSimdBaseIo>::set1(rhs));
                }
            }
        }

        impl Sub<Self> for $s {
            type Output = Self;

            #[inline(always)]
            fn sub(self, rhs: Self) -> Self {
                SimdBaseOps::sub(self, rhs)
            }
        }

        impl SubAssign<Self> for $s {
            #[inline(always)]
            fn sub_assign(&mut self, rhs: Self) {
                *self = SimdBaseOps::sub(*self, rhs);
            }
        }

        impl Sub<<Self as SimdConsts>::Scalar> for $s {
            type Output = Self;

            #[inline(always)]
            fn sub(self, rhs: <Self as SimdConsts>::Scalar) -> Self {
                unsafe { SimdBaseOps::sub(self, <Self as InternalSimdBaseIo>::set1(rhs)) }
            }
        }

        impl Sub<$s> for <$s as SimdConsts>::Scalar {
            type Output = $s;

            #[inline(always)]
            fn sub(self, rhs: $s) -> $s {
                unsafe { SimdBaseOps::sub(<$s as InternalSimdBaseIo>::set1(self), rhs) }
            }
        }

        impl SubAssign<<Self as SimdConsts>::Scalar> for $s {
            #[inline(always)]
            fn sub_assign(&mut self, rhs: <Self as SimdConsts>::Scalar) {
                unsafe {
                    *self = SimdBaseOps::sub(*self, <Self as InternalSimdBaseIo>::set1(rhs));
                }
            }
        }

        impl Mul<Self> for $s {
            type Output = Self;

            #[inline(always)]
            fn mul(self, rhs: Self) -> Self {
                SimdBaseOps::mul(self, rhs)
            }
        }

        impl MulAssign<Self> for $s {
            #[inline(always)]
            fn mul_assign(&mut self, rhs: Self) {
                *self = SimdBaseOps::mul(*self, rhs);
            }
        }

        impl Mul<<Self as SimdConsts>::Scalar> for $s {
            type Output = Self;

            #[inline(always)]
            fn mul(self, rhs: <Self as SimdConsts>::Scalar) -> Self {
                unsafe { SimdBaseOps::mul(self, <Self as InternalSimdBaseIo>::set1(rhs)) }
            }
        }

        impl MulAssign<<Self as SimdConsts>::Scalar> for $s {
            #[inline(always)]
            fn mul_assign(&mut self, rhs: <Self as SimdConsts>::Scalar) {
                unsafe {
                    *self = SimdBaseOps::mul(*self, <Self as InternalSimdBaseIo>::set1(rhs));
                }
            }
        }

        impl Mul<$s> for <$s as SimdConsts>::Scalar {
            type Output = $s;

            #[inline(always)]
            fn mul(self, rhs: $s) -> $s {
                unsafe { SimdBaseOps::mul(<$s as InternalSimdBaseIo>::set1(self), rhs) }
            }
        }

        impl BitAnd<Self> for $s {
            type Output = Self;

            #[inline(always)]
            fn bitand(self, rhs: Self) -> Self {
                SimdBaseOps::bit_and(self, rhs)
            }
        }

        impl BitAndAssign<Self> for $s {
            #[inline(always)]
            fn bitand_assign(&mut self, rhs: Self) {
                *self = SimdBaseOps::bit_and(*self, rhs);
            }
        }

        impl BitAnd<<Self as SimdConsts>::Scalar> for $s {
            type Output = Self;

            #[inline(always)]
            fn bitand(self, rhs: <Self as SimdConsts>::Scalar) -> Self {
                unsafe { SimdBaseOps::bit_and(self, InternalSimdBaseIo::set1(rhs)) }
            }
        }

        impl BitAndAssign<<Self as SimdConsts>::Scalar> for $s {
            #[inline(always)]
            fn bitand_assign(&mut self, rhs: <Self as SimdConsts>::Scalar) {
                unsafe {
                    *self = SimdBaseOps::bit_and(*self, InternalSimdBaseIo::set1(rhs));
                }
            }
        }

        impl BitOr<Self> for $s {
            type Output = Self;

            #[inline(always)]
            fn bitor(self, rhs: Self) -> Self {
                SimdBaseOps::bit_or(self, rhs)
            }
        }

        impl BitOrAssign<Self> for $s {
            #[inline(always)]
            fn bitor_assign(&mut self, rhs: Self) {
                *self = SimdBaseOps::bit_or(*self, rhs);
            }
        }

        impl BitOr<<Self as SimdConsts>::Scalar> for $s {
            type Output = Self;

            #[inline(always)]
            fn bitor(self, rhs: <Self as SimdConsts>::Scalar) -> Self {
                unsafe { SimdBaseOps::bit_or(self, InternalSimdBaseIo::set1(rhs)) }
            }
        }

        impl BitOrAssign<<Self as SimdConsts>::Scalar> for $s {
            #[inline(always)]
            fn bitor_assign(&mut self, rhs: <Self as SimdConsts>::Scalar) {
                unsafe {
                    *self = SimdBaseOps::bit_or(*self, InternalSimdBaseIo::set1(rhs));
                }
            }
        }

        impl BitXor<Self> for $s {
            type Output = Self;

            #[inline(always)]
            fn bitxor(self, rhs: Self) -> Self {
                SimdBaseOps::bit_xor(self, rhs)
            }
        }

        impl BitXorAssign<Self> for $s {
            #[inline(always)]
            fn bitxor_assign(&mut self, rhs: Self) {
                *self = SimdBaseOps::bit_xor(*self, rhs);
            }
        }

        impl BitXor<<Self as SimdConsts>::Scalar> for $s {
            type Output = Self;

            #[inline(always)]
            fn bitxor(self, rhs: <Self as SimdConsts>::Scalar) -> Self {
                unsafe { SimdBaseOps::bit_xor(self, InternalSimdBaseIo::set1(rhs)) }
            }
        }

        impl BitXorAssign<<Self as SimdConsts>::Scalar> for $s {
            #[inline(always)]
            fn bitxor_assign(&mut self, rhs: <Self as SimdConsts>::Scalar) {
                unsafe {
                    *self = SimdBaseOps::bit_xor(*self, InternalSimdBaseIo::set1(rhs));
                }
            }
        }

        impl Not for $s {
            type Output = Self;

            #[inline(always)]
            fn not(self) -> Self {
                SimdBaseOps::bit_not(self)
            }
        }

        impl Neg for $s {
            type Output = Self;

            #[inline(always)]
            fn neg(self) -> Self {
                unsafe { <Self as InternalSimdBaseIo>::zeroes() - self }
            }
        }

        impl Index<usize> for $s {
            type Output = <Self as SimdConsts>::Scalar;

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

        impl Div<<Self as SimdConsts>::Scalar> for $s {
            type Output = Self;

            #[inline(always)]
            fn div(self, rhs: <Self as SimdConsts>::Scalar) -> Self {
                unsafe { SimdFloat::div(self, <Self as InternalSimdBaseIo>::set1(rhs)) }
            }
        }

        impl DivAssign<<Self as SimdConsts>::Scalar> for $s {
            #[inline(always)]
            fn div_assign(&mut self, rhs: <Self as SimdConsts>::Scalar) {
                unsafe {
                    *self = SimdFloat::div(*self, <Self as InternalSimdBaseIo>::set1(rhs));
                }
            }
        }
    };
}

macro_rules! horizontal_add_scalar {
    (i8) => {
        i64
    };
    (i16) => {
        i64
    };
    (i32) => {
        i64
    };
    (i64) => {
        i64
    };
    (f32) => {
        f32
    };
    (f64) => {
        f64
    };
}

macro_rules! define_simd_type {
    ($ty:ty, $width:literal, $underlying:ty) => {
        paste::item! {
            #[derive(Copy, Clone)]
            pub struct [<$ty:upper x $width>]($underlying);
            impl_simd_base_overloads!([<$ty:upper x $width>]);

            impl SimdConsts for [<$ty:upper x $width>] {
                const WIDTH: usize = $width;
                type Scalar = $ty;
                type HorizontalAddScalar = horizontal_add_scalar!($ty);
                type ArrayRepresentation = [$ty; $width];
                type UnderlyingType = $underlying;
            }
        }
    };
    ($ty:ty, $width:literal, $underlying:ty, $suffix:ident) => {
        paste::item! {
            #[derive(Copy, Clone)]
            pub struct [<$ty:upper x $width $suffix>]($underlying);
            impl_simd_base_overloads!([<$ty:upper x $width $suffix>]);

            impl SimdConsts for [<$ty:upper x $width $suffix >] {
                const WIDTH: usize = $width;
                type Scalar = $ty;
                type HorizontalAddScalar = horizontal_add_scalar!($ty);
                type ArrayRepresentation = [$ty; $width];
                type UnderlyingType = $underlying;
            }
        }
    };
}
