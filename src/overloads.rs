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
                SimdBaseOps::add(self, <Self as SimdBaseIo>::set1(rhs))
            }
        }

        impl Add<$s> for <$s as SimdConsts>::Scalar {
            type Output = $s;

            #[inline(always)]
            fn add(self, rhs: $s) -> $s {
                SimdBaseOps::add(<$s as SimdBaseIo>::set1(self), rhs)
            }
        }

        impl AddAssign<<Self as SimdConsts>::Scalar> for $s {
            #[inline(always)]
            fn add_assign(&mut self, rhs: <Self as SimdConsts>::Scalar) {
                *self = SimdBaseOps::add(*self, <Self as SimdBaseIo>::set1(rhs));
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
                SimdBaseOps::sub(self, <Self as SimdBaseIo>::set1(rhs))
            }
        }

        impl Sub<$s> for <$s as SimdConsts>::Scalar {
            type Output = $s;

            #[inline(always)]
            fn sub(self, rhs: $s) -> $s {
                SimdBaseOps::sub(<$s as SimdBaseIo>::set1(self), rhs)
            }
        }

        impl SubAssign<<Self as SimdConsts>::Scalar> for $s {
            #[inline(always)]
            fn sub_assign(&mut self, rhs: <Self as SimdConsts>::Scalar) {
                *self = SimdBaseOps::sub(*self, <Self as SimdBaseIo>::set1(rhs));
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
                SimdBaseOps::mul(self, <Self as SimdBaseIo>::set1(rhs))
            }
        }

        impl MulAssign<<Self as SimdConsts>::Scalar> for $s {
            #[inline(always)]
            fn mul_assign(&mut self, rhs: <Self as SimdConsts>::Scalar) {
                *self = SimdBaseOps::mul(*self, <Self as SimdBaseIo>::set1(rhs));
            }
        }

        impl Mul<$s> for <$s as SimdConsts>::Scalar {
            type Output = $s;

            #[inline(always)]
            fn mul(self, rhs: $s) -> $s {
                SimdBaseOps::mul(<$s as SimdBaseIo>::set1(self), rhs)
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
                SimdBaseOps::bit_and(self, SimdBaseIo::set1(rhs))
            }
        }

        impl BitAndAssign<<Self as SimdConsts>::Scalar> for $s {
            #[inline(always)]
            fn bitand_assign(&mut self, rhs: <Self as SimdConsts>::Scalar) {
                *self = SimdBaseOps::bit_and(*self, SimdBaseIo::set1(rhs));
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
                SimdBaseOps::bit_or(self, SimdBaseIo::set1(rhs))
            }
        }

        impl BitOrAssign<<Self as SimdConsts>::Scalar> for $s {
            #[inline(always)]
            fn bitor_assign(&mut self, rhs: <Self as SimdConsts>::Scalar) {
                *self = SimdBaseOps::bit_or(*self, SimdBaseIo::set1(rhs));
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
                SimdBaseOps::bit_xor(self, SimdBaseIo::set1(rhs))
            }
        }

        impl BitXorAssign<<Self as SimdConsts>::Scalar> for $s {
            #[inline(always)]
            fn bitxor_assign(&mut self, rhs: <Self as SimdConsts>::Scalar) {
                *self = SimdBaseOps::bit_xor(*self, SimdBaseIo::set1(rhs));
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
                <Self as SimdBaseIo>::zeroes() - self
            }
        }

        impl Index<usize> for $s {
            type Output = <Self as SimdConsts>::Scalar;

            #[inline(always)]
            fn index(&self, index: usize) -> &Self::Output {
                unsafe { &(*self.transmute_into_array_ref())[index] }
            }
        }

        impl IndexMut<usize> for $s {
            #[inline(always)]
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
                SimdFloat::div(self, <Self as SimdBaseIo>::set1(rhs))
            }
        }

        impl DivAssign<<Self as SimdConsts>::Scalar> for $s {
            #[inline(always)]
            fn div_assign(&mut self, rhs: <Self as SimdConsts>::Scalar) {
                *self = SimdFloat::div(*self, <Self as SimdBaseIo>::set1(rhs));
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
    (Scalar, $ty:ty, $width:literal, $underlying:ty) => {
        paste::item! {
            #[derive(Copy, Clone)]
            pub struct [<$ty:upper x $width>](pub $underlying);
            impl_simd_base_overloads!([<$ty:upper x $width>]);

            impl SimdConsts for [<$ty:upper x $width>] {
                const WIDTH: usize = $width;
                type Scalar = $ty;
                type HorizontalAddScalar = horizontal_add_scalar!($ty);
                type ArrayRepresentation = [$ty; $width];
                type UnderlyingType = $underlying;
                type Engine = Scalar;
            }

            impl [<SimdTransmute $ty:upper>] for [<$ty:upper x $width>] {
                fn [<try_transmute_ scalar>](&self) -> $underlying {
                    self.0
                }

                fn [<try_transmute_from_ scalar>](val: $underlying) -> Self {
                    Self(val)
                }
            }
        }
    };

    ($engine:ident, $ty:ty, $width:literal, $underlying:ty) => {
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
                type Engine = $engine;
            }

            impl [<SimdTransmute $ty:upper>] for [<$ty:upper x $width>] {
                fn [<try_transmute_ $engine:lower>](&self) -> $underlying {
                    self.0
                }

                fn [<try_transmute_from_ $engine:lower>](val: $underlying) -> Self {
                    Self(val)
                }
            }
        }
    };
    ($engine:ident, $ty:ty, $width:literal, $underlying:ty, $suffix:ident) => {
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
                type Engine = $engine;
            }

            impl [<SimdTransmute $ty:upper>] for [<$ty:upper x $width $suffix >] {
                fn [<try_transmute_ $engine:lower>](&self) -> $underlying {
                    self.0
                }

                fn [<try_transmute_from_ $engine:lower>](val: $underlying) -> Self {
                    Self(val)
                }
            }
        }
    };
}

