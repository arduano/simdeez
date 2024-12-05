use crate::SimdBaseIo;

pub trait SimdIter: SimdBaseIo {
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

impl<T: SimdBaseIo> SimdIter for T {}

pub struct SimdArrayIterator<'a, S: SimdBaseIo> {
    simd: &'a S,
    index: usize,
}

impl<S: SimdBaseIo> Iterator for SimdArrayIterator<'_, S> {
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

pub struct SimdArrayMutIterator<'a, S: SimdBaseIo> {
    simd: &'a mut S,
    index: usize,
}

impl<'a, S: SimdBaseIo> Iterator for SimdArrayMutIterator<'a, S> {
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
