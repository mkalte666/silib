use silib::si::*;

fn main() {
    let a = Angle::new_base(2.0)/Time::new_base(3.0);
    let b = Frequency::new_base(3.0);
    let _c = a + b;
}