macro_rules! impl_simd_base {
    ($engine:ident, $ty:ident, $scalar_ty:ident, |$self:ident| {
        $($hadd:tt)*
    }) => {
        impl SimdBaseIo for $ty {
            #[inline(always)]
            fn zeroes() -> Self {
                unsafe { Self(Ops::<$engine, $scalar_ty>::zeroes()) }
            }

            #[inline(always)]
            fn set1(x: Self::Scalar) -> Self {
                unsafe { Self(Ops::<$engine, $scalar_ty>::set1(x)) }
            }

            #[inline(always)]
            unsafe fn load_from_array(array: Self::ArrayRepresentation) -> Self {
                unsafe { Self(Ops::<$engine, $scalar_ty>::load_aligned(array.as_ptr())) }
            }

            #[inline(always)]
            unsafe fn load_from_ptr_unaligned(ptr: *const Self::Scalar) -> Self {
                unsafe { Self(Ops::<$engine, $scalar_ty>::load_unaligned(ptr)) }
            }

            #[inline(always)]
            unsafe fn copy_to_ptr_unaligned(self, ptr: *mut Self::Scalar) {
                unsafe { Ops::<$engine, $scalar_ty>::store_unaligned(ptr, self.0) }
            }

            #[inline(always)]
            unsafe fn load_from_ptr_aligned(ptr: *const Self::Scalar) -> Self {
                unsafe { Self(Ops::<$engine, $scalar_ty>::load_aligned(ptr)) }
            }

            #[inline(always)]
            unsafe fn copy_to_ptr_aligned(self, ptr: *mut Self::Scalar) {
                unsafe { Ops::<$engine, $scalar_ty>::store_aligned(ptr, self.0) }
            }

            #[inline(always)]
            unsafe fn underlying_value(self) -> Self::UnderlyingType {
                self.0
            }

            #[inline(always)]
            unsafe fn underlying_value_mut(&mut self) -> &mut Self::UnderlyingType {
                &mut self.0
            }

            #[inline(always)]
            unsafe fn from_underlying_value(value: Self::UnderlyingType) -> Self {
                Self(value)
            }
        }

        impl SimdBaseOps for $ty {
            #[inline(always)]
            fn add(self, rhs: Self) -> Self {
                unsafe { Self(Ops::<$engine, $scalar_ty>::add(self.0, rhs.0)) }
            }

            #[inline(always)]
            fn sub(self, rhs: Self) -> Self {
                unsafe { Self(Ops::<$engine, $scalar_ty>::sub(self.0, rhs.0)) }
            }

            #[inline(always)]
            fn mul(self, rhs: Self) -> Self {
                unsafe { Self(Ops::<$engine, $scalar_ty>::mul(self.0, rhs.0)) }
            }

            #[inline(always)]
            fn bit_and(self, rhs: Self) -> Self {
                unsafe {
                    let left = Ops::<$engine, $scalar_ty>::bitcast_binary(self.0);
                    let right = Ops::<$engine, $scalar_ty>::bitcast_binary(rhs.0);
                    let result = Ops::<$engine, binary>::bit_and(left, right);
                    paste::paste! {
                        Self(Ops::<$engine, binary>::[<bitcast_ $scalar_ty>](result))
                    }
                }
            }

            #[inline(always)]
            fn bit_or(self, rhs: Self) -> Self {
                unsafe {
                    let left = Ops::<$engine, $scalar_ty>::bitcast_binary(self.0);
                    let right = Ops::<$engine, $scalar_ty>::bitcast_binary(rhs.0);
                    let result = Ops::<$engine, binary>::bit_or(left, right);
                    paste::paste! {
                        Self(Ops::<$engine, binary>::[<bitcast_ $scalar_ty>](result))
                    }
                }
            }

            #[inline(always)]
            fn bit_xor(self, rhs: Self) -> Self {
                unsafe {
                    let left = Ops::<$engine, $scalar_ty>::bitcast_binary(self.0);
                    let right = Ops::<$engine, $scalar_ty>::bitcast_binary(rhs.0);
                    let result = Ops::<$engine, binary>::bit_xor(left, right);
                    paste::paste! {
                        Self(Ops::<$engine, binary>::[<bitcast_ $scalar_ty>](result))
                    }
                }
            }

            #[inline(always)]
            fn bit_not(self) -> Self {
                unsafe {
                    let val = Ops::<$engine, $scalar_ty>::bitcast_binary(self.0);
                    let result = Ops::<$engine, binary>::bit_not(val);
                    paste::paste! {
                        Self(Ops::<$engine, binary>::[<bitcast_ $scalar_ty>](result))
                    }
                }
            }

            #[inline(always)]
            fn abs(self) -> Self {
                unsafe { Self(Ops::<$engine, $scalar_ty>::abs(self.0)) }
            }

            #[inline(always)]
            fn and_not(self, rhs: Self) -> Self {
                unsafe {
                    let left = Ops::<$engine, $scalar_ty>::bitcast_binary(self.0);
                    let right = Ops::<$engine, $scalar_ty>::bitcast_binary(rhs.0);
                    let result = Ops::<$engine, binary>::bit_andnot(right, left);
                    paste::paste! {
                        Self(Ops::<$engine, binary>::[<bitcast_ $scalar_ty>](result))
                    }
                }
            }

            #[inline(always)]
            fn blendv(self, a: Self, b: Self) -> Self {
                unsafe { Self(Ops::<$engine, $scalar_ty>::blendv(a.0, b.0, self.0)) }
            }

            #[inline(always)]
            fn cmp_eq(self, rhs: Self) -> Self {
                unsafe { Self(Ops::<$engine, $scalar_ty>::eq(self.0, rhs.0)) }
            }

            #[inline(always)]
            fn cmp_neq(self, rhs: Self) -> Self {
                unsafe { Self(Ops::<$engine, $scalar_ty>::neq(self.0, rhs.0)) }
            }

            #[inline(always)]
            fn cmp_lt(self, rhs: Self) -> Self {
                unsafe { Self(Ops::<$engine, $scalar_ty>::lt(self.0, rhs.0)) }
            }

            #[inline(always)]
            fn cmp_lte(self, rhs: Self) -> Self {
                unsafe { Self(Ops::<$engine, $scalar_ty>::lte(self.0, rhs.0)) }
            }

            #[inline(always)]
            fn cmp_gt(self, rhs: Self) -> Self {
                unsafe { Self(Ops::<$engine, $scalar_ty>::gt(self.0, rhs.0)) }
            }

            #[inline(always)]
            fn cmp_gte(self, rhs: Self) -> Self {
                unsafe { Self(Ops::<$engine, $scalar_ty>::gte(self.0, rhs.0)) }
            }

            #[inline(always)]
            fn max(self, rhs: Self) -> Self {
                unsafe { Self(Ops::<$engine, $scalar_ty>::max(self.0, rhs.0)) }
            }

            #[inline(always)]
            fn min(self, rhs: Self) -> Self {
                unsafe { Self(Ops::<$engine, $scalar_ty>::min(self.0, rhs.0)) }
            }

            #[inline(always)]
            fn horizontal_add($self) -> Self::HorizontalAddScalar {
                $($hadd)*
            }
        }
    };
}

