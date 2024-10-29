pub mod dimension;
pub mod kind;
pub mod macros;
pub mod quantity;
pub mod si;
pub mod unit;
pub mod value_type;

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
    use crate::si::{
        Length,
        Metre,
    };
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

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde() {
        let l = Length::new::<Metre>(123.432);
        let json = serde_json::to_string(&l).expect("To json failed");
        let l2: Length<f64> = serde_json::from_str(&json).expect("from json failed");
        assert_eq!(l, l2);
        assert_eq!("123.432", json);
    }
}
