use crate::{
    additional_unit,
    derive_quantities,
    dimension::si::{
        AmountDim,
        CurrentDim,
        IntensityDim,
        LengthDim,
        MassDim,
        NoDim,
        TemperatureDim,
        TimeDim,
    },
    make_base_quantities,
    make_quantity,
    make_units,
    quantity::{
        Quantity,
        QuantityTypeInfo,
    },
    unit::QuantityConversion,
    value_type::ValueType,
};

make_units!(
    Second: "s","SI base unit of time";
    Metre: "m","SI base unit of length";
    Kilogram: "kg","SI base unit of mass";
    Ampere: "A", "SI base unit of electric current";
    Kelvin: "K", "SI base unit of temperature";
    Mole: "mol", "SI base unit of Amount of substance";
    Candela: "cd", "SI base unit of luminous intensity";
);

make_base_quantities!(
    Length: LengthDim, Metre;
    Time: TimeDim, Second;
    Mass: MassDim, Kilogram;
    ElectricCurrent: CurrentDim, Ampere;
    Amount: AmountDim, Mole;
    LouminousIntensity: IntensityDim, Candela;
);

make_quantity!(
    name: ThermodynamicTemperature,
    dimension: TemperatureDim,
    base_unit: Kelvin,
    make_kind: ThermodynamicTemperatureKind,
);

make_quantity!(
    name: TemperatureInterval,
    dimension: TemperatureDim,
    base_unit: Kelvin,
);
pub type Unitless<T> = Quantity<T, NoDim, ()>;

make_units!(
    SquareMetre: "m²", "Unit of area";
    CubicMetre: "m³", "Unit of volume";
    MetresPerSecond: "m/s", "Unit of velocity";
    KilometresPerSecond: "km/s", "";
    KilometresPerHour: "km/h", "";
    MilesPerSecond: "miles/s", "";
    MilesPerHour: "miles/h","";
    MetresPerSecondSquared: "m/s²","Unit of acceleration";
    Newton: "N", "Unit of force";
    Pascal: "Pa", "Unit of pressure and stress";
    Joule: "J", "Unit of energy";
    Watt: "W", "Unit of power";
    Coulomb: "C", "Unit of charge";
    Volt: "V", "Unit of electric potential";
    Farad: "F", "Unit of electrical capacitance";
    Ohm: "Ω", "Unit of electrical resistance";
    Siemens: "S", "Unit of electrical conductance";
    Weber: "Wb", "Unit of magnetic flux";
    Tesla: "T", "Unit of magnetic induction";
    Henry: "H", "Unit of electrical inductance";
    Celsius: "°C", "A unit of absolute temperature";
    Fahrenheit: "°F", "A unit of absolute temperature";
);

additional_unit!(ThermodynamicTemperature, Celsius, 1.0, 273.15);
additional_unit!(ThermodynamicTemperature, Fahrenheit, 0.55555555555, 459.67);

derive_quantities!(
    Area: (Length * Length), SquareMetre;
    Volume: (Length * Length * Length), CubicMetre;
    Velocity: (Length/ Time), MetresPerSecond, [
        KilometresPerSecond: 1000.0,
        KilometresPerHour: 0.27777777777,
        MilesPerSecond: 1609.34,
        MilesPerHour: 0.44703888888,
    ];
    Acceleration: (Length / Time / Time), MetresPerSecondSquared;
    Force: (Mass * Acceleration), Newton;
    Pressure: (Force / Area), Pascal;
    Energy: (Force * Length), Joule;
    Power: (Energy / Time), Watt;
    ElectricCharge: (Time * ElectricCurrent), Coulomb;
    ElectricPotential: (Power / ElectricCurrent), Volt;
    Capacitance: (ElectricCharge / ElectricPotential), Farad;
    ElectricResistance: (ElectricPotential/ElectricCurrent), Ohm;
    ElectricConductivity: (Unitless / ElectricResistance), Siemens;
    MagneticFlux: (Energy / ElectricCurrent), Weber;
    MagneticInduction: (MagneticFlux / Area), Tesla;
    ElectricInducance: (ElectricResistance * Time), Henry;
);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_kind_conversion() {
        //let f = Frequency::new_base(1.0);
        //let f2 = Frequency::new::<Hertz>(2.0*std::f32::consts::PI);
        //let v = AngularVelocity::new::<RadianPerSecond>(2.0*std::f32::consts::PI);

        // this does not compile:
        // let urgh : Frequency<_> = v;

        // this makes not much sense, but is supported:
        //assert_eq!(f2,v.unrestricted());
        // more sense:
        //assert_eq!(f.get::<Hertz>(),v.get::<Hertz>());
    }

    #[test]
    fn stringify() {
        let dist = Length::new::<Metre>(1.346);
        assert_eq!("1.346 m", format!("{}", dist.formatted::<Metre>()));
        assert_eq!("1.35 m", format!("{:.2}", dist.formatted::<Metre>()));
    }
}
