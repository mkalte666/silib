/// Represents kind of quantity.
/// See `crate::Quantity` for the detailed docs on this.
pub trait Kind {}

pub trait KindMul<Rhs> {
    type Output;
}

pub trait KindDiv<Rhs> {
    type Output;
}

impl<K> KindMul<()> for K
where
    K: Kind,
{
    type Output = K;
}

impl<K> KindMul<K> for ()
where
    K: Kind,
{
    type Output = K;
}

impl<K> KindDiv<()> for K
where
    K: Kind,
{
    type Output = K;
}

impl<K> KindDiv<K> for K
where
    K: Kind,
{
    type Output = ();
}

impl KindMul<()> for () {
    type Output = ();
}

impl KindDiv<()> for () {
    type Output = ();
}

impl<K> KindDiv<K> for ()
where
    K: Kind,
{
    type Output = K;
}

/// Angle kind. Defined outside the usual place because its used heavily inside the math ops on quantities (acos, phase(), ... yield angles natively)
pub struct AngleKind {}
impl Kind for AngleKind {}

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
