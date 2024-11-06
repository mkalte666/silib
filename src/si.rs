use num::Complex;

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
    Second: "s","SI base unit of time",prefix_units=[AllPrefixes];
    Metre: "m","SI base unit of length",prefix_units=[AllPrefixes];
    Kilogram: "kg","SI base unit of mass";
    Ampere: "A", "SI base unit of electric current",prefix_units=[AllPrefixes];
    Kelvin: "K", "SI base unit of temperature",prefix_units=[AllPrefixes];
    Mole: "mol", "SI base unit of Amount of substance",prefix_units=[AllPrefixes];
    Candela: "cd", "SI base unit of luminous intensity",prefix_units=[AllPrefixes];
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
pub type Unitless32 = Unitless<f32>;
pub type Unitless64 = Unitless<f64>;
pub type UnitlessC32 = Unitless<Complex<f32>>;
pub type UnitlessC64 = Unitless<Complex<f64>>;

make_units!(
    Ratio: "", "A unitless ratio";
    Gram: "g", "Unit of mass", prefix_units=[KgSpecialCase];
    Hertz: "Hz", "Unit of frequency",prefix_units=[AllPrefixes];
    Radian: "rad", "Unit of angle";
    Degree: "°", "Alternate unit of angle";
    Minute: "min", "Alternate unit of time";
    Hour: "hr", "Alternate unit of time";
    RadianPerSecond: "rad*s⁻¹", "Unit of angular velocity and a unit of angular frequency";
    RevolutionsPerSecond: "rps", "A unit of angular velocity";
    RevolutionsPerMinute: "rpm", "A unit of angular velocity";
    RadianPerSecondSquared: "rad*s⁻²", "Unit of angular acceleration";
    SquareMetre: "m²", "Unit of area";
    SquareMilliMetre: "mm²", "Alternate unit of area";
    CubicMetre: "m³", "Unit of volume";
    MetresPerSecond: "m*s⁻¹", "Unit of velocity",prefix_units=[AllPrefixes];
    KilometresPerSecond: "km*s⁻¹", "A unit of velocity";
    KilometresPerHour: "km*h⁻¹", "A unit of velocity";
    MilesPerSecond: "miles*s⁻¹", "A unit of velocity";
    MilesPerHour: "miles*h⁻¹","A unit of velocity";
    MetresPerSecondSquared: "m*s⁻²","Unit of acceleration";
    MetresPerSecondCubed: "m*s⁻³","Unit of jerk";
    Newton: "N", "Unit of force",prefix_units=[AllPrefixes];
    NewtonSecond: "N*s", "Unit of momentum/impulse";
    NewtonMetreSecond: "N*m*s", "Unit of angular momentum";
    NewtonMetre: "N*m", "Unit of torque",prefix_units=[AllPrefixes];
    Pascal: "Pa", "Unit of pressure and stress",prefix_units=[AllPrefixes];
    Bar: "Ba", "Alternate unit of pressure", prefix_units=[AllPrefixes];
    NewtonPerSquareMetre: "N*m⁻²", "Alternate unit of pressure";
    NewtonPerSquareMilliMetre: "N*mm⁻²", "Alternate unit of pressure";
    Joule: "J", "Unit of energy",prefix_units=[AllPrefixes];
    Watt: "W", "Unit of power",prefix_units=[AllPrefixes];
    Coulomb: "C", "Unit of charge",prefix_units=[AllPrefixes];
    Volt: "V", "Unit of electric potential",prefix_units=[AllPrefixes];
    Farad: "F", "Unit of electrical capacitance",prefix_units=[AllPrefixes];
    Ohm: "Ω", "Unit of electrical resistance",prefix_units=[AllPrefixes];
    Siemens: "S", "Unit of electrical conductance",prefix_units=[AllPrefixes];
    Weber: "Wb", "Unit of magnetic flux",prefix_units=[AllPrefixes];
    Tesla: "T", "Unit of magnetic induction",prefix_units=[AllPrefixes];
    Henry: "H", "Unit of electrical inductance",prefix_units=[AllPrefixes];
    FaradPerMetre: "F*,⁻¹", "Unit of electrical permittivity", prefix_units=[AllPrefixes];
    Celsius: "°C", "A unit of absolute temperature";
    Fahrenheit: "°F", "A unit of absolute temperature";
    KilogramPerSquareMetre: "kg*m⁻²", "Unit of specific volume";
    KilogramPerCubicMetre: "kg*m⁻³", "Unit of density";
    CubicMetrePerKilogram: "m³*kg⁻¹", "Unit of specific volume";
    JouleSecond: "J*s", "Unit of action",prefix_units=[AllPrefixes];
    JoulePerKilogram: "J*kg⁻¹", "Unit of specific energy",prefix_units=[AllPrefixes];
    JoulePerCubicMetre: "J*m⁻²", "Unit of energy tensity";
    NewtonPerMetre: "N*m⁻¹", "Unit of surface tension and Stiffness",prefix_units=[AllPrefixes];
    WattPerSquareMetre: "W*m⁻²", "Unit of heat flox density",prefix_units=[AllPrefixes];
    SquareMetrePerSecond: "m²*s⁻¹", "Unit of kinematic viscosity";
    PascalSecond: "Pa*s", "Unit of dynamic viscosity",prefix_units=[AllPrefixes];
    OnePerKelvin: "K⁻¹", "Temperature coefficient";

);

additional_unit!(ThermodynamicTemperature, Celsius, 1.0, 273.15);
additional_unit!(ThermodynamicTemperature, Fahrenheit, 5.0 / 9.0, 459.67);
additional_unit!(Unitless, Ratio, 1.0);

additional_unit!(Time, Minute, 60.0);
additional_unit!(Time, Hour, 60.0 * 60.0);

make_quantity!(
    name: Angle,
    dimension: NoDim,
    base_unit: Radian,
    use_kind: AngleKind
);

additional_unit!(Angle, Degree, std::f64::consts::PI / 180.0);

derive_quantities!(
    Frequency: (Unitless / Time), Hertz, [
        RadianPerSecond: std::f64::consts::TAU;
    ];
    Area: (Length * Length), SquareMetre, [
        SquareMilliMetre: 1e-6;
    ];
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
    Pressure: (Force / Area), Pascal, [
        NewtonPerSquareMetre: 1.0;
        NewtonPerSquareMilliMetre: 1e-6;
        Bar: 1e5;
    ];
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
    ElectricPermittivity: (Capacitance / Length), FaradPerMetre;
    AngularVelocity: (Angle/Time), RadianPerSecond, [
        Hertz: core::f64::consts::TAU;
        RevolutionsPerSecond: core::f64::consts::TAU;
        RevolutionsPerMinute: core::f64::consts::TAU/60.0;
    ];
    AngularAcceleration: (AngularVelocity/Time), RadianPerSecondSquared;
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
    TemperatureCoefficient: (Unitless / TemperatureInterval), OnePerKelvin;
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

    #[test]
    fn degree_rad() {
        let a = Angle::new::<Radian>(std::f64::consts::PI);
        let b = Angle::new::<Degree>(180.0);
        assert_eq!(a, b);
    }
}
