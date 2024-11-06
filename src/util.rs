use crate::value_type::ValueType;

/// Automatically find the closest SI prefix for a given value, and return the value in that prefix, and the prefix.
pub fn find_prefix<T>(value: T) -> (T, &'static str)
where
    T: ValueType,
{
    static PREFIXES: &[&'static str] = &[
        "q", "r", "y", "z", "a", "f", "p", "n", "μ", "m", "", "k", "M", "G", "T", "P", "E", "Z",
        "Y", "R", "Q",
    ];
    let zero_at = PREFIXES.len() / 2;
    let floated = value.to_f64_mag().abs();
    let prefix_id = (floated.log10() / 3.0)
        .floor()
        .clamp(-(zero_at as f64), zero_at as f64);
    let factor = 10.0_f64.powf(prefix_id * 3.0);
    let index = (prefix_id + zero_at as f64).round() as usize;

    let new_value = value / T::new_from_real_f64(factor);

    (new_value, PREFIXES[index])
}

#[cfg(test)]
mod test {
    use crate::util::find_prefix;

    fn check_pfx(to_check: (f64, &'static str), val: f64, chr: &'static str, eps: f64) {
        assert_eq!(chr, to_check.1);
        assert!(
            (to_check.0 - val).abs() < eps,
            "Float eq failed. a = {}, b = {}, diff={}, eps={}",
            to_check.0,
            val,
            (to_check.0 - val).abs(),
            eps
        );
    }

    #[test]
    fn test_prefix() {
        check_pfx(find_prefix(1e-40f64), 1e-10, "q", 1e-12);
        check_pfx(find_prefix(1e-30f64), 1.0, "q", 1e-12);
        check_pfx(find_prefix(1e-29f64), 10.0, "q", 1e-12);
        check_pfx(find_prefix(1e-15f64), 1.0, "f", 1e-12);
        check_pfx(find_prefix(123.0f64), 123.0, "", 1e-12);
        check_pfx(find_prefix(123_000_000f64), 123.0, "M", 1e-12);
        check_pfx(find_prefix(0.1), 100.0, "m", 1e-12);
    }
}
