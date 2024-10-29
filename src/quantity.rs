use std::{
    fmt::Display,
    marker::PhantomData,
    ops::{
        Add,
        Div,
        Mul,
        Sub,
    },
};

use num::Float;

use crate::{
    dimension::{
        si::NoDim,
        DimInverse,
        DimNthRoot,
        DimPow,
        Dimension,
    },
    kind::{
        AngleKind,
        KindDiv,
        KindMul,
    },
    unit::{
        QuantityConversion,
        Unit,
        UnitFormatter,
    },
    value_type::ValueType,
};

/// A helper trait to quickly be able to access the usually templated types of Quantities.
pub trait QuantityTypeInfo {
    type DataType;
    type Dimension;
    type Kind;
}

/// Represent a specific quantity stored
///
/// `DataType` is the actual stored value
/// Storage always happens in SI Base units. So if you put a kilometer into a Length, it's stored as 1000 meters.
///
/// `Dimension` represents the dimension of the type and is the first level of restriction of quantity operations on each other
///
/// `Kind` - ISO 80000-1:2009 has this down quite nicely, to paraphrase:
///
///   * Units of different dimension are always of a different kind and cannot trivially be combined
///   * Units of matching dimensions can be of different kind. Those *can* have rules for conversion between them, but not necessarily do so.
///   * Units of matching dimension and kind are treated the same
///
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Quantity<DataType, Dim, K>
where
    DataType: ValueType,
    Dim: Dimension,
{
    #[cfg_attr(feature = "serde", serde(skip))]
    _dim: PhantomData<Dim>,
    #[cfg_attr(feature = "serde", serde(skip))]
    _kind: PhantomData<K>,
    value: DataType,
}

impl<DataType, Dim, K> Quantity<DataType, Dim, K>
where
    DataType: ValueType,
    Dim: Dimension,
{
    /// Create a new Quantity in its base representation
    /// Take care that Kind may influence what that actually means
    pub fn new_base(value: DataType) -> Self {
        Self {
            _dim: PhantomData,
            _kind: PhantomData,
            value,
        }
    }

    /// Create a new quantity from the given unit
    pub fn new<Unit>(value: DataType) -> Self
    where
        Unit: QuantityConversion<DataType, Dim, K>,
    {
        Self::new_base(Unit::convert_to_base(value))
    }

    /// Remove the dimensional restriction from this quantity.
    ///
    /// Use this with care. Sometimes it will be unavlidable, as the library may not have a specific conversion implemented.
    /// However, you loose the restriction kind puts on the Dimensional quantity.
    ///
    /// To give an example, 1 rad / s with AngleKind restriction becomes 1 hertz as Frequency (as frequency is unrestricted) due to this.
    /// For the correct conversion, you would grab this quantity as hertz using get(), which would yield you the proper to 1/(2pi) * hz
    pub fn unrestricted(&self) -> Quantity<DataType, Dim, ()> {
        Quantity {
            _dim: PhantomData,
            _kind: PhantomData,
            value: self.value,
        }
    }

    /// Access this Quantity as unit of type Unit
    pub fn get<Unit>(&self) -> DataType
    where
        Unit: QuantityConversion<DataType, Dim, K>,
    {
        Unit::convert_from_base(self.value)
    }
}

impl<DataType, Dim, K> Quantity<DataType, Dim, K>
where
    DataType: ValueType + Display,
    Dim: Dimension,
{
    /// Format this unit using the formatting for a given unit instead of formatting in its base unit.
    pub fn formatted<UnitType>(&self) -> UnitFormatter<DataType, UnitType>
    where
        UnitType: Unit + QuantityConversion<DataType, Dim, K>,
    {
        UnitFormatter::new(self.value)
    }
}

/// Addition on quantities is only defined if they are equal in all aspects
impl<DataType, Dim, K> Add for Quantity<DataType, Dim, K>
where
    DataType: ValueType,
    Dim: Dimension,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value + rhs.value,
            ..self
        }
    }
}

/// Subtraction on quantities is only defined if they are equal in all aspects
impl<DataType, Dim, K> Sub for Quantity<DataType, Dim, K>
where
    DataType: ValueType,
    Dim: Dimension,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value - rhs.value,
            ..self
        }
    }
}

