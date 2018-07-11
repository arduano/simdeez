#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;
use core::mem;
use core::ops::*;


// Newtypes for i32 vectors
// We have to do this to allow for overloading of
// __m128i etc
#[derive(Copy, Debug, Clone)]
pub struct I32x4(pub __m128i);
#[derive(Copy, Debug, Clone)]
pub struct I32x4_41(pub __m128i);
#[derive(Copy, Debug, Clone)]
pub struct I32x8(pub __m256i);

// Newtypes for i64 vectors
// We have to do this to allow for overloading of
// __m128i etc
#[derive(Copy, Debug, Clone)]
pub struct I64x2(pub __m128i);
#[derive(Copy, Debug, Clone)]
pub struct I64x2_41(pub __m128i);
#[derive(Copy, Debug, Clone)]
pub struct I64x4(pub __m256i);

// Newtypes for f32 vectors
// We have to do this to allow for overloading of
// __m128 etc
#[derive(Copy, Debug, Clone)]
pub struct F32x4(pub __m128);
#[derive(Copy, Debug, Clone)]
pub struct F32x8(pub __m256);

// Newtypes for i64 vectors
#[derive(Copy, Debug, Clone)]
pub struct F64x2(pub __m128d);
#[derive(Copy, Debug, Clone)]
pub struct F64x4(pub __m256d);

mod index;
pub use self::index::*;
mod index_mut;
pub use self::index_mut::*;
mod add;
pub use self::add::*;
mod add_assign;
pub use self::add_assign::*;
mod sub;
pub use self::sub::*;
mod sub_assign;
pub use self::sub_assign::*;
mod mul;
pub use self::mul::*;
mod mul_assign;
pub use self::mul_assign::*;
mod div;
pub use self::div::*;
mod div_assign;
pub use self::div_assign::*;
mod and;
pub use self::and::*;
mod and_assign;
pub use self::and_assign::*;
mod or;
pub use self::or::*;
mod or_assign;
pub use self::or_assign::*;
mod xor;
pub use self::xor::*;
mod xor_assign;
pub use self::xor_assign::*;
mod not;
pub use self::not::*;
mod shl;
pub use self::shl::*;
mod shl_assign;
pub use self::shl_assign::*;
mod shr;
pub use self::shr::*;
mod shr_assign;
pub use self::shr_assign::*;

