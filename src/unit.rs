use std::{
    fmt::{
        Display,
        Formatter,
    },
    marker::PhantomData,
};

use crate::{
    dimension::Dimension,
    value_type::ValueType,
};

/// A trait for units. Used for formatting
pub trait Unit {
    /// The long name of a unit. This is "Second, Metre, MetrePerSecond"
    fn long_name() -> &'static str;
    /// The short name of a unit. This is "s, m, m/s, ..."
    fn print_name() -> &'static str;
}

/// This unit is a prefix unit.
/// Conversion is automatically derived for it.
pub trait PrefixUnit: Unit {
    /// The unit this prefixes
    type Base: Unit;
    /// return the prefix factor of this unit.
    fn prefix() -> f64;
}

/// Implements conversion between quantities and the units used to build them
pub trait QuantityConversion<DataType, Dim, K>
where
    DataType: ValueType,
    Dim: Dimension,
{
    /// The factor to convert to/from the base unit
    fn factor() -> DataType;

    /// Offset of the unit to/from the base unit. Because of course thats a thing.
    /// The offset is in the given unit, not in base units.
    /// For Fahrenheit and base unit kelvin that would be 459.67
    fn offset() -> DataType {
        DataType::ZERO
    }

    /// Convert to this from the base unit
    fn convert_from_base(base_value: DataType) -> DataType {
        base_value / Self::factor() - Self::offset()
    }

    /// Convert this to the base unit
    fn convert_to_base(my_value: DataType) -> DataType {
        (my_value + Self::offset()) * Self::factor()
    }
}

impl<DataType, Dim, K, U> QuantityConversion<DataType, Dim, K> for U
where
    U: PrefixUnit,
    U::Base: Unit + QuantityConversion<DataType, Dim, K>,
    DataType: ValueType,
    Dim: Dimension,
{
    fn factor() -> DataType {
        U::Base::factor() * DataType::new_from_real_f64(U::prefix())
    }

    fn offset() -> DataType {
        U::Base::offset()
    }
}
/// A helper struct for formatting, as we cannot return `std::fmt::Arguments`.
///
/// This just combines a value in `DataType` with the `UnitType` and describes how it will be formatted.
pub struct UnitFormatter<DataType, UnitType>
where
    DataType: ValueType,
    UnitType: Unit,
{
    _unit: PhantomData<UnitType>,
    value: DataType,
}

impl<DataType, UnitType> UnitFormatter<DataType, UnitType>
where
    DataType: ValueType + Display,
    UnitType: Unit,
{
    /// Create a new UnitFormatter.
    pub fn new(value: DataType) -> Self {
        Self {
            _unit: PhantomData,
            value,
        }
    }
}

impl<DataType, UnitType> Display for UnitFormatter<DataType, UnitType>
where
    DataType: ValueType + Display,
    UnitType: Unit,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(precision) = f.precision() {
            write!(f, "{:.precision$} {}", self.value, UnitType::print_name())
        } else {
            write!(f, "{} {}", self.value, UnitType::print_name())
        }
    }
}
