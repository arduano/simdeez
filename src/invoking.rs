use crate::{engines, Simd};

#[cfg(target_arch = "aarch64")]
use std::arch::is_aarch64_feature_detected;

#[macro_export]
macro_rules! fix_tuple_type {
    (()) => {
        ()
    };
    (($typ:ty)) => {
        ($typ,)
    };
    (($($typ:ty),*)) => {
        (($($typ),*))
    };
}

#[macro_export]
macro_rules! __simd_generate_base {
    ($(#[$meta:meta])* $vis:vis fn $fn_name:ident $(<$($lt:lifetime),+>)? ($($arg:ident:$typ:ty),* ) -> $rt:ty $body:block  ) => {
        simdeez_paste_item! {
            // In order to pass arguments via generics like this, we need to convert the arguments
            // into tuples. This is part of the reason for the mess below.

            #[inline(always)]
            $vis unsafe fn [<__ $fn_name _generic>]<$($($lt,)+)? S: 'static + Simd>(args_tuple: ($($typ,)*)) -> $rt {
                let ($($arg,)*) = args_tuple;
                S::invoke(#[inline(always)] || $body)
            }

            $(#[$meta])*
            #[inline(always)]
            $vis fn [<$fn_name _generic>] <$($($lt),+,)? S: Simd>($($arg:$typ,)*) -> $rt {
                let args_tuple = ($($arg,)*);
                __run_simd_generic::<S, [<__ $fn_name _dispatch_struct>], fix_tuple_type!(($($typ),*)), $rt>(args_tuple)
            }

            #[allow(non_camel_case_types)]
            struct [<__ $fn_name _dispatch_struct>];

            impl$(<$($lt),+>)? __SimdRunner<fix_tuple_type!(($($typ),*)), $rt> for [<__ $fn_name _dispatch_struct>] {
                unsafe fn run<S: Simd>(args_tuple: fix_tuple_type!(($($typ),*))) -> $rt {
                    [<__ $fn_name _generic>]::<S>(args_tuple)
                }
            }
        }
    };
}

#[macro_export]
macro_rules! simd_runtime_generate {
    ($(#[$meta:meta])* $vis:vis fn $fn_name:ident $(<$($lt:lifetime),+>)? ($($arg:ident:$typ:ty),* $(,)? ) -> $rt:ty $body:block  ) => {
        simdeez_paste_item! {
            // In order to pass arguments via generics like this, we need to convert the arguments
            // into tuples. This is part of the reason for the mess below.

            $(#[$meta])*
            #[inline(always)]
            $vis fn $fn_name $(<$($lt),+>)?($($arg:$typ,)*) -> $rt {
                let args_tuple = ($($arg,)*);
                __run_simd_runtime_decide::<[<__ $fn_name _dispatch_struct>], fix_tuple_type!(($($typ),*)), $rt>(args_tuple)
            }

            $(#[$meta])*
            #[inline(always)]
            $vis fn [<$fn_name _scalar>] $(<$($lt),+>)?($($arg:$typ,)*) -> $rt {
                let args_tuple = ($($arg,)*);
                __run_simd_invoke_scalar::<[<__ $fn_name _dispatch_struct>], fix_tuple_type!(($($typ),*)), $rt>(args_tuple)
            }

            __simd_generate_base!($(#[$meta])* $vis fn $fn_name $(<$($lt),+>)? ($($arg:$typ),* ) -> $rt $body);
        }
    };
    ($(#[$meta:meta])* $vis:vis fn $fn_name:ident ($($arg:ident:$typ:ty),* $(,)? ) $body:block  ) => {
        simd_runtime_generate!($(#[$meta])* $vis fn $fn_name ($($arg:$typ),*) -> () $body);
    };
}

#[macro_export]
macro_rules! simd_compiletime_select {
    ($(#[$meta:meta])* $vis:vis fn $fn_name:ident $(<$($lt:lifetime),+>)? ($($arg:ident:$typ:ty),* $(,)? ) -> $rt:ty $body:block  ) => {
        simdeez_paste_item! {
            $(#[$meta])*
            #[inline(always)]
            $vis fn $fn_name $(<$($lt),+>)?($($arg:$typ,)*) -> $rt {
                let args_tuple = ($($arg,)*);
                __run_simd_compiletime_select::<[<__ $fn_name _dispatch_struct>], fix_tuple_type!(($($typ),*)), $rt>(args_tuple)
            }

            $(#[$meta])*
            #[inline(always)]
            $vis fn [<$fn_name _scalar>] $(<$($lt),+>)?($($arg:$typ,)*) -> $rt {
                let args_tuple = ($($arg,)*);
                __run_simd_invoke_scalar::<[<__ $fn_name _dispatch_struct>], fix_tuple_type!(($($typ),*)), $rt>(args_tuple)
            }

            __simd_generate_base!($(#[$meta])* $vis fn $fn_name $(<$($lt),+>)? ($($arg:$typ),* ) -> $rt $body);
        }
    };
    ($(#[$meta:meta])* $vis:vis fn $fn_name:ident ($($arg:ident:$typ:ty),* $(,)? ) $body:block  ) => {
        simd_runtime_generate!($(#[$meta])* $vis fn $fn_name ($($arg:$typ),*) -> () $body);
    };
}

#[macro_export]
macro_rules! simd_unsafe_generate_all {
    ($(#[$meta:meta])* $vis:vis fn $fn_name:ident $(<$($lt:lifetime),+>)? ($($arg:ident:$typ:ty),* $(,)? ) -> $rt:ty $body:block  ) => {
        simdeez_paste_item! {
            $(#[$meta])*
            #[inline(always)]
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            $vis fn [<$fn_name _scalar>] $(<$($lt),+>)?($($arg:$typ,)*) -> $rt {
                let args_tuple = ($($arg,)*);
                __run_simd_invoke_scalar::<[<__ $fn_name _dispatch_struct>], fix_tuple_type!(($($typ),*)), $rt>(args_tuple)
            }

            $(#[$meta])*
            #[inline(always)]
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            $vis unsafe fn [<$fn_name _sse2>] $(<$($lt),+>)?($($arg:$typ,)*) -> $rt {
                let args_tuple = ($($arg,)*);
                __run_simd_invoke_sse2::<[<__ $fn_name _dispatch_struct>], fix_tuple_type!(($($typ),*)), $rt>(args_tuple)
            }

            $(#[$meta])*
            #[inline(always)]
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            $vis unsafe fn [<$fn_name _sse41>] $(<$($lt),+>)?($($arg:$typ,)*) -> $rt {
                let args_tuple = ($($arg,)*);
                __run_simd_invoke_sse41::<[<__ $fn_name _dispatch_struct>], fix_tuple_type!(($($typ),*)), $rt>(args_tuple)
            }

            $(#[$meta])*
            #[inline(always)]
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            $vis unsafe fn [<$fn_name _avx2>] $(<$($lt),+>)?($($arg:$typ,)*) -> $rt {
                let args_tuple = ($($arg,)*);
                __run_simd_invoke_avx2::<[<__ $fn_name _dispatch_struct>], fix_tuple_type!(($($typ),*)), $rt>(args_tuple)
            }

            $(#[$meta])*
            #[inline(always)]
            #[cfg(target_arch = "aarch64")]
            $vis unsafe fn [<$fn_name _neon>] $(<$($lt),+>)?($($arg:$typ,)*) -> $rt {
                let args_tuple = ($($arg,)*);
                __run_simd_invoke_neon::<[<__ $fn_name _dispatch_struct>], fix_tuple_type!(($($typ),*)), $rt>(args_tuple)
            }

            $(#[$meta])*
            #[inline(always)]
            #[cfg(target_arch = "wasm32")]
            $vis unsafe fn [<$fn_name _wasm>] $(<$($lt),+>)?($($arg:$typ,)*) -> $rt {
                let args_tuple = ($($arg,)*);
                __run_simd_invoke_wasm::<[<__ $fn_name _dispatch_struct>], fix_tuple_type!(($($typ),*)), $rt>(args_tuple)
            }

            __simd_generate_base!($(#[$meta])* $vis fn $fn_name $(<$($lt),+>)? ($($arg:$typ),* ) -> $rt $body);
        }
    };
    ($(#[$meta:meta])* $vis:vis fn $fn_name:ident ($($arg:ident:$typ:ty),* $(,)? ) $body:block  ) => {
        simd_runtime_generate!($(#[$meta])* $vis fn $fn_name ($($arg:$typ),*) -> () $body);
    };
}

pub trait __SimdRunner<A, R> {
    unsafe fn run<S: Simd>(args: A) -> R;
}

#[inline(always)]
pub fn __run_simd_runtime_decide<S: __SimdRunner<A, R>, A, R>(args: A) -> R {
    #![allow(unreachable_code)]

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    {
        if is_x86_feature_detected!("avx2") && is_x86_feature_detected!("fma") {
            return unsafe { S::run::<engines::avx2::Avx2>(args) };
        }

        if is_x86_feature_detected!("sse4.1") {
            return unsafe { S::run::<engines::sse41::Sse41>(args) };
        }

        if is_x86_feature_detected!("sse2") {
            return unsafe { S::run::<engines::sse2::Sse2>(args) };
        }
    }

    #[cfg(target_arch = "aarch64")]
    if is_aarch64_feature_detected!("neon") {
        return unsafe { S::run::<engines::neon::Neon>(args) };
    }

    #[cfg(target_arch = "wasm32")]
    {
        // Note: there's currently no way to detect SIMD support in WebAssembly at runtime
        return unsafe { S::run::<engines::wasm32::Wasm>(args) };
    }

    unsafe { S::run::<engines::scalar::Scalar>(args) }
}

#[inline(always)]
pub fn __run_simd_generic<E: Simd, S: __SimdRunner<A, R>, A, R>(args: A) -> R {
    unsafe { S::run::<E>(args) }
}

#[inline(always)]
pub fn __run_simd_compiletime_select<S: __SimdRunner<A, R>, A, R>(args: A) -> R {
    #![allow(unreachable_code)]
    #![allow(clippy::needless_return)]

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    {
        #[cfg(all(target_feature = "avx2", target_feature = "fma"))]
        return unsafe { S::run::<engines::avx2::Avx2>(args) };

        #[cfg(target_feature = "sse4.1")]
        return unsafe { S::run::<engines::sse41::Sse41>(args) };

        #[cfg(target_feature = "sse2")]
        return unsafe { S::run::<engines::sse2::Sse2>(args) };
    }

    #[cfg(target_arch = "aarch64")]
    {
        #[cfg(target_feature = "neon")]
        return unsafe { S::run::<engines::neon::Neon>(args) };
    }

    #[cfg(target_arch = "wasm32")]
    {
        return unsafe { S::run::<engines::wasm32::Wasm>(args) };
    }

    return unsafe { S::run::<engines::scalar::Scalar>(args) };
}

#[inline(always)]
pub fn __run_simd_invoke_scalar<S: __SimdRunner<A, R>, A, R>(args: A) -> R {
    unsafe { S::run::<engines::scalar::Scalar>(args) }
}

#[inline(always)]
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
pub unsafe fn __run_simd_invoke_sse2<S: __SimdRunner<A, R>, A, R>(args: A) -> R {
    unsafe { S::run::<engines::sse2::Sse2>(args) }
}

#[inline(always)]
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
pub unsafe fn __run_simd_invoke_sse41<S: __SimdRunner<A, R>, A, R>(args: A) -> R {
    unsafe { S::run::<engines::sse41::Sse41>(args) }
}

#[inline(always)]
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
pub unsafe fn __run_simd_invoke_avx2<S: __SimdRunner<A, R>, A, R>(args: A) -> R {
    unsafe { S::run::<engines::avx2::Avx2>(args) }
}

#[inline(always)]
#[cfg(target_feature = "neon")]
pub unsafe fn __run_simd_invoke_neon<S: __SimdRunner<A, R>, A, R>(args: A) -> R {
    unsafe { S::run::<engines::neon::Neon>(args) }
}

#[inline(always)]
#[cfg(target_arch = "wasm32")]
pub unsafe fn __run_simd_invoke_wasm<S: __SimdRunner<A, R>, A, R>(args: A) -> R {
    unsafe { S::run::<engines::wasm32::Wasm>(args) }
}

#[macro_export]
macro_rules! simd_invoke {
    ($g:ident, $($r:tt)+) => {
        $g::invoke(
            #[inline(always)]
            || $($r)+
        )
    }
}
