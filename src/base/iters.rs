use crate::InternalSimdBaseIo;

pub trait SimdIter: InternalSimdBaseIo {
    #[inline(always)]
    fn iter(&self) -> SimdArrayIterator<'_, Self> {
        SimdArrayIterator {
            simd: self,
            index: 0,
        }
    }

    #[inline(always)]
    fn iter_mut(&mut self) -> SimdArrayMutIterator<'_, Self> {
        SimdArrayMutIterator {
            simd: self,
            index: 0,
        }
    }
}

impl<T: InternalSimdBaseIo> SimdIter for T {}

pub struct SimdArrayIterator<'a, S: InternalSimdBaseIo> {
    simd: &'a S,
    index: usize,
}

impl<'a, S: InternalSimdBaseIo> Iterator for SimdArrayIterator<'a, S> {
    type Item = S::Scalar;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= S::WIDTH {
            return None;
        }

        unsafe {
            let value = self.simd.get_unchecked(self.index);
            self.index += 1;
            Some(value)
        }
    }
}

pub struct SimdArrayMutIterator<'a, S: InternalSimdBaseIo> {
    simd: &'a mut S,
    index: usize,
}

impl<'a, S: InternalSimdBaseIo> Iterator for SimdArrayMutIterator<'a, S> {
    type Item = &'a mut S::Scalar;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= S::WIDTH {
            return None;
        }

        unsafe {
            let value = self.simd.get_unchecked_mut(self.index);
            self.index += 1;
            Some(value)
        }
    }
}
