use std::ops::*;

use crate::units::DimsType;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct UnitDims {
    pub time: DimsType,
    pub length: DimsType,
    pub mass: DimsType,
    pub electric_current: DimsType,
    pub temperature: DimsType,
    pub amount: DimsType,
    pub luminous_intensity: DimsType,
}
impl UnitDims {
    pub const DIMENSIONLESS: Self = UnitDims {
        time: 0,
        length: 0,
        mass: 0,
        electric_current: 0,
        temperature: 0,
        amount: 0,
        luminous_intensity: 0,
    };
    pub const TIME: Self = UnitDims{
        time: 1,
        ..Self::DIMENSIONLESS
    };
    pub const LENGTH: Self = UnitDims{
        length: 1,
        ..Self::DIMENSIONLESS
    };
    pub const MASS: Self = UnitDims{
        mass: 1,
        ..Self::DIMENSIONLESS
    };
    pub const ELECTRIC_CURRENT: Self = UnitDims{
        electric_current: 1,
        ..Self::DIMENSIONLESS
    };
    pub const TEMPERATURE: Self = UnitDims{
        temperature: 1,
        ..Self::DIMENSIONLESS
    };
    pub const AMOUNT: Self = UnitDims{
        amount: 1,
        ..Self::DIMENSIONLESS
    };
    pub const LUMINOUS_INTENSITY: Self = UnitDims{
        luminous_intensity: 1,
        ..Self::DIMENSIONLESS
    };

    pub const fn add(self, rhs: Self) -> Self {
        Self {
            time: self.time + rhs.time,
            length: self.length + rhs.length,
            mass: self.mass + rhs.mass,
            electric_current: self.electric_current + rhs.electric_current,
            temperature: self.temperature + rhs.temperature,
            amount: self.amount + rhs.amount,
            luminous_intensity: self.luminous_intensity + rhs.luminous_intensity,
        }
    }

    pub const fn sub(self, rhs: Self) -> Self {
        Self {
            time: self.time - rhs.time,
            length: self.length - rhs.length,
            mass: self.mass - rhs.mass,
            electric_current: self.electric_current - rhs.electric_current,
            temperature: self.temperature - rhs.temperature,
            amount: self.amount - rhs.amount,
            luminous_intensity: self.luminous_intensity - rhs.luminous_intensity,
        }
    }

    pub const fn assert_eq(self, rhs: Self) -> usize{
        if self.time == rhs.time &&
            self.length == rhs.length &&
            self.mass == rhs.mass &&
            self.electric_current == rhs.electric_current &&
            self.temperature == rhs.temperature &&
            self.amount == rhs.amount &&
            self.luminous_intensity == rhs.luminous_intensity{
            0
        }
        else{
            panic!("Dims not equal!")
        }
    }

    pub const fn convert_for_const(self) -> usize {
        0
    }
}
impl Default for UnitDims {
    fn default() -> Self {
        Self::DIMENSIONLESS
    }
}
impl Add for UnitDims {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.add(rhs)
    }
}
impl AddAssign for UnitDims {
    fn add_assign(&mut self, rhs: Self) {
        self.time += rhs.time;
        self.length += rhs.length;
        self.mass += rhs.mass;
        self.electric_current += rhs.electric_current;
        self.temperature += rhs.temperature;
        self.amount += rhs.amount;
        self.luminous_intensity += rhs.luminous_intensity;
    }
}
impl Sub for UnitDims {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.sub(rhs)
    }
}
impl SubAssign for UnitDims {
    fn sub_assign(&mut self, rhs: Self) {
        self.time -= rhs.time;
        self.length -= rhs.length;
        self.mass -= rhs.mass;
        self.electric_current -= rhs.electric_current;
        self.temperature -= rhs.temperature;
        self.amount -= rhs.amount;
        self.luminous_intensity -= rhs.luminous_intensity;
    }
}
impl Mul<DimsType> for UnitDims {
    type Output = Self;

    fn mul(self, rhs: DimsType) -> Self::Output {
        Self {
            time: self.time * rhs,
            length: self.length * rhs,
            mass: self.mass * rhs,
            electric_current: self.electric_current * rhs,
            temperature: self.temperature * rhs,
            amount: self.amount * rhs,
            luminous_intensity: self.luminous_intensity * rhs,
        }
    }
}
impl MulAssign<DimsType> for UnitDims {
    fn mul_assign(&mut self, rhs: DimsType) {
        self.time *= rhs;
        self.length *= rhs;
        self.mass *= rhs;
        self.electric_current *= rhs;
        self.temperature *= rhs;
        self.amount *= rhs;
        self.luminous_intensity *= rhs;
    }
}
impl Div<DimsType> for UnitDims {
    type Output = Self;

    fn div(self, rhs: DimsType) -> Self::Output {
        Self {
            time: self.time / rhs,
            length: self.length / rhs,
            mass: self.mass / rhs,
            electric_current: self.electric_current / rhs,
            temperature: self.temperature / rhs,
            amount: self.amount / rhs,
            luminous_intensity: self.luminous_intensity / rhs,
        }
    }
}
impl DivAssign<DimsType> for UnitDims {
    fn div_assign(&mut self, rhs: DimsType) {
        self.time /= rhs;
        self.length /= rhs;
        self.mass /= rhs;
        self.electric_current /= rhs;
        self.temperature /= rhs;
        self.amount /= rhs;
        self.luminous_intensity /= rhs;
    }
}
impl Rem<DimsType> for UnitDims {
    type Output = Self;

    fn rem(self, rhs: DimsType) -> Self::Output {
        Self {
            time: self.time % rhs,
            length: self.length % rhs,
            mass: self.mass % rhs,
            electric_current: self.electric_current % rhs,
            temperature: self.temperature % rhs,
            amount: self.amount % rhs,
            luminous_intensity: self.luminous_intensity % rhs,
        }
    }
}
impl RemAssign<DimsType> for UnitDims {
    fn rem_assign(&mut self, rhs: DimsType) {
        self.time %= rhs;
        self.length %= rhs;
        self.mass %= rhs;
        self.electric_current %= rhs;
        self.temperature %= rhs;
        self.amount %= rhs;
        self.luminous_intensity %= rhs;
    }
}
