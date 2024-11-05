use silib::{
    derive_quantities,
    make_units,
    si::{
        Length,
        Mass,
        Time,
    },
};

make_units!(
    Meter2Second2PerKilo: "m²*s²/kg", "A weird new unit";
    FuzzyWaffles: "fw", "A werid new unit but bigger";
);

derive_quantities!(
    Length2Time2PerMass: (Length * Length * Time * Time / Mass), Meter2Second2PerKilo, [
        FuzzyWaffles: 42.0;
    ];
);

fn main() {
    let weird_new_quantity = Length2Time2PerMass64::new_base(420.0);
    assert_eq!(weird_new_quantity.get::<FuzzyWaffles>(), 10.0);
    assert_eq!(
        "10 fw",
        format!("{}", weird_new_quantity.formatted::<FuzzyWaffles>())
    );
    assert_eq!(
        "420 m²*s²/kg",
        format!("{}", weird_new_quantity.formatted::<Meter2Second2PerKilo>())
    );
}
