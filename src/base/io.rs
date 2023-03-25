use crate::SimdConsts;

pub trait SimdBaseIo: SimdConsts {
    /// Transmutes the vector into a array representation defined by `Self::ArrayRepresentation`.
    /// Please don't use this function directly unless necessary.
    #[inline(always)]
    unsafe fn transmute_into_array_ref(&self) -> &Self::ArrayRepresentation {
        core::mem::transmute(self)
    }

    /// Transmutes the vector into a mutable array representation defined by `Self::ArrayRepresentation`.
    /// Please don't use this function directly unless necessary.
    #[inline(always)]
    unsafe fn transmute_into_array_mut(&mut self) -> &mut Self::ArrayRepresentation {
        core::mem::transmute(self)
    }

    /// Initialize a vector with all elements set to zero
    fn zeroes() -> Self;

    /// Initialize a vector with all elements set to the same value
    fn set1(x: Self::Scalar) -> Self;

    unsafe fn load_from_array(array: Self::ArrayRepresentation) -> Self;
    unsafe fn as_array(&self) -> Self::ArrayRepresentation {
        self.transmute_into_array_ref().clone()
    }

    /// Load a vector from an unaligned raw pointer.
    unsafe fn load_from_ptr_unaligned(ptr: *const Self::Scalar) -> Self;
    /// Store a vector to an unaligned raw pointer.
    unsafe fn copy_to_ptr_unaligned(self, ptr: *mut Self::Scalar);

    /// Load a vector from a 32 bit aligned raw pointer.
    unsafe fn load_from_ptr_aligned(ptr: *const Self::Scalar) -> Self;
    /// Store a vector to a 32 bit aligned raw pointer.
    unsafe fn copy_to_ptr_aligned(self, ptr: *mut Self::Scalar);

    unsafe fn underlying_value(self) -> Self::UnderlyingType;
    unsafe fn underlying_value_mut(&mut self) -> &mut Self::UnderlyingType;
    unsafe fn from_underlying_value(value: Self::UnderlyingType) -> Self;

    /// Gets the value at the specified index, without a bounds check.
    #[inline(always)]
    unsafe fn get_unchecked(&self, index: usize) -> Self::Scalar {
        unsafe {
            let underlying_ptr = &self.underlying_value() as *const Self::UnderlyingType;
            let ptr_scalar = underlying_ptr as *mut Self::Scalar;
            let ptr = ptr_scalar.add(index);
            *ptr
        }
    }

    /// Gets the value at the specified index, without a bounds check.
    #[inline(always)]
    unsafe fn get_unchecked_mut<'a>(&mut self, index: usize) -> &'a mut Self::Scalar {
        unsafe {
            let underlying_ptr = self.underlying_value_mut() as *mut Self::UnderlyingType;
            let ptr_scalar = underlying_ptr as *mut Self::Scalar;
            let ptr = ptr_scalar.add(index);
            &mut *ptr
        }
    }

    /// Tries to load from a slice. If the slice is too short, it returns the length of the slice.
    fn load_from_slice_exact(slice: &[Self::Scalar]) -> Result<Self, usize> {
        if slice.len() < Self::WIDTH {
            Err(slice.len())
        } else {
            unsafe { Ok(Self::load_from_ptr_unaligned(slice.as_ptr())) }
        }
    }

    /// Tries to load from a slice. If the slice is too short, it uses zeroes for the remaining values.
    fn load_from_slice(slice: &[Self::Scalar]) -> Self {
        unsafe {
            if slice.len() < Self::WIDTH {
                let mut val = Self::zeroes();
                for (i, s) in slice.iter().copied().enumerate() {
                    let ptr = val.get_unchecked_mut(i);
                    *ptr = s;
                }
                val
            } else {
                Self::load_from_ptr_unaligned(slice.as_ptr())
            }
        }
    }

    /// Tries to copy to a slice. If the slice is too short, it returns the length of the slice.
    fn copy_to_slice_exact(self, slice: &mut [Self::Scalar]) -> Result<(), usize> {
        unsafe {
            if slice.len() < Self::WIDTH {
                Err(slice.len())
            } else {
                self.copy_to_ptr_unaligned(slice.as_mut_ptr());
                Ok(())
            }
        }
    }

    /// Tries to copy to a slice. If the slice is too short, it copies as much as possible until the end of the slice.
    fn copy_to_slice(self, slice: &mut [Self::Scalar]) {
        unsafe {
            if slice.len() < Self::WIDTH {
                for (i, s) in slice.iter_mut().enumerate() {
                    *s = self.get_unchecked(i);
                }
            } else {
                self.copy_to_ptr_unaligned(slice.as_mut_ptr());
            }
        }
    }
}
