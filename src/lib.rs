pub mod dimension;
pub mod kind;
pub mod macros;
pub mod quantity;
pub mod si;
pub mod unit;
pub mod value_type;

pub use num::{
    self,
    complex::{
        c32,
        c64,
        Complex,
        Complex32,
        Complex64,
    },
};
pub use paste::paste;

mod static_checks {
    use num::complex::{
        Complex32,
        Complex64,
    };
    use static_assertions::{
        assert_eq_align,
        assert_eq_size,
    };

    use crate::si::*;
    assert_eq_size!(Length<f32>, f32);
    assert_eq_align!(Length<f32>, f32);
    assert_eq_size!(Length<f64>, f64);
    assert_eq_align!(Length<f64>, f64);
    assert_eq_size!(Length<Complex32>, Complex32);
    assert_eq_align!(Length<Complex32>, Complex32);
    assert_eq_size!(Length<Complex64>, Complex64);
    assert_eq_align!(Length<Complex64>, Complex64);
}

#[cfg(test)]
mod tests {
    use crate::si::*;
    #[test]
    fn sqrt_cbrt() {
        let m2 = Length::new_base(4.0) * Length::new_base(4.0);
        let m: Length<_> = m2.sqrt();
        // let nope = m2.cbrt();
        let m3 = m2 * Length::new_base(4.0);
        let m_cbrt: Length<_> = m3.cbrt();

        assert_eq!(m.get::<Metre>(), 4.0);
        assert_eq!(m_cbrt.get::<Metre>(), 4.0);
    }

    #[test]
    fn test_prefixes() {
        let l1 = Length::new::<Metre>(200.0);
        let l2 = Length::new::<KiloMetre>(0.2);
        assert_eq!(l1, l2);

        let mass1 = Mass::new::<Kilogram>(10.0);
        let mass2 = Mass::new::<Gram>(1e4);
        assert_eq!(mass1, mass2);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde() {
        let l = Length::new::<Metre>(123.432);
        let json = serde_json::to_string(&l).expect("To json failed");
        let l2: Length<f64> = serde_json::from_str(&json).expect("from json failed");
        assert_eq!(l, l2);
        assert_eq!("123.432", json);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde_basetype() {
        use super::c32;
        let l = Length::new::<Metre>(123.432);
        let json = serde_json::to_string(&l).expect("To json failed");
        let l2 = 123.432;
        let json2 = serde_json::to_string(&l2).expect("To json failed");
        assert_eq!(json, json2);

        let lc = LengthC32::new::<Metre>(c32(1.0, 5.0));
        let lc_val = c32(1.0, 5.0);
        let json = serde_json::to_string(&lc).expect("To json failed");
        let json2 = serde_json::to_string(&lc_val).expect("To json failed");
        assert_eq!(json, json2);
    }
}
