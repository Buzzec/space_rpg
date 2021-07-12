use std::num::NonZeroIsize;

use f128::f128;
use lazy_static::lazy_static;

pub use unit_dims::*;
pub use unit_value::*;
pub use value::*;

use crate::units::sealed::Sealed;

mod unit_dims;
mod unit_value;
mod value;

pub type DimsType = isize;
pub type DisplayValue = f128;

pub type TimeUnit = time::Time;
pub type MassUnit = mass::Mass;
pub type LengthUnit = length::Length;
pub type ElectricCurrentUnit = electric_current::ElectricCurrent;

#[derive(Clone, Debug)]
pub struct CompositeUnit {
    pub units: Vec<(Unit, NonZeroIsize)>,
}
impl CompositeUnit {
    pub fn dims(&self) -> UnitDims {
        let mut out = UnitDims::default();
        for (unit, multiplier) in &self.units {
            out += unit.dims() * multiplier.get();
        }
        out
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Unit {
    Standard(StandardUnit),
    Custom {
        dims: UnitDims,
        amount_of_base: DisplayValue,
    },
}
impl Unit {
    pub fn dims(&self) -> UnitDims {
        match self {
            Unit::Standard(standard) => standard.dims(),
            Unit::Custom { dims, .. } => *dims,
        }
    }
}
#[derive(Copy, Clone, Debug)]
pub enum StandardUnit {
    Time(TimeUnit),
    Mass(MassUnit),
    Length(LengthUnit),
    ElectricCurrent(ElectricCurrentUnit),
}
impl StandardUnit {
    pub fn dims(&self) -> UnitDims {
        match self {
            StandardUnit::Time(_) => TimeUnit::DIMS,
            StandardUnit::Mass(_) => MassUnit::DIMS,
            StandardUnit::Length(_) => LengthUnit::DIMS,
            StandardUnit::ElectricCurrent(_) => ElectricCurrentUnit::DIMS,
        }
    }
}

pub trait StandardUnitImpl<const DIMS: UnitDims>: Copy + Sealed{
    const DIMS: UnitDims = DIMS;
    const BASE_UNIT: Self;

    fn amount_of_base(self) -> DisplayValue;
    fn base_repr<T>(self, display_value: DisplayValue) -> T
    where
        T: From<DisplayValue>,
    {
        (display_value * self.amount_of_base()).into()
    }
    fn display_repr<T>(self, base_value: T) -> DisplayValue
    where
        DisplayValue: From<T>,
    {
        DisplayValue::from(base_value) / self.amount_of_base()
    }
}

mod sealed {
    pub trait Sealed{}
}

mod time {
    use super::*;

    lazy_static!{
        static ref NANOSECOND_FACTOR: DisplayValue = f128::from(1);
        static ref MICROSECOND_FACTOR: DisplayValue = f128::from(1000) * *NANOSECOND_FACTOR;
        static ref MILLISECOND_FACTOR: DisplayValue = f128::from(1000) * *MICROSECOND_FACTOR;
        static ref SECOND_FACTOR: DisplayValue = f128::from(1000) * *MILLISECOND_FACTOR;
        static ref MINUTE_FACTOR: DisplayValue = f128::from(60) * *SECOND_FACTOR;
        static ref HOUR_FACTOR: DisplayValue = f128::from(60) * *MINUTE_FACTOR;
        static ref GS_DAY_FACTOR: DisplayValue = f128::from(24) * *HOUR_FACTOR;
        static ref GS_MONTH_FACTOR: DisplayValue = f128::from(30) * *GS_DAY_FACTOR;
        static ref GS_YEAR_FACTOR: DisplayValue = f128::from(12) * *GS_MONTH_FACTOR;
        static ref GS_CENTURY_FACTOR: DisplayValue = f128::from(100) * *GS_YEAR_FACTOR;
        static ref GS_MILLENNIUM_FACTOR: DisplayValue = f128::from(1000) * *GS_YEAR_FACTOR;
    }

    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
    pub enum Time {
        Nanosecond,
        Microsecond,
        Millisecond,
        Second,
        Minute,
        Hour,
        GSDay,
        GSMonth,
        GSYear,
        GSCentury,
        GSMillennium,
    }
    impl Sealed for Time{}
    impl StandardUnitImpl<{ UnitDims::TIME }> for Time {
        const BASE_UNIT: Self = Self::Nanosecond;

        fn amount_of_base(self) -> DisplayValue {
            match self {
                Time::Nanosecond => *NANOSECOND_FACTOR,
                Time::Microsecond => *MICROSECOND_FACTOR,
                Time::Millisecond => *MILLISECOND_FACTOR,
                Time::Second => *SECOND_FACTOR,
                Time::Minute => *MINUTE_FACTOR,
                Time::Hour => *HOUR_FACTOR,
                Time::GSDay => *GS_DAY_FACTOR,
                Time::GSMonth => *GS_MONTH_FACTOR,
                Time::GSYear => *GS_YEAR_FACTOR,
                Time::GSCentury => *GS_CENTURY_FACTOR,
                Time::GSMillennium => *GS_MILLENNIUM_FACTOR,
            }
        }
    }
}
mod length {
    use super::*;

    lazy_static!{
        static ref NANOMETER_FACTOR: DisplayValue = f128::from(1);
        static ref MICROMETER_FACTOR: DisplayValue = f128::from(1000) * *NANOMETER_FACTOR;
        static ref MILLIMETER_FACTOR: DisplayValue = f128::from(1000) * *MICROMETER_FACTOR;
        static ref CENTIMETER_FACTOR: DisplayValue = f128::from(10) * *MILLIMETER_FACTOR;
        static ref DECIMETER_FACTOR: DisplayValue = f128::from(100) * *MILLIMETER_FACTOR;
        static ref METER_FACTOR: DisplayValue = f128::from(1000) * *MILLIMETER_FACTOR;
        static ref KILOMETER_FACTOR: DisplayValue = f128::from(1000) * *METER_FACTOR;
        static ref AU_FACTOR: DisplayValue = f128::from(1.496e+11) * *METER_FACTOR;
        static ref LIGHT_YEAR_FACTOR: DisplayValue = f128::from(9.461e+15) * *METER_FACTOR;
    }

    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
    pub enum Length {
        NanoMeter,
        MicroMeter,
        MilliMeter,
        Centimeter,
        Decimeter,
        Meter,
        Kilometer,
        AU,
        LightYear,
    }
    impl Sealed for Length{}
    impl StandardUnitImpl<{ UnitDims::LENGTH }> for Length {
        const BASE_UNIT: Self = Self::NanoMeter;

        fn amount_of_base(self) -> DisplayValue {
            match self {
                Length::NanoMeter => *NANOMETER_FACTOR,
                Length::MicroMeter => *MICROMETER_FACTOR,
                Length::MilliMeter => *MILLIMETER_FACTOR,
                Length::Centimeter => *CENTIMETER_FACTOR,
                Length::Decimeter => *DECIMETER_FACTOR,
                Length::Meter => *METER_FACTOR,
                Length::Kilometer => *KILOMETER_FACTOR,
                Length::AU => *AU_FACTOR,
                Length::LightYear => *LIGHT_YEAR_FACTOR,
            }
        }
    }
}
mod mass {
    use super::*;

    lazy_static! {
        static ref MILLIGRAM_FACTOR: DisplayValue = f128::from(1);
        static ref GRAM_FACTOR: DisplayValue = f128::from(1000) * *MILLIGRAM_FACTOR;
        static ref KILOGRAM_FACTOR: DisplayValue = f128::from(1000) * *GRAM_FACTOR;
        static ref TONNE_FACTOR: DisplayValue = f128::from(1000) * *KILOGRAM_FACTOR;
        static ref OZ_FACTOR: DisplayValue = f128::from(28.3495) * *GRAM_FACTOR;
        static ref POUND_FACTOR: DisplayValue = f128::from(453.59237) * *GRAM_FACTOR;
        static ref TON_FACTOR: DisplayValue = f128::from(2000) * *POUND_FACTOR;
        static ref EARTH_MASS_FACTOR: DisplayValue = f128::from(5.9722e+24) * *KILOGRAM_FACTOR;
        static ref SOLAR_MASS_FACTOR: DisplayValue = f128::from(1.989e+30) * *KILOGRAM_FACTOR;
    }
    #[allow(clippy::enum_variant_names)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
    pub enum Mass {
        Milligram,
        Gram,
        Kilogram,
        Tonne,
        Oz,
        Pound,
        Ton,
        EarthMass,
        SolarMass,
    }
    impl Sealed for Mass{}
    impl StandardUnitImpl<{ UnitDims::MASS }> for Mass {
        const BASE_UNIT: Self = Self::Milligram;

        fn amount_of_base(self) -> DisplayValue {
            match self {
                Mass::Milligram => *MILLIGRAM_FACTOR,
                Mass::Gram => *GRAM_FACTOR,
                Mass::Kilogram => *KILOGRAM_FACTOR,
                Mass::Tonne => *TONNE_FACTOR,
                Mass::Oz => *OZ_FACTOR,
                Mass::Pound => *POUND_FACTOR,
                Mass::Ton => *TON_FACTOR,
                Mass::EarthMass => *EARTH_MASS_FACTOR,
                Mass::SolarMass => *SOLAR_MASS_FACTOR,
            }
        }
    }
}
mod electric_current {
    use super::*;

    lazy_static! {
        static ref PICOAMPERE_FACTOR: DisplayValue = f128::from(1);
        static ref NANOAMPERE_FACTOR: DisplayValue = f128::from(1000) * *PICOAMPERE_FACTOR;
        static ref MICROAMPERE_FACTOR: DisplayValue = f128::from(1000) * *NANOAMPERE_FACTOR;
        static ref MILLIAMPERE_FACTOR: DisplayValue = f128::from(1000) * *MICROAMPERE_FACTOR;
        static ref AMPERE_FACTOR: DisplayValue = f128::from(1000) * *MILLIAMPERE_FACTOR;
    }

    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
    pub enum ElectricCurrent {
        Picoampere,
        Nanoampere,
        Microampere,
        Milliampere,
        Ampere,
    }
    impl Sealed for ElectricCurrent{}
    impl StandardUnitImpl<{ UnitDims::ELECTRIC_CURRENT }> for ElectricCurrent {
        const BASE_UNIT: Self = Self::Picoampere;

        fn amount_of_base(self) -> DisplayValue {
            match self {
                ElectricCurrent::Picoampere => *PICOAMPERE_FACTOR,
                ElectricCurrent::Nanoampere => *NANOAMPERE_FACTOR,
                ElectricCurrent::Microampere => *MICROAMPERE_FACTOR,
                ElectricCurrent::Milliampere => *MILLIAMPERE_FACTOR,
                ElectricCurrent::Ampere => *AMPERE_FACTOR,
            }
        }
    }
}
