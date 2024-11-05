#[macro_export]
macro_rules! unit {
    ($name:ident,$print_name:literal,$description:literal) => {
        #[doc=$description]
        pub struct $name {}

        impl $crate::unit::Unit for $name {
            fn long_name() -> &'static str {
                stringify!($name)
            }

            fn print_name() -> &'static str {
                $print_name
            }
        }
    };
}

#[macro_export]
macro_rules! make_units {
        (
            $($long_name:ident: $print_name:literal,$description:literal;)*
        ) => {
            $(
                $crate::unit!($long_name, $print_name, $description);
            )*
        };
    }

#[macro_export]
macro_rules! make_quantity {
    (
        name: $name:ident,
        dimension: $dim:tt,
        base_unit: $base:ident,
        use_kind: $kind:ty $(,)*
    ) => {
        pub type $name<T> = $crate::quantity::Quantity<T,$dim,$kind>;
        $crate::paste!(
            pub type [<$name 32>] = $name<f32>;
            pub type [<$name 64>] = $name<f64>;
            pub type [<$name C32>] = $name<$crate::num::Complex<f32>>;
            pub type [<$name C64>] = $name<$crate::num::Complex<f64>>;
        );
        impl<T> $crate::unit::QuantityConversion<T,$dim,$kind> for $base
        where T: $crate::value_type::ValueType
        {
            fn factor() -> T {
                T::new_from_real_f64(1.0)
            }
        }
    };
    (
        name: $name:ident,
        dimension: $dim:tt,
        base_unit: $base_unit:ident$(,)*
    ) => {
        make_quantity!(
            name: $name,
            dimension: $dim,
            base_unit: $base_unit,
            use_kind: (),
        );
    };
    (
        name: $name:ident,
        dimension: $dim:tt,
        base_unit: $base_unit:ident,
        make_kind: $kind:ident$(,)*
    ) => {
        pub struct $kind {}
        impl $crate::kind::Kind for $kind {}

        make_quantity!(
            name: $name,
            dimension: $dim,
            base_unit: $base_unit,
            use_kind: $kind,
        );
    };
}

#[macro_export]
macro_rules! make_base_quantities {
    ($(
        $name:ident: $dim:ident, $unit:ident;
    )*
    ) => {
        $(
            $crate::make_quantity!(
                name: $name,
                dimension: $dim,
                base_unit: $unit,
            );
        )*
    };
}

#[macro_export]
macro_rules! type_math {
    ($a:tt) => {$a<T>};
    ($a:tt * $($b:tt)+) => {
        typenum::Prod<$a<T>,$crate::type_math!($($b)*)>
    };
    ($a:tt / $($b:tt)+) => {
        typenum::Quot<$a<T>,$crate::type_math!($($b)*)>
    };
}

#[macro_export]
macro_rules! additional_unit {
    ($name:ident, $unit:ident, $factor:expr $(,$offset:expr)*) => {
        impl<T>
            $crate::unit::QuantityConversion<
                T,
                <$name<T> as $crate::quantity::QuantityTypeInfo>::Dimension,
                <$name<T> as $crate::quantity::QuantityTypeInfo>::Kind,
            > for $unit
        where
            T: $crate::value_type::ValueType,
        {
            fn factor() -> T {
                T::new_from_real_f64($factor)
            }
            $(
                fn offset() -> T {
                    T::new_from_real_f64($offset)
                }
            )*
        }
    };
}

#[macro_export]
macro_rules! derive_quantity {
    (
        name: $name:ident,
        quantity_type: ($($ty:tt)*),
        unit: $unit:ident,
        $(additional_units: [
            $($extra_unit_name:ident: $extra_unit_factor:expr $(, $extra_unit_offset:expr)*;)*
        ]$(,)*)*
    ) => {
        pub type $name<T> = $crate::type_math!($($ty)*);
        $crate::paste!(
            pub type [<$name 32>] = $name<f32>;
            pub type [<$name 64>] = $name<f64>;
            pub type [<$name C32>] = $name<$crate::num::Complex<f32>>;
            pub type [<$name C64>] = $name<$crate::num::Complex<f64>>;
        );
        impl<T> $crate::unit::QuantityConversion<T,<$name<T> as $crate::quantity::QuantityTypeInfo>::Dimension,<$name<T> as $crate::quantity::QuantityTypeInfo>::Kind> for $unit
        where T: $crate::value_type::ValueType
        {
            fn factor() -> T {
                T::new_from_real_f64(1.0)
            }
        }
        $($(
            $crate::additional_unit!($name, $extra_unit_name, $extra_unit_factor $(,$extra_unit_offset)*);
        )*)*
    };
}

#[macro_export]
macro_rules! derive_quantities {
    ($($name:ident: ($($ty:tt)*), $unit:ident $(,[$($extra_unit_name:ident: $extra_unit_factor:expr $(,$extra_unit_offset:expr)*;)*])*;)*) => {
        $(
            $crate::derive_quantity!(
                name: $name,
                quantity_type: ($($ty)*),
                unit: $unit,
                $(
                additional_units: [$(
                  $extra_unit_name: $extra_unit_factor $(, $extra_unit_offset)*;
                )*]
                )*
            );
        )*
    };
}

#[macro_export]
macro_rules! add_kind_rule {
    ($lhs:ident + $rhs:tt -> $result:tt ) => {
        impl $crate::kind::KindAdd<$rhs> for $lhs {
            type Output = $result;
        }
        impl $crate::kind::KindAdd<$lhs> for $rhs {
            type Output = $result;
        }
    };
    ($lhs:ident + -> $result:tt ) => {
        impl $crate::kind::KindAdd<$lhs> for $lhs {
            type Output = $result;
        }
    };
    ($lhs:ident - $rhs:tt -> $result:tt ) => {
        impl $crate::kind::KindSub<$rhs> for $lhs {
            type Output = $result;
        }
        impl $crate::kind::KindSub<$lhs> for $rhs {
            type Output = $result;
        }
    };
    ($lhs:ident - -> $result:tt ) => {
        impl $crate::kind::KindSub<$lhs> for $lhs {
            type Output = $result;
        }
    };
    ($lhs:ident * $rhs:tt -> $result:tt ) => {
        impl $crate::kind::KindMul<$rhs> for $lhs {
            type Output = $result;
        }
        impl $crate::kind::KindMul<$lhs> for $rhs {
            type Output = $result;
        }
    };
    ($lhs:ident * -> $result:tt ) => {
        impl $crate::kind::KindMul<$lhs> for $lhs {
            type Output = $result;
        }
    };
    ($lhs:ident / $rhs:tt-> $result:tt ) => {
        impl $crate::kind::KindDiv<$rhs> for $lhs {
            type Output = $result;
        }
    };
}

#[macro_export]
macro_rules! make_kind {
    (
        name: $name:ident,
        rules: [$(($($op:tt)*),)*]
    ) => {
        #[derive(Debug, Copy, Clone, PartialEq)]
        pub struct $name {}
        impl $crate::kind::Kind for $name {}

        $(
            $crate::add_kind_rule!($($op)*);
        )*
    };
}
