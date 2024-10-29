use std::{
    marker::PhantomData,
    ops::{
        Add,
        Div,
        Mul,
        Neg,
        Sub,
    },
};

use typenum::{
    PartialDiv,
    PartialQuot,
    Prod,
};

/// Helper trait so that you do not have to work with SiDim directly.
/// Should increase readability by a massive amount
pub trait Dimension {
    type DimType;
}

/// If the dimension implements this, 1/dim is available.
/// All should have this
pub trait DimInverse {
    type Output;
}

/// If the dimension implements this, dim^(Power) is available.
/// All should have this
pub trait DimPow<Power> {
    type Output;
}

/// If the dimension implements this, dim^1/Power is available.
/// All dimensions with all elements a multiple of Power (Aka implementing PartialQuot, N/Power=Integer) implement this.
pub trait DimNthRoot<Power> {
    type Output;
}

/// The underlying struct implementing SI Dimensions
/// Be wary if you tread here, this is ugly and a lot of code
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct SiDim<Length, Mass, Time, Current, Temperature, Amount, Intensity> {
    _l: PhantomData<Length>,
    _m: PhantomData<Mass>,
    _t: PhantomData<Time>,
    _i: PhantomData<Current>,
    _temp: PhantomData<Temperature>,
    _amount: PhantomData<Amount>,
    _intensity: PhantomData<Intensity>,
}

impl<
        LengthA,
        MassA,
        TimeA,
        CurrentA,
        TemperatureA,
        AmountA,
        IntensityA,
        LengthB,
        MassB,
        TimeB,
        CurrentB,
        TemperatureB,
        AmountB,
        IntensityB,
    > Mul<SiDim<LengthB, MassB, TimeB, CurrentB, TemperatureB, AmountB, IntensityB>>
    for SiDim<LengthA, MassA, TimeA, CurrentA, TemperatureA, AmountA, IntensityA>
where
    LengthA: typenum::Integer + Add<LengthB>,
    MassA: typenum::Integer + Add<MassB>,
    TimeA: typenum::Integer + Add<TimeB>,
    CurrentA: typenum::Integer + Add<CurrentB>,
    TemperatureA: typenum::Integer + Add<TemperatureB>,
    AmountA: typenum::Integer + Add<AmountB>,
    IntensityA: typenum::Integer + Add<IntensityB>,
{
    type Output = SiDim<
        <LengthA as Add<LengthB>>::Output,
        <MassA as Add<MassB>>::Output,
        <TimeA as Add<TimeB>>::Output,
        <CurrentA as Add<CurrentB>>::Output,
        <TemperatureA as Add<TemperatureB>>::Output,
        <AmountA as Add<AmountB>>::Output,
        <IntensityA as Add<IntensityB>>::Output,
    >;

    fn mul(
        self,
        _rhs: SiDim<LengthB, MassB, TimeB, CurrentB, TemperatureB, AmountB, IntensityB>,
    ) -> Self::Output {
        panic!("Do not call ops() of dimensional analysis types directly");
    }
}

impl<
        LengthA,
        MassA,
        TimeA,
        CurrentA,
        TemperatureA,
        AmountA,
        IntensityA,
        LengthB,
        MassB,
        TimeB,
        CurrentB,
        TemperatureB,
        AmountB,
        IntensityB,
    > Div<SiDim<LengthB, MassB, TimeB, CurrentB, TemperatureB, AmountB, IntensityB>>
    for SiDim<LengthA, MassA, TimeA, CurrentA, TemperatureA, AmountA, IntensityA>
where
    LengthA: typenum::Integer + Sub<LengthB>,
    MassA: typenum::Integer + Sub<MassB>,
    TimeA: typenum::Integer + Sub<TimeB>,
    CurrentA: typenum::Integer + Sub<CurrentB>,
    TemperatureA: typenum::Integer + Sub<TemperatureB>,
    AmountA: typenum::Integer + Sub<AmountB>,
    IntensityA: typenum::Integer + Sub<IntensityB>,
{
    type Output = SiDim<
        <LengthA as Sub<LengthB>>::Output,
        <MassA as Sub<MassB>>::Output,
        <TimeA as Sub<TimeB>>::Output,
        <CurrentA as Sub<CurrentB>>::Output,
        <TemperatureA as Sub<TemperatureB>>::Output,
        <AmountA as Sub<AmountB>>::Output,
        <IntensityA as Sub<IntensityB>>::Output,
    >;

    fn div(
        self,
        _rhs: SiDim<LengthB, MassB, TimeB, CurrentB, TemperatureB, AmountB, IntensityB>,
    ) -> Self::Output {
        panic!("Do not call ops() of dimensional analysis types directly");
    }
}

