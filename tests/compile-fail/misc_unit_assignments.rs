use silib::si::*;

fn main() {
    let a = Angle64::new_base(2.0)/Time64::new_base(3.0);
    let b = Frequency64::new_base(3.0);
    let _a2 = a+a;
    let _c = a + b;
}