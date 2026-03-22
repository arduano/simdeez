use crate::{SimdFloat32, SimdFloat64};

#[inline(always)]
pub(crate) fn unary_f32<V: SimdFloat32>(input: V, f: impl Fn(f32) -> f32) -> V {
    unsafe {
        let mut lanes = input.as_array();
        for i in 0..V::WIDTH {
            lanes[i] = f(lanes[i]);
        }
        V::load_from_ptr_unaligned(&lanes as *const V::ArrayRepresentation as *const f32)
    }
}

#[inline(always)]
pub(crate) fn unary_f64<V: SimdFloat64>(input: V, f: impl Fn(f64) -> f64) -> V {
    unsafe {
        let mut lanes = input.as_array();
        for i in 0..V::WIDTH {
            lanes[i] = f(lanes[i]);
        }
        V::load_from_ptr_unaligned(&lanes as *const V::ArrayRepresentation as *const f64)
    }
}

#[inline(always)]
pub(crate) fn binary_f32<V: SimdFloat32>(lhs: V, rhs: V, f: impl Fn(f32, f32) -> f32) -> V {
    unsafe {
        let lhs_lanes = lhs.as_array();
        let rhs_lanes = rhs.as_array();
        let mut out_lanes = lhs_lanes.clone();
        for i in 0..V::WIDTH {
            out_lanes[i] = f(lhs_lanes[i], rhs_lanes[i]);
        }
        V::load_from_ptr_unaligned(&out_lanes as *const V::ArrayRepresentation as *const f32)
    }
}

#[inline(always)]
pub(crate) fn binary_f64<V: SimdFloat64>(lhs: V, rhs: V, f: impl Fn(f64, f64) -> f64) -> V {
    unsafe {
        let lhs_lanes = lhs.as_array();
        let rhs_lanes = rhs.as_array();
        let mut out_lanes = lhs_lanes.clone();
        for i in 0..V::WIDTH {
            out_lanes[i] = f(lhs_lanes[i], rhs_lanes[i]);
        }
        V::load_from_ptr_unaligned(&out_lanes as *const V::ArrayRepresentation as *const f64)
    }
}
