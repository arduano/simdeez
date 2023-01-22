use super::*;
use core::mem;


pub struct Sse2;
impl Simd for Sse2 {
    type Vi16 = I16x8;
    type Vi32 = I32x4;
    type Vf32 = F32x4;
    type Vf64 = F64x2;
    type Vi64 = I64x2;
    const VF32_WIDTH: usize = 4;
    const VF64_WIDTH: usize = 2;
    const VI16_WIDTH: usize = 8;
    const VI32_WIDTH: usize = 4;
    const VI64_WIDTH: usize = 2;

    #[inline(always)]
    unsafe fn abs_ps(a: Self::Vf32) -> Self::Vf32 {
        let b = _mm_set1_ps(-0.0f32);
        F32x4(_mm_andnot_ps(b, a.0))
    }
    #[inline(always)]
    unsafe fn abs_pd(a: Self::Vf64) -> Self::Vf64 {
        let b = _mm_set1_pd(-0.0f64);
        F64x2(_mm_andnot_pd(b, a.0))
    }
    #[inline(always)]
    unsafe fn mullo_epi16(a: Self::Vi16, b: Self::Vi16) -> Self::Vi16 {
        I16x8(_mm_mullo_epi16(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn andnot_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x4(_mm_andnot_ps(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn andnot_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        F64x2(_mm_andnot_pd(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn andnot_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x4(_mm_andnot_si128(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn andnot_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        I64x2(_mm_andnot_si128(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn blendv_epi32(a: Self::Vi32, b: Self::Vi32, mask: Self::Vi32) -> Self::Vi32 {
        I32x4(_mm_or_si128(
            _mm_andnot_si128(mask.0, a.0),
            _mm_and_si128(mask.0, b.0),
        ))
    }
    #[inline(always)]
    unsafe fn blendv_epi64(a: Self::Vi64, b: Self::Vi64, mask: Self::Vi64) -> Self::Vi64 {
        I64x2(_mm_or_si128(
            _mm_andnot_si128(mask.0, a.0),
            _mm_and_si128(mask.0, b.0),
        ))
    }
    #[inline(always)]
    unsafe fn blendv_ps(a: Self::Vf32, b: Self::Vf32, mask: Self::Vf32) -> Self::Vf32 {
        F32x4(_mm_or_ps(
            _mm_andnot_ps(mask.0, a.0),
            _mm_and_ps(mask.0, b.0),
        ))
    }
    #[inline(always)]
    unsafe fn blendv_pd(a: Self::Vf64, b: Self::Vf64, mask: Self::Vf64) -> Self::Vf64 {
        F64x2(_mm_or_pd(
            _mm_andnot_pd(mask.0, a.0),
            _mm_and_pd(mask.0, b.0),
        ))
    }
    #[inline(always)]
    unsafe fn castps_epi32(a: Self::Vf32) -> Self::Vi32 {
        I32x4(_mm_castps_si128(a.0))
    }
    #[inline(always)]
    unsafe fn castpd_epi64(a: Self::Vf64) -> Self::Vi64 {
        I64x2(_mm_castpd_si128(a.0))
    }
    #[inline(always)]
    unsafe fn castepi32_ps(a: Self::Vi32) -> Self::Vf32 {
        F32x4(_mm_castsi128_ps(a.0))
    }
    #[inline(always)]
    unsafe fn castepi64_pd(a: Self::Vi64) -> Self::Vf64 {
        F64x2(_mm_castsi128_pd(a.0))
    }
    #[inline(always)]
    unsafe fn castps_pd(a: Self::Vf32) -> Self::Vf64 {
        F64x2(_mm_castps_pd(a.0))
    }
    #[inline(always)]
    unsafe fn castpd_ps(a: Self::Vf64) -> Self::Vf32 {
        F32x4(_mm_castpd_ps(a.0))
    }
    #[inline(always)]
    unsafe fn cmpeq_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x4(_mm_cmpeq_epi32(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn cmpneq_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        Self::not_epi32(I32x4(_mm_cmpeq_epi32(a.0, b.0)))
    }
    #[inline(always)]
    unsafe fn cmpge_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        Self::not_epi32(I32x4(_mm_cmpgt_epi32(b.0, a.0)))
    }
    #[inline(always)]
    unsafe fn cmpgt_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x4(_mm_cmpgt_epi32(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn cmple_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        Self::not_epi32(I32x4(_mm_cmplt_epi32(b.0, a.0)))
    }
    #[inline(always)]
    unsafe fn cmplt_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x4(_mm_cmplt_epi32(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn cmpeq_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        let com32 = _mm_cmpeq_epi32(a.0, b.0); // 32 bit compares
        let com32s = _mm_shuffle_epi32(com32, 0xB1); // swap low and high dwords
        let test = _mm_and_si128(com32, com32s); // low & high
        let teste = _mm_srai_epi32(test, 31); // extend sign bit to 32 bits
        I64x2(_mm_shuffle_epi32(teste, 0xF5)) // extend sign bit to 64 bits
    }
    #[inline(always)]
    unsafe fn cmpneq_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        Self::not_epi64(Self::cmpeq_epi64(a, b))
    }
    #[inline(always)]
    unsafe fn cmpge_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        Self::not_epi64(Self::cmplt_epi64(a, b))
    }
    #[inline(always)]
    unsafe fn cmpgt_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        Self::cmplt_epi64(b, a)
    }
    #[inline(always)]
    unsafe fn cmple_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        Self::cmpge_epi64(b, a)
    }
    #[inline(always)]
    unsafe fn cmplt_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        let s = _mm_sub_epi64(a.0, b.0); // a-b

        // a < b if a and b have same sign and s < 0 or (a < 0 and b >= 0)
        // The latter () corrects for overflow
        let axb = _mm_xor_si128(a.0, b.0); // a ^ b
        let anb = _mm_andnot_si128(b.0, a.0); // a & ~b
        let snaxb = _mm_andnot_si128(axb, s); // s & ~(a ^ b)
        let or1 = _mm_or_si128(anb, snaxb); // (a & ~b) | (s & ~(a ^ b))
        let teste = _mm_srai_epi32(or1, 31); // extend sign bit to 32 bits
        I64x2(_mm_shuffle_epi32(teste, 0xF5)) // extend sign bit to 64 bits
    }
    #[inline(always)]
    unsafe fn cmpeq_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x4(_mm_cmpeq_ps(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn cmpneq_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x4(_mm_cmpneq_ps(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn cmpge_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x4(_mm_cmpge_ps(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn cmpgt_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x4(_mm_cmpgt_ps(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn cmple_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x4(_mm_cmple_ps(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn cmplt_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x4(_mm_cmplt_ps(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn cmpeq_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        F64x2(_mm_cmpeq_pd(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn cmpneq_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        F64x2(_mm_cmpneq_pd(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn cmpge_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        F64x2(_mm_cmpge_pd(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn cmpgt_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        F64x2(_mm_cmpgt_pd(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn cmple_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        F64x2(_mm_cmple_pd(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn cmplt_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        F64x2(_mm_cmplt_pd(a.0, b.0))
    }    
    #[inline(always)]
    unsafe fn cvtps_epi32(a: Self::Vf32) -> Self::Vi32 {
        I32x4(_mm_cvtps_epi32(a.0))
    }
    #[inline(always)]
    unsafe fn cvtpd_epi64(a: Self::Vf64) -> Self::Vi64 {
        let x = _mm_add_pd(a.0, _mm_set1_pd(core::mem::transmute::<i64,f64>(0x0018000000000000)));
        I64x2(_mm_sub_epi64(
            _mm_castpd_si128(x),
            _mm_castpd_si128(_mm_set1_pd(core::mem::transmute::<i64,f64>(0x0018000000000000)))
        ))
    }
    #[inline(always)]
    unsafe fn cvtepi32_ps(a: Self::Vi32) -> Self::Vf32 {
        F32x4(_mm_cvtepi32_ps(a.0))
    }
    #[inline(always)]
    unsafe fn cvtepi64_pd(a: Self::Vi64) -> Self::Vf64 {
        let x = _mm_add_epi64(a.0, _mm_castpd_si128(_mm_set1_pd(core::mem::transmute::<i64,f64>(0x0018000000000000))));
        F64x2(_mm_sub_pd(_mm_castsi128_pd(x), _mm_set1_pd(core::mem::transmute::<i64,f64>(0x0018000000000000))))
    }
    #[inline(always)]
    unsafe fn ceil_ps(a: Self::Vf32) -> Self::Vf32 {
        let t1 = _mm_getcsr();
        let t2 = t1 | (2 << 13);
        _mm_setcsr(t2);
        let r = Self::round_ps(a);
        _mm_setcsr(t1);
        r
    }
    #[inline(always)]
    unsafe fn ceil_pd(a: Self::Vf64) -> Self::Vf64 {
        let t1 = _mm_getcsr();
        let t2 = t1 | (2 << 13);
        _mm_setcsr(t2);
        let r = Self::round_pd(a);
        _mm_setcsr(t1);
        r
    }
    #[inline(always)]
    unsafe fn floor_ps(a: Self::Vf32) -> Self::Vf32 {
        let t1 = _mm_getcsr();
        let t2 = t1 | (1 << 13);
        _mm_setcsr(t2);
        let r = Self::round_ps(a);
        _mm_setcsr(t1);
        r
    }
    #[inline(always)]
    unsafe fn floor_pd(a: Self::Vf64) -> Self::Vf64 {
        let t1 = _mm_getcsr();
        let t2 = t1 | (1 << 13);
        _mm_setcsr(t2);
        let r = Self::round_pd(a);
        _mm_setcsr(t1);
        r
    }
    #[inline(always)]
    unsafe fn fast_floor_ps(a: Self::Vf32) -> Self::Vf32 {
        let i = _mm_cvttps_epi32(a.0);
        let fi = _mm_cvtepi32_ps(i);
        let igx = _mm_cmpgt_ps(fi, a.0);
        let j = _mm_and_ps(igx, _mm_set1_ps(1.0));
        F32x4(_mm_sub_ps(fi, j))
    }
    #[inline(always)]
    unsafe fn fast_floor_pd(a: Self::Vf64) -> Self::Vf64 {
        let i = Self::cvtpd_epi64(a);        
        let fi = Self::cvtepi64_pd(i);
        let igx = _mm_cmpgt_pd(fi.0, a.0);
        let j = _mm_and_pd(igx, _mm_set1_pd(1.0));
        F64x2(_mm_sub_pd(fi.0, j))
    }
    #[inline(always)]
    unsafe fn fast_ceil_ps(a: Self::Vf32) -> Self::Vf32 {
        let i = _mm_cvttps_epi32(a.0);
        let fi = _mm_cvtepi32_ps(i);
        let igx = _mm_cmplt_ps(fi, a.0);
        let j = _mm_and_ps(igx, _mm_set1_ps(1.0));
        F32x4(_mm_add_ps(fi, j))
    }
    #[inline(always)]
    unsafe fn fast_round_ps(a: Self::Vf32) -> Self::Vf32 {
        let v0 = _mm_setzero_ps(); //generate the highest value &lt; 2
        let v1 = _mm_cmpeq_ps(v0, v0);
        let v_nearest2 = _mm_castsi128_ps(_mm_srli_epi32(_mm_castps_si128(v1), 2));
        let i = _mm_cvttps_epi32(a.0);
        let a_trunc = _mm_cvtepi32_ps(i); // truncate a
        let rmd = _mm_sub_ps(a.0, a_trunc); // get remainder
        let rmd2 = _mm_mul_ps(rmd, v_nearest2); // mul remainder by near 2 will yield the needed offset
        let rmd2i = _mm_cvttps_epi32(rmd2); // after being truncated of course
        let rmd2_trunc = _mm_cvtepi32_ps(rmd2i);
        F32x4(_mm_add_ps(a_trunc, rmd2_trunc))
    }
    #[inline(always)]
    unsafe fn fmadd_ps(a: Self::Vf32, b: Self::Vf32, c: Self::Vf32) -> Self::Vf32 {
        F32x4(_mm_add_ps(_mm_mul_ps(a.0, b.0), c.0))
    }
    #[inline(always)]
    unsafe fn fnmadd_ps(a: Self::Vf32, b: Self::Vf32, c: Self::Vf32) -> Self::Vf32 {
        F32x4(_mm_add_ps(
            _mm_mul_ps(_mm_set1_ps(-1.0), _mm_mul_ps(a.0, b.0)),
            c.0,
        ))
    }
    #[inline(always)]
    unsafe fn fmadd_pd(a: Self::Vf64, b: Self::Vf64, c: Self::Vf64) -> Self::Vf64 {
        F64x2(_mm_add_pd(_mm_mul_pd(a.0, b.0), c.0))
    }
    #[inline(always)]
    unsafe fn fnmadd_pd(a: Self::Vf64, b: Self::Vf64, c: Self::Vf64) -> Self::Vf64 {
        F64x2(_mm_add_pd(
            _mm_mul_pd(_mm_set1_pd(-1.0), _mm_mul_pd(a.0, b.0)),
            c.0,
        ))
    }
    #[inline(always)]
    unsafe fn fmsub_ps(a: Self::Vf32, b: Self::Vf32, c: Self::Vf32) -> Self::Vf32 {
        F32x4(_mm_sub_ps(_mm_mul_ps(a.0, b.0), c.0))
    }
    #[inline(always)]
    unsafe fn fnmsub_ps(a: Self::Vf32, b: Self::Vf32, c: Self::Vf32) -> Self::Vf32 {
        F32x4(_mm_sub_ps(
            _mm_mul_ps(_mm_set1_ps(-1.0), _mm_mul_ps(a.0, b.0)),
            c.0,
        ))
    }
    #[inline(always)]
    unsafe fn fmsub_pd(a: Self::Vf64, b: Self::Vf64, c: Self::Vf64) -> Self::Vf64 {
        F64x2(_mm_sub_pd(_mm_mul_pd(a.0, b.0), c.0))
    }
    #[inline(always)]
    unsafe fn fnmsub_pd(a: Self::Vf64, b: Self::Vf64, c: Self::Vf64) -> Self::Vf64 {
        F64x2(_mm_sub_pd(
            _mm_mul_pd(_mm_set1_pd(-1.0), _mm_mul_pd(a.0, b.0)),
            c.0,
        ))
    }
    #[inline(always)]
    unsafe fn horizontal_add_ps(a: Self::Vf32) -> f32 {
        let t1 = _mm_movehl_ps(a.0, a.0);
        let t2 = _mm_add_ps(a.0, t1);
        let t3 = _mm_shuffle_ps(t2, t2, 1);
        _mm_cvtss_f32(_mm_add_ss(t2, t3))
    }
    #[inline(always)]
    unsafe fn horizontal_add_pd(a: Self::Vf64) -> f64 {
        let t0 = _mm_castpd_ps(a.0);
        let t1 = _mm_castps_pd(_mm_movehl_ps(t0, t0));
        let t2 = _mm_add_sd(a.0, t1);
        _mm_cvtsd_f64(t2)
    }
    #[inline(always)]
    unsafe fn i32gather_epi32(arr: &[i32], index: Self::Vi32) -> Self::Vi32 {
        let index_as_arr = mem::transmute::<I32x4, [i32; 4]>(index);
        I32x4(_mm_set_epi32(
            arr[index_as_arr[3] as usize],
            arr[index_as_arr[2] as usize],
            arr[index_as_arr[1] as usize],
            arr[index_as_arr[0] as usize],
        ))
    }
    #[inline(always)]
    unsafe fn i64gather_epi64(arr: &[i64], index: Self::Vi64) -> Self::Vi64 {
        let index_as_arr = mem::transmute::<I64x2, [i64; 2]>(index);
        I64x2(_mm_set_epi64x(            
            arr[index_as_arr[1] as usize],
            arr[index_as_arr[0] as usize],
        ))
    }
    #[inline(always)]
    unsafe fn i32gather_ps(arr: &[f32], index: Self::Vi32) -> Self::Vf32 {
        let index_as_arr = mem::transmute::<I32x4, [i32; 4]>(index);
        F32x4(_mm_set_ps(
            arr[index_as_arr[3] as usize],
            arr[index_as_arr[2] as usize],
            arr[index_as_arr[1] as usize],
            arr[index_as_arr[0] as usize],
        ))
    }
    #[inline(always)]
    unsafe fn load_pd(a: &f64) -> Self::Vf64 {
        F64x2(_mm_load_pd(a as *const f64))
    }
    #[inline(always)]
    unsafe fn load_ps(a: &f32) -> Self::Vf32 {
        F32x4(_mm_load_ps(a as *const f32))
    }
    #[inline(always)]
    unsafe fn load_epi16(a: &i16) -> Self::Vi16 {
        let m = mem::transmute::<&i16, &__m128i>(a);
        I16x8(_mm_load_si128(m))
    }
    #[inline(always)]
    unsafe fn load_epi32(a: &i32) -> Self::Vi32 {
        let m = mem::transmute::<&i32, &__m128i>(a);
        I32x4(_mm_load_si128(m))
    }
    #[inline(always)]
    unsafe fn load_epi64(a: &i64) -> Self::Vi64 {
        let m = mem::transmute::<&i64, &__m128i>(a);
        I64x2(_mm_load_si128(m))
    }
    #[inline(always)]
    unsafe fn loadu_pd(a: &f64) -> Self::Vf64 {
        F64x2(_mm_loadu_pd(a as *const f64))
    }
    #[inline(always)]
    unsafe fn loadu_ps(a: &f32) -> Self::Vf32 {
        F32x4(_mm_loadu_ps(a as *const f32))
    }
    #[inline(always)]
    unsafe fn loadu_epi32(a: &i32) -> Self::Vi32 {
        let m = mem::transmute::<&i32, &__m128i>(a);
        I32x4(_mm_loadu_si128(m))
    }
    #[inline(always)]
    unsafe fn loadu_epi64(a: &i64) -> Self::Vi64 {
        let m = mem::transmute::<&i64, &__m128i>(a);
        I64x2(_mm_loadu_si128(m))
    }
    #[inline(always)]
    unsafe fn maskload_epi32(mem_addr: &i32, mask: Self::Vi32) -> Self::Vi32 {
        let mut result = I32x4(_mm_setzero_si128());
        let ptr = mem_addr as *const i32;
        result[0] = if mask[0] != 0 { *ptr } else { 0 };
        result[1] = if mask[1] != 0 { *ptr.offset(1) } else { 0 };
        result[2] = if mask[2] != 0 { *ptr.offset(2) } else { 0 };
        result[3] = if mask[3] != 0 { *ptr.offset(3) } else { 0 };
        result
    }
    #[inline(always)]
    unsafe fn maskload_epi64(mem_addr: &i64, mask: Self::Vi64) -> Self::Vi64 {
        let mut result = I64x2(_mm_setzero_si128());
        let ptr = mem_addr as *const i64;
        result[0] = if mask[0] != 0 { *ptr } else { 0 };
        result[1] = if mask[1] != 0 { *ptr.offset(1) } else { 0 };
        result
    }
    #[inline(always)]
    unsafe fn maskload_ps(mem_addr: &f32, mask: Self::Vi32) -> Self::Vf32 {
        let mut result = F32x4(_mm_setzero_ps());
        let ptr = mem_addr as *const f32;
        result[0] = if mask[0] != 0 { *ptr } else { 0.0 };
        result[1] = if mask[1] != 0 { *ptr.offset(1) } else { 0.0 };
        result[2] = if mask[2] != 0 { *ptr.offset(2) } else { 0.0 };
        result[3] = if mask[3] != 0 { *ptr.offset(3) } else { 0.0 };
        result
    }
    #[inline(always)]
    unsafe fn maskload_pd(mem_addr: &f64, mask: Self::Vi64) -> Self::Vf64 {
        let mut result = F64x2(_mm_setzero_pd());
        let ptr = mem_addr as *const f64;
        result[0] = if mask[0] != 0 { *ptr } else { 0.0 };
        result[1] = if mask[1] != 0 { *ptr.offset(1) } else { 0.0 };
        result
    }
    #[inline(always)]
    unsafe fn store_ps(mem_addr: &mut f32, a: Self::Vf32) {
        _mm_store_ps(mem_addr as *mut f32, a.0);
    }
    #[inline(always)]
    unsafe fn store_pd(mem_addr: &mut f64, a: Self::Vf64) {
        _mm_store_pd(mem_addr as *mut f64, a.0);
    }
    #[inline(always)]
    unsafe fn store_epi32(mem_addr: &mut i32, a: Self::Vi32) {
        let mem_addr_128 = mem::transmute::<&mut i32, &mut __m128i>(mem_addr);
        _mm_store_si128(mem_addr_128, a.0);
    }
    #[inline(always)]
    unsafe fn store_epi64(mem_addr: &mut i64, a: Self::Vi64) {
        let mem_addr_128 = mem::transmute::<&mut i64, &mut __m128i>(mem_addr);
        _mm_store_si128(mem_addr_128, a.0);
    }
    #[inline(always)]
    unsafe fn storeu_ps(mem_addr: &mut f32, a: Self::Vf32) {
        _mm_storeu_ps(mem_addr as *mut f32, a.0);
    }
    #[inline(always)]
    unsafe fn storeu_pd(mem_addr: &mut f64, a: Self::Vf64) {
        _mm_storeu_pd(mem_addr as *mut f64, a.0);
    }
    #[inline(always)]
    unsafe fn storeu_epi32(mem_addr: &mut i32, a: Self::Vi32) {
        let mem_addr_128 = mem::transmute::<&mut i32, &mut __m128i>(mem_addr);
        _mm_storeu_si128(mem_addr_128, a.0);
    }
    #[inline(always)]
    unsafe fn storeu_epi64(mem_addr: &mut i64, a: Self::Vi64) {
        let mem_addr_128 = mem::transmute::<&mut i64, &mut __m128i>(mem_addr);
        _mm_storeu_si128(mem_addr_128, a.0);
    }
    #[inline(always)]
    unsafe fn maskstore_epi32(mem_addr: &mut i32, mask: Self::Vi32, a: Self::Vi32) {
        let ptr = mem_addr as *mut i32;
        if mask[0] != 0 {
            *ptr = a[0]
        };
        if mask[1] != 0 {
            *ptr.offset(1) = a[1]
        };
        if mask[2] != 0 {
            *ptr.offset(2) = a[2]
        };
        if mask[3] != 0 {
            *ptr.offset(3) = a[3]
        };
    }
    #[inline(always)]
    unsafe fn maskstore_epi64(mem_addr: &mut i64, mask: Self::Vi64, a: Self::Vi64) {
        let ptr = mem_addr as *mut i64;
        if mask[0] != 0 {
            *ptr = a[0]
        };
        if mask[1] != 0 {
            *ptr.offset(1) = a[1]
        };
    }
    #[inline(always)]
    unsafe fn maskstore_ps(mem_addr: &mut f32, mask: Self::Vi32, a: Self::Vf32) {
        let ptr = mem_addr as *mut f32;
        if mask[0] != 0 {
            *ptr = a[0]
        };
        if mask[1] != 0 {
            *ptr.offset(1) = a[1]
        };
        if mask[2] != 0 {
            *ptr.offset(2) = a[2]
        };
        if mask[3] != 0 {
            *ptr.offset(3) = a[3]
        };
    }
    #[inline(always)]
    unsafe fn maskstore_pd(mem_addr: &mut f64, mask: Self::Vi64, a: Self::Vf64) {
        let ptr = mem_addr as *mut f64;
        if mask[0] != 0 {
            *ptr = a[0]
        };
        if mask[1] != 0 {
            *ptr.offset(1) = a[1]
        };
    }
    #[inline(always)]
    unsafe fn max_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        let gt = _mm_cmpgt_epi32(a.0, b.0);
        Self::blendv_epi32(b, a, I32x4(gt))
    }
    #[inline(always)]
    unsafe fn min_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        let gt = _mm_cmpgt_epi32(a.0, b.0);
        Self::blendv_epi32(a, b, I32x4(gt))
    }
    #[inline(always)]
    unsafe fn max_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x4(_mm_max_ps(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn min_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x4(_mm_min_ps(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn max_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        F64x2(_mm_max_pd(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn min_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        F64x2(_mm_min_pd(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn mullo_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        let tmp1 = _mm_mul_epu32(a.0, b.0); /* mul 2,0*/
        let tmp2 = _mm_mul_epu32(_mm_srli_si128(a.0, 4), _mm_srli_si128(b.0, 4)); /* mul 3,1 */
        I32x4(_mm_unpacklo_epi32(
            _mm_shuffle_epi32(tmp1, mm_shuffle!(0, 0, 2, 0) as i32),
            _mm_shuffle_epi32(tmp2, mm_shuffle!(0, 0, 2, 0) as i32),
        )) /* shuffle results to [63..0] and pack */
    }
    #[inline(always)]
    unsafe fn mullo_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        let mut result = Self::setzero_epi64();
        result[0] = a[0]*b[0];
        result[1] = a[1]*b[1];
        result
    }
    #[inline(always)]
    unsafe fn rcp_ps(a: Self::Vf32) -> Self::Vf32 {
        F32x4(_mm_rcp_ps(a.0))
    }
    #[inline(always)]
    unsafe fn round_ps(a: Self::Vf32) -> Self::Vf32 {
        let sign_mask = _mm_set1_ps(-0.0);
        let magic = _mm_castsi128_ps(_mm_set1_epi32(0x4B000000));
        let sign = _mm_and_ps(a.0, sign_mask);
        let signed_magic = _mm_or_ps(magic, sign);
        let b = _mm_add_ps(a.0, signed_magic);
        F32x4(_mm_sub_ps(b, signed_magic))
    }
    #[inline(always)]
    unsafe fn round_pd(a: Self::Vf64) -> Self::Vf64 {
        let sign_mask = _mm_set1_pd(-0.0);
        let magic = _mm_castsi128_pd(_mm_set_epi32(0, 0x43300000, 0, 0x43300000));
        let sign = _mm_and_pd(a.0, sign_mask);
        let signedmagic = _mm_or_pd(magic, sign);
        let b = _mm_add_pd(a.0, signedmagic);
        F64x2(_mm_sub_pd(b, signedmagic))
    }
    #[inline(always)]
    unsafe fn set1_epi32(a: i32) -> Self::Vi32 {
        I32x4(_mm_set1_epi32(a))
    }
    #[inline(always)]
    unsafe fn set1_epi64(a: i64) -> Self::Vi64 {              
        I64x2(_mm_set1_epi64x(a))
    }
   #[inline(always)]
    unsafe fn set1_pd(a: f64) -> Self::Vf64 {
        F64x2(_mm_set1_pd(a))
    }
    #[inline(always)]
    unsafe fn set1_ps(a: f32) -> Self::Vf32 {
        F32x4(_mm_set1_ps(a))
    }
    #[inline(always)]
    unsafe fn setzero_pd() -> Self::Vf64 {
        F64x2(_mm_setzero_pd())
    }
    #[inline(always)]
    unsafe fn setzero_ps() -> Self::Vf32 {
        F32x4(_mm_setzero_ps())
    }
    #[inline(always)]
    unsafe fn setzero_epi32() -> Self::Vi32 {
        I32x4(_mm_setzero_si128())
    }
    #[inline(always)]
    unsafe fn setzero_epi64() -> Self::Vi64 {
        I64x2(_mm_setzero_si128())
    }
    #[inline(always)]
    unsafe fn srai_epi64(a: Self::Vi64, amt_const: i32) -> Self::Vi64 {
        // instruction does not exist. Split into 32-bit shifts
        if amt_const <= 32 {
            let bb = _mm_cvtsi32_si128(amt_const); // b
            let sra = _mm_sra_epi32(a.0, bb); // a >> b signed dwords
            let srl = _mm_srl_epi64(a.0, bb); // a >> b unsigned qwords
            let mask = _mm_setr_epi32(0, -1, 0, -1); // mask for signed high part
            Self::blendv_epi64(I64x2(srl), I64x2(sra), I64x2(mask))
        } else {
            // b > 32
            let bm32 = _mm_cvtsi32_si128(amt_const - 32); // b - 32
            let sign = _mm_srai_epi32(a.0, 31); // sign of a
            let sra2 = _mm_sra_epi32(a.0, bm32); // a >> (b-32) signed dwords
            let sra3 = _mm_srli_epi64(sra2, 32); // a >> (b-32) >> 32 (second shift unsigned qword)
            let mask = _mm_setr_epi32(0, -1, 0, -1); // mask for high part containing only sign
            Self::blendv_epi64(I64x2(sra3), I64x2(sign), I64x2(mask))
        }
    }
    #[inline(always)]
    unsafe fn srli_epi32(a: Self::Vi32, amt_const: i32) -> Self::Vi32 {
        macro_rules! call {
            ($amt_const:expr) => {
                I32x4(_mm_srli_epi32(a.0, $amt_const))
            };
        }
        constify_imm8!(amt_const, call)
    }
    #[inline(always)]
    unsafe fn slli_epi64(a: Self::Vi64, amt_const: i32) -> Self::Vi64 {
        macro_rules! call {
            ($amt_const:expr) => {
                I64x2(_mm_slli_epi64(a.0, $amt_const))
            };
        }
        constify_imm8!(amt_const, call)
    }
    #[inline(always)]
    unsafe fn sra_epi32(a: Self::Vi32, amt: i32) -> Self::Vi32 {
        I32x4(_mm_sra_epi32(a.0, _mm_set1_epi32(amt)))
    }
    #[inline(always)]
    unsafe fn srl_epi32(a: Self::Vi32, amt: i32) -> Self::Vi32 {
        I32x4(_mm_srl_epi32(a.0, _mm_set1_epi32(amt)))
    }
    #[inline(always)]
    unsafe fn sll_epi32(a: Self::Vi32, amt: i32) -> Self::Vi32 {
        I32x4(_mm_sll_epi32(a.0, _mm_set1_epi32(amt)))
    }

    #[inline(always)]
    unsafe fn sqrt_ps(a: Self::Vf32) -> Self::Vf32 {
        F32x4(_mm_sqrt_ps(a.0))
    }
    #[inline(always)]
    unsafe fn rsqrt_ps(a: Self::Vf32) -> Self::Vf32 {
        F32x4(_mm_rsqrt_ps(a.0))
    }
    #[inline(always)]
    unsafe fn sqrt_pd(a: Self::Vf64) -> Self::Vf64 {
        F64x2(_mm_sqrt_pd(a.0))
    }
    #[inline(always)]
    unsafe fn rsqrt_pd(a: Self::Vf64) -> Self::Vf64 {
        F64x2(_mm_div_pd(_mm_set1_pd(1.0), _mm_sqrt_pd(a.0)))
    }
    #[inline(always)]
    unsafe fn shuffle_epi32(a: Self::Vi32, imm8: i32) -> Self::Vi32 {
        macro_rules! call {
            ($imm8:expr) => {
                I32x4(_mm_shuffle_epi32(a.0, $imm8))
            };
        }
        constify_imm8!(imm8, call)
    }
    cfg_if::cfg_if! {
        if #[cfg(feature = "sleef")] {
            #[inline(always)]
            unsafe fn sin_ps(a: Self::Vf32) -> Self::Vf32 {
                F32x4(sleef_sys::Sleef_sinf4_u10sse2(a.0))
            }
            #[inline(always)]
            unsafe fn cos_ps(a: Self::Vf32) -> Self::Vf32 {
                F32x4(sleef_sys::Sleef_cosf4_u10sse2(a.0))
            }
            #[inline(always)]
            unsafe fn fast_sin_ps(a: Self::Vf32) -> Self::Vf32 {
                F32x4(sleef_sys::Sleef_sinf4_u35sse2(a.0))
            }
            #[inline(always)]
            unsafe fn fast_cos_ps(a: Self::Vf32) -> Self::Vf32 {
                F32x4(sleef_sys::Sleef_cosf4_u35sse2(a.0))
            }
            #[inline(always)]
            unsafe fn asin_ps(a: Self::Vf32) -> Self::Vf32 {
                F32x4(sleef_sys::Sleef_asinf4_u10sse2(a.0))
            }
            #[inline(always)]
            unsafe fn acos_ps(a: Self::Vf32) -> Self::Vf32 {
                F32x4(sleef_sys::Sleef_acosf4_u10sse2(a.0))
            }
            #[inline(always)]
            unsafe fn fast_asin_ps(a: Self::Vf32) -> Self::Vf32 {
                F32x4(sleef_sys::Sleef_asinf4_u35sse2(a.0))
            }
            #[inline(always)]
            unsafe fn fast_acos_ps(a: Self::Vf32) -> Self::Vf32 {
                F32x4(sleef_sys::Sleef_acosf4_u35sse2(a.0))
            }
            #[inline(always)]
            unsafe fn tan_ps(a: Self::Vf32) -> Self::Vf32 {
                F32x4(sleef_sys::Sleef_tanf4_u10sse2(a.0))
            }
            #[inline(always)]
            unsafe fn fast_tan_ps(a: Self::Vf32) -> Self::Vf32 {
                F32x4(sleef_sys::Sleef_tanf4_u35sse2(a.0))
            }
            #[inline(always)]
            unsafe fn atan_ps(a: Self::Vf32) -> Self::Vf32 {
                F32x4(sleef_sys::Sleef_atanf4_u10sse2(a.0))
            }
            #[inline(always)]
            unsafe fn fast_atan_ps(a: Self::Vf32) -> Self::Vf32 {
                F32x4(sleef_sys::Sleef_atanf4_u35sse2(a.0))
            }
            // hyperbolic
            #[inline(always)]
            unsafe fn sinh_ps(a: Self::Vf32) -> Self::Vf32 {
                F32x4(sleef_sys::Sleef_sinhf4_u10sse2(a.0))
            }
            #[inline(always)]
            unsafe fn cosh_ps(a: Self::Vf32) -> Self::Vf32 {
                F32x4(sleef_sys::Sleef_coshf4_u10sse2(a.0))
            }
            #[inline(always)]
            unsafe fn fast_sinh_ps(a: Self::Vf32) -> Self::Vf32 {
                F32x4(sleef_sys::Sleef_sinhf4_u35sse2(a.0))
            }
            #[inline(always)]
            unsafe fn fast_cosh_ps(a: Self::Vf32) -> Self::Vf32 {
                F32x4(sleef_sys::Sleef_coshf4_u35sse2(a.0))
            }
            #[inline(always)]
            unsafe fn asinh_ps(a: Self::Vf32) -> Self::Vf32 {
                F32x4(sleef_sys::Sleef_asinhf4_u10sse2(a.0))
            }
            #[inline(always)]
            unsafe fn acosh_ps(a: Self::Vf32) -> Self::Vf32 {
                F32x4(sleef_sys::Sleef_acoshf4_u10sse2(a.0))
            }
            #[inline(always)]
            unsafe fn tanh_ps(a: Self::Vf32) -> Self::Vf32 {
                F32x4(sleef_sys::Sleef_tanhf4_u10sse2(a.0))
            }
            #[inline(always)]
            unsafe fn fast_tanh_ps(a: Self::Vf32) -> Self::Vf32 {
                F32x4(sleef_sys::Sleef_tanhf4_u35sse2(a.0))
            }
            #[inline(always)]
            unsafe fn atanh_ps(a: Self::Vf32) -> Self::Vf32 {
                F32x4(sleef_sys::Sleef_atanhf4_u10sse2(a.0))
            }
            #[inline(always)]
            unsafe fn atan2_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
                F32x4(sleef_sys::Sleef_atan2f4_u10sse2(a.0,b.0))
            }
            #[inline(always)]
            unsafe fn fast_atan2_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
                F32x4(sleef_sys::Sleef_atan2f4_u35sse2(a.0,b.0))
            }
            #[inline(always)]
            unsafe fn ln_ps(a: Self::Vf32) -> Self::Vf32 {
                F32x4(sleef_sys::Sleef_logf4_u10sse2(a.0))
            }
            #[inline(always)]
            unsafe fn fast_ln_ps(a: Self::Vf32) -> Self::Vf32 {
                F32x4(sleef_sys::Sleef_logf4_u35sse2(a.0))
            }
            #[inline(always)]
            unsafe fn log2_ps(a: Self::Vf32) -> Self::Vf32 {
                F32x4(sleef_sys::Sleef_log2f4_u10sse2(a.0))
            }
            #[inline(always)]
            unsafe fn log10_ps(a: Self::Vf32) -> Self::Vf32 {
                F32x4(sleef_sys::Sleef_log10f4_u10sse2(a.0))
            }
            #[inline(always)]
            unsafe fn hypot_ps(a:Self::Vf32,b:Self::Vf32) -> Self::Vf32 {
                F32x4(sleef_sys::Sleef_hypotf4_u05sse2(a.0,b.0))
            }
            #[inline(always)]
            unsafe fn fast_hypot_ps(a:Self::Vf32,b:Self::Vf32) -> Self::Vf32 {
                F32x4(sleef_sys::Sleef_hypotf4_u35sse2(a.0,b.0))
            }
            #[inline(always)]
            unsafe fn fmod_ps(a:Self::Vf32,b:Self::Vf32) -> Self::Vf32 {
                F32x4(sleef_sys::Sleef_fmodf4_sse2(a.0,b.0))
            }

        }
    }
}

#[cfg(all(test, target_feature = "sse2"))]
mod test {
    use super::*;

    union Converter {
        simd: I64x2,
        i64_2: [i64; 2],
    }

    #[test]
    fn test_sse2_slli_epi64() {
        unsafe {
            let lanes = Sse2::set1_epi64(123456);
            let converter = Converter { simd: Sse2::slli_epi64(lanes, 0) };
            assert_eq!(converter.i64_2, [123456, 123456]);
            let got = Converter { simd: Sse2::slli_epi64(lanes, 3) };
            assert_eq!(got.i64_2, [987648, 987648]);
        }
    }
}
