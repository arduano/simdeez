#![allow(dead_code)]
#![allow(non_snake_case)]
//! Some traits for using tuples as function arguments. Mostly copied from warp, with minor modifications.
//! https://github.com/seanmonstar/warp/blob/4e9c4fd6ce238197fd1088061bbc07fa2852cb0f/src/generic.rs
//!
//! This will no longer be necessary once fn_traits goes out of nightly.

#[derive(Debug)]
pub struct Product<H, T: HList>(pub(crate) H, pub(crate) T);

pub trait HList: Sized {
    type Tuple: Tuple<HList = Self>;
}

// Typeclass that tuples can be converted into a Product (or unit ()).
pub trait Tuple: Sized {
    type HList: HList<Tuple = Self>;

    fn hlist(self) -> Self::HList;
}

// Combines Product together.
pub trait Combine<T: HList> {
    type Output: HList;

    fn combine(self, other: T) -> Self::Output;
}

pub trait Func<Args> {
    type Output;

    fn call(&self, args: Args) -> Self::Output;
}

impl HList for () {
    type Tuple = ();
}

impl Tuple for () {
    type HList = ();

    #[inline]
    fn hlist(self) -> Self::HList {}
}

impl<F, R> Func<()> for F
where
    F: Fn() -> R,
{
    type Output = R;

    #[inline]
    fn call(&self, _args: ()) -> Self::Output {
        (*self)()
    }
}

macro_rules! product {
    ($H:expr) => { Product($H, ()) };
    ($H:expr, $($T:expr),*) => { Product($H, product!($($T),*)) };
}

macro_rules! Product {
    ($H:ty) => { Product<$H, ()> };
    ($H:ty, $($T:ty),*) => { Product<$H, Product!($($T),*)> };
}

macro_rules! product_pat {
    ($H:pat) => { Product($H, ()) };
    ($H:pat, $($T:pat),*) => { Product($H, product_pat!($($T),*)) };
}

macro_rules! generics {
    ($type:ident) => {
        impl<$type> HList for Product!($type) {
            type Tuple = ($type,);
        }

        impl<$type> Tuple for ($type,) {
            type HList = Product!($type);
            #[inline]
            fn hlist(self) -> Self::HList {
                product!(self.0)
            }
        }

        impl<F, R, $type> Func<Product!($type)> for F
        where
            F: Fn($type) -> R,
        {
            type Output = R;

            #[inline]
            fn call(&self, args: Product!($type)) -> Self::Output {
                (*self)(args.0)
            }

        }

        impl<F, R, $type> Func<($type,)> for F
        where
            F: Fn($type) -> R,
        {
            type Output = R;

            #[inline]
            fn call(&self, args: ($type,)) -> Self::Output {
                (*self)(args.0)
            }
        }
    };

    ($type1:ident, $( $type:ident ),*) => {
        generics!($( $type ),*);

        impl<$type1, $( $type ),*> HList for Product!($type1, $($type),*) {
            type Tuple = ($type1, $( $type ),*);
        }

        impl<$type1, $( $type ),*> Tuple for ($type1, $($type),*) {
            type HList = Product!($type1, $( $type ),*);

            #[inline]
            fn hlist(self) -> Self::HList {
                #[allow(non_snake_case)]
                let ($type1, $( $type ),*) = self;
                product!($type1, $( $type ),*)
            }
        }

        impl<F, R, $type1, $( $type ),*> Func<Product!($type1, $($type),*)> for F
        where
            F: Fn($type1, $( $type ),*) -> R,
        {
            type Output = R;

            #[inline]
            fn call(&self, args: Product!($type1, $($type),*)) -> Self::Output {
                #[allow(non_snake_case)]
                let product_pat!($type1, $( $type ),*) = args;
                (*self)($type1, $( $type ),*)
            }
        }

        impl<F, R, $type1, $( $type ),*> Func<($type1, $($type),*)> for F
        where
            F: Fn($type1, $( $type ),*) -> R,
        {
            type Output = R;

            #[inline]
            fn call(&self, args: ($type1, $($type),*)) -> Self::Output {
                #[allow(non_snake_case)]
                let ($type1, $( $type ),*) = args;
                (*self)($type1, $( $type ),*)
            }
        }
    };
}

generics! {
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    T8,
    T9,
    T10,
    T11,
    T12,
    T13,
    T14,
    T15,
    T16
}