macro_rules! impl_simd_int {
    ($engine:ident, $ty:ident, $scalar_ty:ident, |$self:ident| {
        $($hadd:tt)*
    }) => {
        impl SimdInt for $ty {
            #[inline(always)]
            fn shl(self, rhs: i32) -> Self {
                unsafe { Self(Ops::<$engine, $scalar_ty>::shl(self.0, rhs)) }
            }

            #[inline(always)]
            fn shr(self, rhs: i32) -> Self {
                unsafe { Self(Ops::<$engine, $scalar_ty>::shr(self.0, rhs)) }
            }

            #[inline(always)]
            fn shl_const<const BY: i32>(self) -> Self {
                unsafe { Self(Ops::<$engine, $scalar_ty>::shl_const::<BY>(self.0)) }
            }

            #[inline(always)]
            fn shr_const<const BY: i32>(self) -> Self {
                unsafe { Self(Ops::<$engine, $scalar_ty>::shr_const::<BY>(self.0)) }
            }

            #[inline(always)]
            fn horizontal_unsigned_add($self) -> Self::HorizontalAddScalar {
                $($hadd)*
            }

            #[inline(always)]
            fn from_i64(value: i64) -> Self {
                Self::set1(value as $scalar_ty)
            }
        }
    };
}

macro_rules! impl_simd_float {
    ($engine:ident, $ty:ident, $scalar_ty:ident) => {
        impl SimdFloat for $ty {
            #[inline(always)]
            fn div(self, rhs: Self) -> Self {
                unsafe { Self(Ops::<$engine, $scalar_ty>::div(self.0, rhs.0)) }
            }

            #[inline(always)]
            fn ceil(self) -> Self {
                unsafe { Self(Ops::<$engine, $scalar_ty>::ceil(self.0)) }
            }

            #[inline(always)]
            fn floor(self) -> Self {
                unsafe { Self(Ops::<$engine, $scalar_ty>::floor(self.0)) }
            }

            #[inline(always)]
            fn round(self) -> Self {
                unsafe { Self(Ops::<$engine, $scalar_ty>::round(self.0)) }
            }

            #[inline(always)]
            fn fast_ceil(self) -> Self {
                unsafe { Self(Ops::<$engine, $scalar_ty>::fast_ceil(self.0)) }
            }

            #[inline(always)]
            fn fast_floor(self) -> Self {
                unsafe { Self(Ops::<$engine, $scalar_ty>::fast_floor(self.0)) }
            }

            #[inline(always)]
            fn fast_round(self) -> Self {
                unsafe { Self(Ops::<$engine, $scalar_ty>::fast_round(self.0)) }
            }

            #[inline(always)]
            fn mul_add(self, a: Self, b: Self) -> Self {
                unsafe { Self(Ops::<$engine, $scalar_ty>::mul_add(self.0, a.0, b.0)) }
            }

            #[inline(always)]
            fn mul_sub(self, a: Self, b: Self) -> Self {
                unsafe { Self(Ops::<$engine, $scalar_ty>::mul_sub(self.0, a.0, b.0)) }
            }

            #[inline(always)]
            fn neg_mul_add(self, a: Self, b: Self) -> Self {
                unsafe { Self(Ops::<$engine, $scalar_ty>::neg_mul_add(self.0, a.0, b.0)) }
            }

            #[inline(always)]
            fn neg_mul_sub(self, a: Self, b: Self) -> Self {
                unsafe { Self(Ops::<$engine, $scalar_ty>::neg_mul_sub(self.0, a.0, b.0)) }
            }

            #[inline(always)]
            fn sqrt(self) -> Self {
                unsafe { Self(Ops::<$engine, $scalar_ty>::sqrt(self.0)) }
            }

            #[inline(always)]
            fn rsqrt(self) -> Self {
                unsafe { Self(Ops::<$engine, $scalar_ty>::rsqrt(self.0)) }
            }

            #[inline(always)]
            fn from_f64(value: f64) -> Self {
                Self::set1(value as $scalar_ty)
            }
        }
    };
}