/// Multiplication of quantities is defined if Dim and Kind can be multiplied
impl<DataType, DimLhs, KLhs, DimRhs, KRhs> Mul<Quantity<DataType, DimRhs, KRhs>>
    for Quantity<DataType, DimLhs, KLhs>
where
    DataType: ValueType,
    DimLhs: Dimension + Mul<DimRhs>,
    DimRhs: Dimension,
    KLhs: KindMul<KRhs>,
    <DimLhs as Mul<DimRhs>>::Output: Dimension,
{
    type Output =
        Quantity<DataType, <DimLhs as Mul<DimRhs>>::Output, <KLhs as KindMul<KRhs>>::Output>;

    fn mul(self, rhs: Quantity<DataType, DimRhs, KRhs>) -> Self::Output {
        Self::Output {
            _dim: PhantomData,
            _kind: PhantomData,
            value: self.value * rhs.value,
        }
    }
}

/// Division of quantities is defined if Dim and Kind can be divided
impl<DataType, DimLhs, KLhs, DimRhs, KRhs> Div<Quantity<DataType, DimRhs, KRhs>>
    for Quantity<DataType, DimLhs, KLhs>
where
    DataType: ValueType,
    DimLhs: Dimension + Div<DimRhs>,
    DimRhs: Dimension,
    KLhs: KindDiv<KRhs>,
    <DimLhs as Div<DimRhs>>::Output: Dimension,
{
    type Output =
        Quantity<DataType, <DimLhs as Div<DimRhs>>::Output, <KLhs as KindDiv<KRhs>>::Output>;

    fn div(self, rhs: Quantity<DataType, DimRhs, KRhs>) -> Self::Output {
        Self::Output {
            _dim: PhantomData,
            _kind: PhantomData,
            value: self.value / rhs.value,
        }
    }
}

impl<T, D, K> QuantityTypeInfo for Quantity<T, D, K>
where
    T: ValueType,
    D: Dimension,
{
    type DataType = T;
    type Dimension = D;
    type Kind = K;
}

// floating point methods impls

macro_rules! producer_methods {
    ($($name:ident,)*) => {
        $(
            pub fn $name() -> Self {
                Self {
                    _dim: PhantomData,
                    _kind: PhantomData,
                    value: T::$name()
                }
            }
        )*
    };
}
macro_rules! no_param_methods {
    ($($name:ident -> $result:ty;)*) => {
        $(
            pub fn $name(self) -> $result {
                Quantity {
                    _dim: PhantomData,
                    _kind: PhantomData,
                    value: self.value.$name()
                }
            }
        )*
    };
}

macro_rules! one_param_methods {
    ($($name:ident -> $result:ty;)*) => {
        $(
            pub fn $name(self, other: Self) -> $result {
                Quantity {
                    _dim: PhantomData,
                    _kind: PhantomData,
                    value: self.value.$name(other.value)
                }
            }
        )*
    };
}

impl<T, D, K> Quantity<T, D, K>
where
    T: ValueType + Float,
    D: Dimension,
{
    producer_methods!(
        infinity,
        max_value,
        min_value,
        min_positive_value,
        nan,
        neg_infinity,
        neg_zero,
    );
    no_param_methods!(
        abs -> Quantity<T,D,K>;
        ceil -> Quantity<T,D,K>;
        floor -> Quantity<T,D,K>;
        fract -> Quantity<T,D,K>;
        round -> Quantity<T,D,K>;
    );
    one_param_methods!(
        abs_sub -> Quantity<T,D,K>;
        max -> Quantity<T,D,K>;
        min -> Quantity<T,D,K>;
    );

    pub fn is_finite(&self) -> bool {
        self.value.is_finite()
    }

    pub fn is_infinite(&self) -> bool {
        self.value.is_infinite()
    }

    pub fn is_nan(&self) -> bool {
        self.value.is_nan()
    }

    pub fn is_normal(&self) -> bool {
        self.value.is_normal()
    }

    pub fn is_sign_negative(&self) -> bool {
        self.value.is_sign_negative()
    }

    pub fn is_sign_positive(&self) -> bool {
        self.value.is_sign_positive()
    }

    pub fn clamp(self, min: Self, max: Self) -> Self {
        Self {
            _dim: Default::default(),
            _kind: Default::default(),
            value: self.value.clamp(min.value, max.value),
        }
    }
}

