use std::ops::*;

use crate::units::{DisplayValue, StandardUnitImpl, Value};
use crate::units::unit_dims::UnitDims;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct UnitValue<T, const DIMS: UnitDims>(T)
where
    T: Value;
impl<T, const DIMS: UnitDims> UnitValue<T, DIMS>
where
    T: Value,
{
    pub fn new(value: T) -> Self{
        Self(value)
    }

    pub const fn value(&self) -> &T {
        &self.0
    }

    pub const fn value_mut(&mut self) -> &mut T {
        &mut self.0
    }

    pub fn into_inner(self) -> T{
        self.0
    }

    pub fn into_standard_unit<SU>(self, unit: SU) -> DisplayValue
    where
        DisplayValue: From<T>,
        SU: StandardUnitImpl<DIMS>,
    {
        unit.display_repr(self.0)
    }
}
impl<T, const DIMS: UnitDims> From<T> for UnitValue<T, DIMS> where T: Value{
    fn from(from: T) -> Self {
        Self(from)
    }
}
impl<TL, TR, const DIMS: UnitDims> Add<UnitValue<TR, DIMS>> for UnitValue<TL, DIMS>
where
    TL: Value,
    TR: Value,
    TL: Add<TR>,
    TL::Output: Value,
{
    type Output = UnitValue<TL::Output, DIMS>;

    fn add(self, rhs: UnitValue<TR, DIMS>) -> Self::Output {
        UnitValue(self.0 + rhs.0)
    }
}
impl<TL, TR, const DIMS: UnitDims> AddAssign<UnitValue<TR, DIMS>> for UnitValue<TL, DIMS>
where
    TL: Value,
    TR: Value,
    TL: AddAssign<TR>,
{
    fn add_assign(&mut self, rhs: UnitValue<TR, DIMS>) {
        self.0 += rhs.0
    }
}
impl<TL, TR, const DIMS: UnitDims> Sub<UnitValue<TR, DIMS>> for UnitValue<TL, DIMS>
where
    TL: Value,
    TR: Value,
    TL: Sub<TR>,
    TL::Output: Value,
{
    type Output = UnitValue<TL::Output, DIMS>;

    fn sub(self, rhs: UnitValue<TR, DIMS>) -> Self::Output {
        UnitValue(self.0 - rhs.0)
    }
}
impl<TL, TR, const DIMS: UnitDims> SubAssign<UnitValue<TR, DIMS>> for UnitValue<TL, DIMS>
where
    TL: Value,
    TR: Value,
    TL: SubAssign<TR>,
{
    fn sub_assign(&mut self, rhs: UnitValue<TR, DIMS>) {
        self.0 -= rhs.0
    }
}
impl<TL, TR, const DIMS_L: UnitDims, const DIMS_R: UnitDims> Mul<UnitValue<TR, DIMS_R>>
    for UnitValue<TL, DIMS_L>
where
    TL: Value,
    TR: Value,
    TL: Mul<TR>,
    TL::Output: Value,
    [(); DIMS_L.add(DIMS_R).convert_for_const()]: ,
{
    type Output = UnitValue<TL::Output, { DIMS_L.add(DIMS_R) }>;

    fn mul(self, rhs: UnitValue<TR, DIMS_R>) -> Self::Output {
        UnitValue(self.0 * rhs.0)
    }
}
impl<TL, TR, const DIMS: UnitDims> Mul<TR> for UnitValue<TL, DIMS>
where
    TL: Value,
    TR: Value,
    TL: Mul<TR>,
    TL::Output: Value,
{
    type Output = UnitValue<TL::Output, DIMS>;

    fn mul(self, rhs: TR) -> Self::Output {
        UnitValue(self.0 * rhs)
    }
}
impl<TL, TR, const DIMS: UnitDims> MulAssign<UnitValue<TR, { UnitDims::DIMENSIONLESS }>>
    for UnitValue<TL, DIMS>
where
    TL: Value,
    TR: Value,
    TL: MulAssign<TR>,
{
    fn mul_assign(&mut self, rhs: UnitValue<TR, { UnitDims::DIMENSIONLESS }>) {
        self.0 *= rhs.0;
    }
}
impl<TL, TR, const DIMS: UnitDims> MulAssign<TR> for UnitValue<TL, DIMS>
where
    TL: Value,
    TR: Value,
    TL: MulAssign<TR>,
{
    fn mul_assign(&mut self, rhs: TR) {
        self.0 *= rhs;
    }
}
impl<TL, TR, const DIMS_L: UnitDims, const DIMS_R: UnitDims> Div<UnitValue<TR, DIMS_R>>
    for UnitValue<TL, DIMS_L>
where
    TL: Value,
    TR: Value,
    TL: Div<TR>,
    TL::Output: Value,
    [(); DIMS_L.sub(DIMS_R).convert_for_const()]: ,
{
    type Output = UnitValue<TL::Output, { DIMS_L.sub(DIMS_R) }>;

    fn div(self, rhs: UnitValue<TR, DIMS_R>) -> Self::Output {
        UnitValue(self.0 / rhs.0)
    }
}
impl<TL, TR, const DIMS: UnitDims> Div<TR> for UnitValue<TL, DIMS>
where
    TL: Value,
    TR: Value,
    TL: Div<TR>,
    TL::Output: Value,
{
    type Output = UnitValue<TL::Output, DIMS>;

    fn div(self, rhs: TR) -> Self::Output {
        UnitValue(self.0 / rhs)
    }
}
impl<TL, TR, const DIMS: UnitDims> DivAssign<UnitValue<TR, { UnitDims::DIMENSIONLESS }>>
    for UnitValue<TL, DIMS>
where
    TL: Value,
    TR: Value,
    TL: DivAssign<TR>,
{
    fn div_assign(&mut self, rhs: UnitValue<TR, { UnitDims::DIMENSIONLESS }>) {
        self.0 /= rhs.0;
    }
}
impl<TL, TR, const DIMS: UnitDims> DivAssign<TR> for UnitValue<TL, DIMS>
where
    TL: Value,
    TR: Value,
    TL: DivAssign<TR>,
{
    fn div_assign(&mut self, rhs: TR) {
        self.0 /= rhs;
    }
}

#[cfg(test)]
mod test{
    use rand::random;

    use crate::units::unit_dims::UnitDims;
    use crate::units::UnitValue;

    #[test]
    fn value_test(){
        let internal_value: i128 = random();
        let mut value = UnitValue::<_, { UnitDims{
            time: 1,
            length: 2,
            mass: 3,
            electric_current: 4,
            temperature: 5,
            amount: 6,
            luminous_intensity: 7
        } }>::from(internal_value);
        assert_eq!(internal_value, *value.value());
        assert_eq!(internal_value, *value.value_mut());
        assert_eq!(internal_value, value.into_inner());
    }
}
