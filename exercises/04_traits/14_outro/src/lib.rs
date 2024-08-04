// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

use std::ops::Add;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SaturatingU16(u16);

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.0 == *other
    }
}

macro_rules! impl_add {
    ($struct_name:ident) => {
        impl_add!($struct_name, $struct_name, |x: $struct_name| x.0);
        impl_add!($struct_name, &$struct_name, |x: &$struct_name| x.0);
    };
    ($struct_name:ident + &$other:ty) => {
        impl_add!($struct_name, &$other, |x: &$other| *x);
    };
    ($struct_name:ident + $other:ty) => {
        impl_add!($struct_name, $other, |x: $other| x);
    };
    ($struct_name:ident, $other:ty, $get_val:expr) => {
        impl Add<$other> for $struct_name {
            type Output = $struct_name;
            fn add(self, rhs: $other) -> Self::Output {
                $struct_name(self.0.saturating_add($get_val(rhs)))
            }
        }
    };
}

impl_add!(SaturatingU16);
impl_add!(SaturatingU16 + u16);
impl_add!(SaturatingU16 + &u16);

macro_rules! impl_from {
    /*($struct_name:ident < [$($from_ty:ty),+]) => {
        $(impl_from! {
            $struct_name < $from_ty
        })+
    };*/
    ($struct_name:ident < &$from_ty:ty) => {
        impl_from!(@from $struct_name < &$from_ty, |f: &$from_ty| *f);
    };
    ($struct_name:ident < $from_ty:ty) => {
        impl_from!(@from $struct_name < $from_ty, |f: $from_ty| f);
    };
    (@from $struct_name:ident < $from_ty:ty, $deref:expr) => {
        impl From<$from_ty> for $struct_name {
            fn from(from_value: $from_ty) -> Self {
                $struct_name($deref(from_value).into())
            }
        }
    };
}

impl_from!(SaturatingU16 < u16);
impl_from!(SaturatingU16 < u8);
impl_from!(SaturatingU16 < &u16);
impl_from!(SaturatingU16 < &u8);
// impl_from!(SaturatingU16 < [u16, u8, &u16, &u8]); // doesn't work for some reason :(
