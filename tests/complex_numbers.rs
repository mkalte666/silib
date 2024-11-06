use num::complex::c64;
use silib::{
    quantity::QuantityType,
    si::*,
};

#[test]
fn basic_ops() {
    let a = ElectricResistance64::new::<Ohm>(123.0);
    let b = ElectricResistanceC64::new::<Ohm>(c64(0.0, 432.0));
    let c: ElectricResistanceC64 = a + b;
    let d = b + a;
    assert_eq!(c.get::<Ohm>(), c64(123.0, 432.0));
    assert_eq!(d, c);
}

#[test]
fn basetype_scaling() {
    let a = ElectricResistanceC64::new_base(c64(4.0, 5.0));
    let b = a * 10.0;
    assert_eq!(b.get::<Ohm>(), c64(40.0, 50.0));

    let c = ElectricResistance64::new_base(2.0);
    let d = c * c64(3.0, 5.0);
    assert_eq!(d.get::<Ohm>(), c64(6.0, 10.0));
}

#[test]
fn some_real_use() {
    // lets to an R||C

    let r = ElectricResistance64::new::<Ohm>(50.0);
    let c = Capacitance64::new::<Farad>(1e-6);
    let f = Frequency64::new::<Hertz>(1e4);

    let z_r: ElectricResistanceC64 = r * UnitlessC64::one();
    let z_c: ElectricResistanceC64 = -(f * c * std::f64::consts::TAU).inv() * UnitlessC64::j();

    let z_total = (z_r * z_c) / (z_r + z_c);
    assert!((z_total.norm().get::<Ohm>() - 15.1657235527).abs() < 0.0000001);
    assert!((z_total.arg().get::<Radian>() - (-1.2626272557)).abs() < 0.0000001);
}
