/// Represents kind of quantity.
/// See `crate::Quantity` for the detailed docs on this.
pub trait Kind {}

/// Allows addition of Self with Rhs
pub trait KindAdd<Rhs> {
    type Output;
}

/// Allow subtraction of Self with Rhs
pub trait KindSub<Rhs> {
    type Output;
}

/// Allow multiplication of Self with Rhs
pub trait KindMul<Rhs> {
    type Output;
}

/// Allow division of Self with Rhs
pub trait KindDiv<Rhs> {
    type Output;
}

/// () with K is K
impl<K> KindMul<()> for K
where
    K: Kind,
{
    type Output = K;
}

/// K with () is also K
impl<K> KindMul<K> for ()
where
    K: Kind,
{
    type Output = K;
}

/// K / () is K
impl<K> KindDiv<()> for K
where
    K: Kind,
{
    type Output = K;
}

/// K / K is ()
impl<K> KindDiv<K> for K
where
    K: Kind,
{
    type Output = ();
}

/// () * () is ()
impl KindMul<()> for () {
    type Output = ();
}

/// () / () is ()
impl KindDiv<()> for () {
    type Output = ();
}

/// () + () is ok
impl KindAdd<()> for () {
    type Output = ();
}

/// () - () is ok
impl KindSub<()> for () {
    type Output = ();
}

/// Angle kind. Defined outside the usual place because its used heavily inside the math ops on quantities (acos, phase(), ... yield angles natively)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AngleKind {}
impl Kind for AngleKind {}
// angles can be added/subtracted
impl KindAdd<AngleKind> for AngleKind {
    type Output = AngleKind;
}
impl KindSub<AngleKind> for AngleKind {
    type Output = ();
}

// one over angle is still something doing angles. Like Joule/Rad - thats just torgque
impl KindDiv<AngleKind> for () {
    type Output = AngleKind;
}

#[cfg(test)]
mod test {
    use typenum::assert_type_eq;

    use super::*;

    struct A {}
    struct B {}
    struct C {}
    impl Kind for A {}
    impl Kind for B {}

    impl KindDiv<B> for A {
        type Output = C;
    }

    #[test]
    fn kind_conversion() {
        assert_type_eq!(<A as KindDiv<A>>::Output, ());
        assert_type_eq!(<A as KindDiv<()>>::Output, A);
        assert_type_eq!(<A as KindMul<()>>::Output, A);
        assert_type_eq!(<() as KindMul<A>>::Output, A);
        assert_type_eq!(<() as KindMul<()>>::Output, ());
        assert_type_eq!(<() as KindDiv<()>>::Output, ());
        assert_type_eq!(<A as KindDiv<B>>::Output, C);
    }
}