macro_rules! impl_i8_simd_type {
    ($engine:ident, $i8_ty:ident, $i16_ty:ident) => {
        impl_simd_base!($engine, $i8_ty, i8, |self| {
            self.partial_horizontal_add()
                .partial_horizontal_add()
                .partial_horizontal_add()
                .partial_horizontal_add()
        });
        impl_simd_int!($engine, $i8_ty, i8, |self| {
            self.partial_horizontal_unsigned_add()
                .partial_horizontal_unsigned_add()
                .partial_horizontal_unsigned_add()
                .partial_horizontal_add()
        });

        impl SimdInt8 for $i8_ty {
            #[inline(always)]
            fn extend_to_i16(self) -> (<Self::Engine as Simd>::Vi16, <Self::Engine as Simd>::Vi16) {
                let (a, b) = unsafe { Ops::<$engine, i8>::extend_i16(self.0) };
                ($i16_ty(a), $i16_ty(b))
            }

            #[inline(always)]
            fn unsigned_extend_to_i16(
                self,
            ) -> (<Self::Engine as Simd>::Vi16, <Self::Engine as Simd>::Vi16) {
                let (a, b) = unsafe { Ops::<$engine, i8>::unsigned_extend_i16(self.0) };
                ($i16_ty(a), $i16_ty(b))
            }

            #[inline(always)]
            fn get_mask(self) -> u32 {
                unsafe { Ops::<$engine, i8>::get_mask(self.0) }
            }
        }
    };
}