impl<Length, Mass, Time, Current, Temperature, Amount, Intensity> DimInverse
    for SiDim<Length, Mass, Time, Current, Temperature, Amount, Intensity>
where
    Length: typenum::Integer + Neg,
    Mass: typenum::Integer + Neg,
    Time: typenum::Integer + Neg,
    Current: typenum::Integer + Neg,
    Temperature: typenum::Integer + Neg,
    Amount: typenum::Integer + Neg,
    Intensity: typenum::Integer + Neg,
{
    type Output = SiDim<
        <Length as Neg>::Output,
        <Mass as Neg>::Output,
        <Time as Neg>::Output,
        <Current as Neg>::Output,
        <Temperature as Neg>::Output,
        <Amount as Neg>::Output,
        <Intensity as Neg>::Output,
    >;
}

impl<Length, Mass, Time, Current, Temperature, Amount, Intensity, Power> DimPow<Power>
    for SiDim<Length, Mass, Time, Current, Temperature, Amount, Intensity>
where
    Power: typenum::Integer,
    Length: typenum::Integer + Mul<Power>,
    Mass: typenum::Integer + Mul<Power>,
    Time: typenum::Integer + Mul<Power>,
    Current: typenum::Integer + Mul<Power>,
    Temperature: typenum::Integer + Mul<Power>,
    Amount: typenum::Integer + Mul<Power>,
    Intensity: typenum::Integer + Mul<Power>,
{
    type Output = SiDim<
        Prod<Length, Power>,
        Prod<Mass, Power>,
        Prod<Time, Power>,
        Prod<Current, Power>,
        Prod<Temperature, Power>,
        Prod<Amount, Power>,
        Prod<Intensity, Power>,
    >;
}

impl<Length, Mass, Time, Current, Temperature, Amount, Intensity, Power> DimNthRoot<Power>
    for SiDim<Length, Mass, Time, Current, Temperature, Amount, Intensity>
where
    Power: typenum::Integer,
    Length: typenum::Integer + PartialDiv<Power>,
    Mass: typenum::Integer + PartialDiv<Power>,
    Time: typenum::Integer + PartialDiv<Power>,
    Current: typenum::Integer + PartialDiv<Power>,
    Temperature: typenum::Integer + PartialDiv<Power>,
    Amount: typenum::Integer + PartialDiv<Power>,
    Intensity: typenum::Integer + PartialDiv<Power>,
{
    type Output = SiDim<
        PartialQuot<Length, Power>,
        PartialQuot<Mass, Power>,
        PartialQuot<Time, Power>,
        PartialQuot<Current, Power>,
        PartialQuot<Temperature, Power>,
        PartialQuot<Amount, Power>,
        PartialQuot<Intensity, Power>,
    >;
}

impl<Length, Mass, Time, Current, Temperature, Amount, Intensity> Dimension
    for SiDim<Length, Mass, Time, Current, Temperature, Amount, Intensity>
where
    Length: typenum::Integer,
    Mass: typenum::Integer,
    Current: typenum::Integer,
    Temperature: typenum::Integer,
    Amount: typenum::Integer,
    Intensity: typenum::Integer,
{
    type DimType = SiDim<Length, Mass, Time, Current, Temperature, Amount, Intensity>;
}

use typenum::{
    P1,
    Z0,
};

/// The dimension of Unitless
pub type NoDim = SiDim<Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
/// The dimension of Length
pub type LengthDim = SiDim<P1, Z0, Z0, Z0, Z0, Z0, Z0>;
/// The dimension of Mass
pub type MassDim = SiDim<Z0, P1, Z0, Z0, Z0, Z0, Z0>;
/// The dimension of Time
pub type TimeDim = SiDim<Z0, Z0, P1, Z0, Z0, Z0, Z0>;
/// The dimension of electrical current
pub type CurrentDim = SiDim<Z0, Z0, Z0, P1, Z0, Z0, Z0>;
/// The dimension of temperature
pub type TemperatureDim = SiDim<Z0, Z0, Z0, Z0, P1, Z0, Z0>;
/// The dimension of substance amount
pub type AmountDim = SiDim<Z0, Z0, Z0, Z0, Z0, P1, Z0>;
/// The dimension of luminous intensity
pub type IntensityDim = SiDim<Z0, Z0, Z0, Z0, Z0, Z0, P1>;
