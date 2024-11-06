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
macro_rules! prefix {
    (Quetta) => {
        1e30
    };
    (Ronna) => {
        1e27
    };
    (Yotta) => {
        1e24
    };
    (Zetta) => {
        1e21
    };
    (Exa) => {
        1e18
    };
    (Peta) => {
        1e15
    };
    (Tera) => {
        1e12
    };
    (Giga) => {
        1e9
    };
    (Mega) => {
        1e6
    };
    (Kilo) => {
        1e3
    };
    (Hecto) => {
        1e2
    };
    (Deca) => {
        1e1
    };
    (Deci) => {
        1e-1
    };
    (Centi) => {
        1e-2
    };
    (Milli) => {
        1e-3
    };
    (Micro) => {
        1e-6
    };
    (Nano) => {
        1e-9
    };
    (Pico) => {
        1e-12
    };
    (Femto) => {
        1e-15
    };
    (Atto) => {
        1e-18
    };
    (Zepto) => {
        1e-21
    };
    (Yocto) => {
        1e-24
    };
    (Ronto) => {
        1e-27
    };
    (Quecto) => {
        1e-30
    };
}

#[macro_export]
macro_rules! make_unit_prefixes {
    ([AllPrefixes,],$long_name:ident, $print_name:literal,$description:literal) => {
        $crate::make_unit_prefixes!([
            Quetta, "Q",
            Ronna, "R",
            Yotta, "Y",
            Zetta, "Z",
            Exa, "E",
            Peta, "P",
            Tera, "T",
            Giga, "G",
            Mega, "M",
            Kilo,"k",
            Hecto,"h",
            Deca,"da",
            Deci,"d",
            Centi,"c",
            Milli,"m",
            Micro,"μ",
            Nano,"n",
            Pico,"p",
            Femto,"f",
            Atto,"a",
            Zepto,"z",
            Yocto,"y",
            Ronto,"r",
            Quecto,"q"
        ], 1.0, $long_name, $print_name, $description);
    };
    ([KgSpecialCase,],$long_name:ident, $print_name:literal,$description:literal) => {
        $crate::make_unit_prefixes!([
            Quetta, "Q",
            Ronna, "R",
            Yotta, "Y",
            Zetta, "Z",
            Exa, "E",
            Peta, "P",
            Tera, "T",
            Giga, "G",
            Mega, "M",
            Hecto,"h",
            Deca,"da",
            Deci,"d",
            Centi,"c",
            Milli,"m",
            Micro,"μ",
            Nano,"n",
            Pico,"p",
            Femto,"f",
            Atto,"a",
            Zepto,"z",
            Yocto,"y",
            Ronto,"r",
            Quecto,"q"
        ], 1e-3, Kilogram, $long_name, $print_name, $description);

        impl $crate::unit::PrefixUnit for $long_name {
                type Base = Kilogram;
                fn prefix() -> f64 {
                    1e-3
                }
            }
    };
    ([$pfx_name:ident, $pfx:literal,$($rest:tt)*], $additional_factor:literal, $long_name:ident, $print_name:literal, $description:literal) => {
        $crate::make_unit_prefixes!([$pfx_name,$pfx], $additional_factor, $long_name, $long_name, $print_name, $description);
        $crate::make_unit_prefixes!([$($rest)*], $additional_factor, $long_name,$long_name, $print_name, $description);
    };
    ([$pfx_name:ident, $pfx:literal,$($rest:tt)*], $additional_factor:literal, $base_name:ident, $long_name:ident,  $print_name:literal, $description:literal) => {
        $crate::make_unit_prefixes!([$pfx_name,$pfx], $additional_factor, $base_name, $long_name, $print_name, $description);
        $crate::make_unit_prefixes!([$($rest)*], $additional_factor, $base_name,$long_name, $print_name, $description);
    };
    ([$pfx_name:ident, $pfx:literal$(,)*], $additional_factor:literal, $base_name:ident, $long_name:ident, $print_name:literal,$description:literal) => {
        $crate::paste!(
        #[doc=$description]
            pub struct [<$pfx_name $long_name>] {}

            impl $crate::unit::Unit for [<$pfx_name $long_name>] {
                fn long_name() -> &'static str {
                    stringify!($pfx_name, $long_name)
                }

                fn print_name() -> &'static str {
                    concat!($pfx, $print_name)
                }
            }

            impl $crate::unit::PrefixUnit for [<$pfx_name $long_name>] {
                type Base = $base_name;
                fn prefix() -> f64 {
                    $crate::prefix!($pfx_name)*$additional_factor
                }
            }

        );
    };
}

#[macro_export]
macro_rules! make_units {
        (
            $($long_name:ident: $print_name:literal,$description:literal$(,prefix_units=[$($pfx_commands:tt)*])*;)*
        ) => {
            $(
                $crate::unit!($long_name, $print_name, $description);
                $(
                    $crate::make_unit_prefixes!([$($pfx_commands,)*], $long_name,$print_name, $description);
                )*
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

/// Helper macro to generate T*Quantity and T/Quantity operations
///
/// Unfortunatley, you cannot trivially overload those for any data type.
/// So if you ever implement your own ValueType, you will have to throw this macro at the data type.
///
/// Otherwise you can do Quantity * T, but not T * Quantity.
#[macro_export]
macro_rules! reverse_ops {
    ($tname:ty) => {
        impl<Dim, K> Mul<$crate::quantity::Quantity<$tname, Dim, K>> for $tname
        where
            Dim: $crate::dimension::Dimension,
        {
            type Output = $crate::quantity::Quantity<$tname, Dim, K>;

            fn mul(self, rhs: $crate::quantity::Quantity<$tname, Dim, K>) -> Self::Output {
                $crate::quantity::Quantity::new_base(self * rhs.base_value())
            }
        }

        impl<Dim, K> Div<$crate::quantity::Quantity<$tname, Dim, K>> for $tname
        where
            Dim: $crate::dimension::Dimension + $crate::dimension::DimInverse,
            <Dim as $crate::dimension::DimInverse>::Output: $crate::dimension::Dimension,
        {
            type Output = $crate::quantity::Quantity<
                $tname,
                <Dim as $crate::dimension::DimInverse>::Output,
                K,
            >;

            fn div(self, rhs: $crate::quantity::Quantity<$tname, Dim, K>) -> Self::Output {
                $crate::quantity::Quantity::new_base(self / rhs.base_value())
            }
        }
    };
}
