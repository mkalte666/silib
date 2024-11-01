use crate::{
    additional_unit,
    derive_quantities,
    dimension::{
        AmountDim,
        CurrentDim,
        IntensityDim,
        LengthDim,
        MassDim,
        NoDim,
        TemperatureDim,
        TimeDim,
    },
    kind::AngleKind,
    make_base_quantities,
    make_kind,
    make_quantity,
    make_units,
    quantity::Quantity,
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
    use_kind: ThermodynamicTemperatureKind,
);

make_kind!(
    name: ThermodynamicTemperatureKind,
    rules: [
        (ThermodynamicTemperatureKind + () -> ThermodynamicTemperatureKind),
        (ThermodynamicTemperatureKind - -> ()),
    ]
);

make_quantity!(
    name: TemperatureInterval,
    dimension: TemperatureDim,
    base_unit: Kelvin,
);
pub type Unitless<T> = Quantity<T, NoDim, ()>;

make_units!(
    Hertz: "Hz", "Unit of frequency";
    Radian: "rad", "Unit of angle";
    RadianPerSecond: "rad*s⁻¹", "Unit of angular velocity";
    RevolutionsPerSecond: "rps", "A unit of angular velocity";
    RevolutionsPerMinute: "rpm", "A unit of angular velocity";
    RadianPerSecondSquared: "rad*s⁻²", "Unit of angular acceleration";
    SquareMetre: "m²", "Unit of area";
    CubicMetre: "m³", "Unit of volume";
    MetresPerSecond: "m*s⁻¹", "Unit of velocity";
    KilometresPerSecond: "km*s⁻¹", "A unit of velocity";
    KilometresPerHour: "km*h⁻¹", "A unit of velocity";
    MilesPerSecond: "miles*s⁻¹", "A unit of velocity";
    MilesPerHour: "miles*h⁻¹","A unit of velocity";
    MetresPerSecondSquared: "m*s⁻²","Unit of acceleration";
    MetresPerSecondCubed: "m*s⁻³","Unit of jerk";
    Newton: "N", "Unit of force";
    NewtonSecond: "N*s", "Unit of momentum/impulse";
    NewtonMetreSecond: "N*m*s", "Unit of angular momentum";
    NewtonMetre: "N*m", "Unit of torque";
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
    KilogramPerSquareMetre: "kg*m⁻²", "Unit of specific volume";
    KilogramPerCubicMetre: "kg*m⁻³", "Unit of density";
    CubicMetrePerKilogram: "m³*kg⁻¹", "Unit of specific volume";
    JouleSecond: "J*s", "Unit of action";
    JoulePerKilogram: "J*kg⁻¹", "Unit of specific energy";
    JoulePerCubicMetre: "J*m⁻²", "Unit of energy tensity";
    NewtonPerMetre: "N*m⁻¹", "Unit of surface tension and Stiffness";
    WattPerSquareMetre: "W*m⁻²", "Unit of heat flox density";
    SquareMetrePerSecond: "m²*s⁻¹", "Unit of kinematic viscosity";
    PascalSecond: "Pa*s", "Unit of dynamic viscosity";
);

additional_unit!(ThermodynamicTemperature, Celsius, 1.0, 273.15);
additional_unit!(ThermodynamicTemperature, Fahrenheit, 5.0 / 9.0, 459.67);

make_quantity!(
    name: Angle,
    dimension: NoDim,
    base_unit: Radian,
    use_kind: AngleKind
);

derive_quantities!(
    Frequency: (Unitless / Time), Hertz;
    Area: (Length * Length), SquareMetre;
    Volume: (Length * Length * Length), CubicMetre;
    Velocity: (Length/ Time), MetresPerSecond, [
        KilometresPerSecond: 1000.0;
        KilometresPerHour: 1000.0/(60.0*60.0);
        MilesPerSecond: 1609.34;
        MilesPerHour: 1609.34/(60.0*60.0);
    ];
    Acceleration: (Length / Time / Time), MetresPerSecondSquared;
    Jerk: (Acceleration / Time), MetresPerSecondCubed;
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
    AngluarVelocity: (Angle/Time), RadianPerSecond, [
        Hertz: core::f64::consts::TAU;
        RevolutionsPerSecond: core::f64::consts::TAU;
        RevolutionsPerMinute: core::f64::consts::TAU/60.0;
    ];
    AngluarAcceleration: (AngluarVelocity/Time), RadianPerSecondSquared;
    Torque: (Energy/Angle), NewtonMetre;
    AngularMomentum: (Torque * Time), NewtonMetreSecond;
    AreaDensity: (Mass/Area), KilogramPerSquareMetre;
    Density: (Mass/Volume), KilogramPerCubicMetre;
    SpecificVolume: (Volume/Mass), CubicMetrePerKilogram;
    Action: (Energy * Time), JouleSecond;
    SpecificEnergy: (Energy / Mass), JoulePerKilogram;
    EnergyDensity: (Energy / Volume), JoulePerCubicMetre;
    SurfaceTension: (Force / Length), NewtonPerMetre;
    HeatFluxDensity: (Power/Area), WattPerSquareMetre;
    KinematicViscosity: (Area/Time), SquareMetrePerSecond;
    DynamicViscosity: (Pressure * Time), PascalSecond;
);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_kind_conversion() {
        let angle = Angle::new_base(std::f64::consts::PI);
        let angle2 = Angle::new_base(std::f64::consts::PI);
        let a = Unitless::new_base(1.0);
        assert_eq!(angle + angle2, Angle::new_base(std::f64::consts::PI * 2.0));
        //assert_eq!(a,angle); does not compile
        assert_eq!(a * angle2, angle2); // that is fine.
    }

    #[test]
    fn check_temp() {
        let t1 = ThermodynamicTemperature::new::<Celsius>(20.0);
        assert_eq!(t1.get::<Kelvin>(), 293.15);

        // Intervals are in kelvin!
        //let t2 = TemperatureInterval::new::<Celsius>(10.0);
        let t2 = TemperatureInterval::new::<Kelvin>(10.0);
        let t3 = TemperatureInterval::new::<Kelvin>(5.0);
        // you can add intervals
        let t4 = t2 + t3;
        // you can add intervals to temps.
        let t5 = t1 + t4;
        let t6 = t4 + t1;
        assert_eq!(t5, t6);
        // you cannot add thermodynamic temperatures!
        //let t7 = t1 + t1;
        // you *can* substract them. It yields intervals!
        let t8: TemperatureInterval<_> = t1 - t1;
        assert_eq!(t8.get::<Kelvin>(), 0.0);
    }

    #[test]
    fn stringify() {
        let dist = Length::new::<Metre>(1.346);
        assert_eq!("1.346 m", format!("{}", dist.formatted::<Metre>()));
        assert_eq!("1.35 m", format!("{:.2}", dist.formatted::<Metre>()));
    }
}
