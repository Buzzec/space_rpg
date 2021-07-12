use std::num::*;
use crate::units::DisplayValue;
use crate::units::sealed::Sealed;

pub trait Value: Sealed {}
impl Value for u8 {}
impl Value for u16 {}
impl Value for u32 {}
impl Value for u64 {}
impl Value for u128 {}
impl Value for i8 {}
impl Value for i16 {}
impl Value for i32 {}
impl Value for i64 {}
impl Value for i128 {}
impl Value for f32 {}
impl Value for f64 {}
impl Value for NonZeroU8 {}
impl Value for NonZeroU16 {}
impl Value for NonZeroU32 {}
impl Value for NonZeroU64 {}
impl Value for NonZeroU128 {}
impl Value for NonZeroI8 {}
impl Value for NonZeroI16 {}
impl Value for NonZeroI32 {}
impl Value for NonZeroI64 {}
impl Value for NonZeroI128 {}

impl Sealed for u8 {}
impl Sealed for u16 {}
impl Sealed for u32 {}
impl Sealed for u64 {}
impl Sealed for u128 {}
impl Sealed for i8 {}
impl Sealed for i16 {}
impl Sealed for i32 {}
impl Sealed for i64 {}
impl Sealed for i128 {}
impl Sealed for f32 {}
impl Sealed for f64 {}
impl Sealed for NonZeroU8 {}
impl Sealed for NonZeroU16 {}
impl Sealed for NonZeroU32 {}
impl Sealed for NonZeroU64 {}
impl Sealed for NonZeroU128 {}
impl Sealed for NonZeroI8 {}
impl Sealed for NonZeroI16 {}
impl Sealed for NonZeroI32 {}
impl Sealed for NonZeroI64 {}
impl Sealed for NonZeroI128 {}

pub trait ToDisplayValue{
    fn into_display_value(self) -> DisplayValue;
}
pub trait FromDisplayValue{
    fn from_display_value(value: DisplayValue) -> Self;
}