impl<T, D, K> Quantity<T, D, K>
where
    T: ValueType + Float,
    D: Dimension + DimInverse,
    <D as DimInverse>::Output: Dimension,
{
    pub fn recip(self) -> Quantity<T, <D as DimInverse>::Output, ()> {
        Quantity {
            _dim: Default::default(),
            _kind: Default::default(),
            value: self.value.recip(),
        }
    }
}

impl<T, D, K> Quantity<T, D, K>
where
    T: ValueType + Float,
    D: Dimension,
{
    /// Powi changes units, and for now can only be called using the typenum integer types
    ///
    pub fn powi<Power>(
        self,
        _power: Power,
    ) -> Quantity<T, <D::DimType as DimPow<Power>>::Output, ()>
    where
        Power: typenum::Integer,
        <D as Dimension>::DimType: DimPow<Power>,
        <D::DimType as DimPow<Power>>::Output: Dimension,
    {
        Quantity {
            _dim: Default::default(),
            _kind: Default::default(),
            value: self.value.powi(Power::to_i32()),
        }
    }

    /// nth_root changes units, and for now can only be called using the typenum integer types
    ///
    pub fn nth_root<Power>(
        self,
        _power: Power,
    ) -> Quantity<T, <D::DimType as DimNthRoot<Power>>::Output, ()>
    where
        Power: typenum::Integer,
        <D as Dimension>::DimType: DimNthRoot<Power>,
        <D::DimType as DimNthRoot<Power>>::Output: Dimension,
    {
        Quantity {
            _dim: Default::default(),
            _kind: Default::default(),
            value: self
                .value
                .powf(T::ONE / T::new_from_real_f64(Power::to_i32() as f64)),
        }
    }
}

impl<T, D, K> Quantity<T, D, K>
where
    T: ValueType + Float,
    D: Dimension + DimNthRoot<typenum::P2>,
    <D as DimNthRoot<typenum::P2>>::Output: Dimension,
{
    pub fn sqrt(self) -> Quantity<T, D::Output, ()> {
        Quantity {
            _dim: Default::default(),
            _kind: Default::default(),
            value: self.value.sqrt(),
        }
    }
}

impl<T, D, K> Quantity<T, D, K>
where
    T: ValueType + Float,
    D: Dimension + DimNthRoot<typenum::P3>,
    <D as DimNthRoot<typenum::P3>>::Output: Dimension,
{
    pub fn cbrt(self) -> Quantity<T, D::Output, ()> {
        Quantity {
            _dim: Default::default(),
            _kind: Default::default(),
            value: self.value.cbrt(),
        }
    }
}

impl<T, K> Quantity<T, NoDim, K>
where
    T: ValueType + Float,
{
    no_param_methods!(
        acos -> Quantity<T,NoDim,AngleKind>;
        acosh -> Quantity<T,NoDim,AngleKind>;
        asin -> Quantity<T,NoDim,AngleKind>;
        asinh -> Quantity<T,NoDim,AngleKind>;
        atan -> Quantity<T,NoDim,AngleKind>;
        atanh -> Quantity<T,NoDim,AngleKind>;
        cos -> Quantity<T,NoDim,()>;
        cosh -> Quantity<T,NoDim,()>;
        exp -> Quantity<T,NoDim,()>;
        exp2 -> Quantity<T,NoDim,()>;
        exp_m1 -> Quantity<T,NoDim,()>;
        ln -> Quantity<T,NoDim,()>;
        ln_1p -> Quantity<T,NoDim,()>;
        log10 -> Quantity<T,NoDim,()>;
        log2 -> Quantity<T,NoDim,()>;
    );
    one_param_methods!(
        atan2 -> Quantity<T,NoDim,AngleKind>;
    );

    pub fn log(self, value: T) -> Quantity<T, NoDim, ()> {
        Quantity {
            _dim: Default::default(),
            _kind: Default::default(),
            value: self.value.log(value),
        }
    }
}
