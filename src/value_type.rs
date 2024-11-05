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

pub trait ValueType:
    FromPrimitive + Copy + NumOps + ConstOne + ConstZero + Sized + 'static
{
    fn new_from_real_f64(value: f64) -> Self;
}

pub trait RealValueType {}
pub trait ComplexValueType {}

impl ValueType for f32 {
    fn new_from_real_f64(value: f64) -> Self {
        value as f32
    }
}
impl RealValueType for f32 {}

impl ValueType for f64 {
    fn new_from_real_f64(value: f64) -> Self {
        value
    }
}

impl RealValueType for f64 {}

impl ValueType for Complex32 {
    fn new_from_real_f64(value: f64) -> Self {
        c32(value as f32, 0.0)
    }
}
impl ComplexValueType for Complex32 {}

impl ValueType for Complex64 {
    fn new_from_real_f64(value: f64) -> Self {
        c64(value, 0.0)
    }
}
impl ComplexValueType for Complex64 {}
