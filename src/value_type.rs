use std::{
    fmt::{
        Debug,
        Display,
    },
    str::FromStr,
};

use num::{
    complex::{
        c32,
        c64,
        Complex32,
        Complex64,
    },
    traits::{
        ConstOne,
        ConstZero,
        NumOps,
    },
    FromPrimitive,
};

/// A not-so-marker trait for anything that can be a value
/// Real-Valued datatypes should also mark themself with `RealValueType`, and complex datatypes with `ComplexValueType`.
/// Note that this exists mainly to avoid tripple and quadruple implementations of things.
///
/// You will need to put in significant work to make your own complex data types work.
/// Custom *real* data types are probably no problem.
///
/// This trait may grow to actually contain all the complex operations needed, analog to nums `Float` and `FloatCore` etc.
pub trait ValueType:
    Display + FromStr + Debug + FromPrimitive + Copy + NumOps + ConstOne + ConstZero + Sized + 'static
{
    /// Create a purely real-valued of this from a f64. This is useful for init from consts etc
    fn new_from_real_f64(value: f64) -> Self;

    /// Get the magnitude (or real value, for real valued data types) of this DataType.
    ///
    /// This is mainly useful for automated formatting and the likes.
    fn to_f64_mag(&self) -> f64;
}

pub trait RealValueType {}
pub trait ComplexValueType {}

impl ValueType for f32 {
    fn new_from_real_f64(value: f64) -> Self {
        value as f32
    }

    fn to_f64_mag(&self) -> f64 {
        *self as f64
    }
}
impl RealValueType for f32 {}

impl ValueType for f64 {
    fn new_from_real_f64(value: f64) -> Self {
        value
    }

    fn to_f64_mag(&self) -> f64 {
        *self
    }
}

impl RealValueType for f64 {}

impl ValueType for Complex32 {
    fn new_from_real_f64(value: f64) -> Self {
        c32(value as f32, 0.0)
    }

    fn to_f64_mag(&self) -> f64 {
        self.norm() as f64
    }
}
impl ComplexValueType for Complex32 {}

impl ValueType for Complex64 {
    fn new_from_real_f64(value: f64) -> Self {
        c64(value, 0.0)
    }

    fn to_f64_mag(&self) -> f64 {
        self.norm()
    }
}
impl ComplexValueType for Complex64 {}