macro_rules! impl_i16_simd_type {
    ($engine:ident, $i16_ty:ident, $i32_ty:ident) => {
        impl_simd_base!($engine, $i16_ty, i16, |self| {
            self.partial_horizontal_add()
                .partial_horizontal_add()
                .partial_horizontal_add()
        });
        impl_simd_int!($engine, $i16_ty, i16, |self| {
            self.partial_horizontal_unsigned_add()
                .partial_horizontal_unsigned_add()
                .partial_horizontal_add()
        });

        impl SimdInt16 for $i16_ty {
            #[inline(always)]
            fn extend_to_i32(self) -> (<Self::Engine as Simd>::Vi32, <Self::Engine as Simd>::Vi32) {
                let (a, b) = unsafe { Ops::<$engine, i16>::extend_i32(self.0) };
                ($i32_ty(a), $i32_ty(b))
            }

            #[inline(always)]
            fn unsigned_extend_to_i32(
                self,
            ) -> (<Self::Engine as Simd>::Vi32, <Self::Engine as Simd>::Vi32) {
                let (a, b) = unsafe { Ops::<$engine, i16>::unsigned_extend_i32(self.0) };
                ($i32_ty(a), $i32_ty(b))
            }
        }
    };
}

macro_rules! impl_i32_simd_type {
    ($engine:ident, $i32_ty:ident, $f32_ty:ident, $i64_ty:ident) => {
        impl_simd_base!($engine, $i32_ty, i32, |self| {
            self.partial_horizontal_add().partial_horizontal_add()
        });
        impl_simd_int!($engine, $i32_ty, i32, |self| {
            self.partial_horizontal_unsigned_add()
                .partial_horizontal_add()
        });

        impl SimdInt32 for $i32_ty {
            #[inline(always)]
            fn bitcast_f32(self) -> <Self::Engine as Simd>::Vf32 {
                unsafe { $f32_ty(Ops::<$engine, i32>::bitcast_f32(self.0)) }
            }

            #[inline(always)]
            fn cast_f32(self) -> <Self::Engine as Simd>::Vf32 {
                unsafe { $f32_ty(Ops::<$engine, i32>::cast_f32(self.0)) }
            }

            #[inline(always)]
            fn extend_to_i64(self) -> (<Self::Engine as Simd>::Vi64, <Self::Engine as Simd>::Vi64) {
                let (a, b) = unsafe { Ops::<$engine, i32>::extend_i64(self.0) };
                ($i64_ty(a), $i64_ty(b))
            }

            #[inline(always)]
            fn unsigned_extend_to_i64(
                self,
            ) -> (<Self::Engine as Simd>::Vi64, <Self::Engine as Simd>::Vi64) {
                let (a, b) = unsafe { Ops::<$engine, i32>::unsigned_extend_i64(self.0) };
                ($i64_ty(a), $i64_ty(b))
            }
        }
    };
}

macro_rules! impl_i64_simd_type {
    ($engine:ident, $i64_ty:ident, $f64_ty:ident) => {
        impl_simd_base!($engine, $i64_ty, i64, |self| {
            self.partial_horizontal_add()
        });
        impl_simd_int!($engine, $i64_ty, i64, |self| {
            self.partial_horizontal_add()
        });

        impl SimdInt64 for $i64_ty {
            #[inline(always)]
            fn bitcast_f64(self) -> <Self::Engine as Simd>::Vf64 {
                unsafe { $f64_ty(Ops::<$engine, i64>::bitcast_f64(self.0)) }
            }

            #[inline(always)]
            fn cast_f64(self) -> <Self::Engine as Simd>::Vf64 {
                unsafe { $f64_ty(Ops::<$engine, i64>::cast_f64(self.0)) }
            }

            #[inline(always)]
            fn partial_horizontal_add(self) -> i64 {
                unsafe { Ops::<$engine, i64>::horizontal_add(self.0) }
            }
        }
    };
}

macro_rules! impl_f32_simd_type {
    ($engine:ident, $f32_ty:ident, $i32_ty:ident) => {
        impl_simd_base!($engine, $f32_ty, f32, |self| {
            unsafe { Ops::<$engine, f32>::horizontal_add(self.0) }
        });
        impl_simd_float!($engine, $f32_ty, f32);

        impl SimdFloat32 for $f32_ty {
            #[inline(always)]
            fn bitcast_i32(self) -> <Self::Engine as Simd>::Vi32 {
                unsafe { $i32_ty(Ops::<$engine, f32>::bitcast_i32(self.0)) }
            }

            #[inline(always)]
            fn cast_i32(self) -> <Self::Engine as Simd>::Vi32 {
                unsafe { $i32_ty(Ops::<$engine, f32>::cast_i32(self.0)) }
            }

            #[inline(always)]
            fn fast_inverse(self) -> Self {
                unsafe { Self(Ops::<$engine, f32>::recip(self.0)) }
            }
        }
    };
}

macro_rules! impl_f64_simd_type {
    ($engine:ident, $f64_ty:ident, $i64_ty:ident) => {
        impl_simd_base!($engine, $f64_ty, f64, |self| {
            unsafe { Ops::<$engine, f64>::horizontal_add(self.0) }
        });
        impl_simd_float!($engine, $f64_ty, f64);

        impl SimdFloat64 for $f64_ty {
            #[inline(always)]
            fn bitcast_i64(self) -> <Self::Engine as Simd>::Vi64 {
                unsafe { $i64_ty(Ops::<$engine, f64>::bitcast_i64(self.0)) }
            }

            #[inline(always)]
            fn cast_i64(self) -> <Self::Engine as Simd>::Vi64 {
                unsafe { $i64_ty(Ops::<$engine, f64>::cast_i64(self.0)) }
            }
        }
    };
}
