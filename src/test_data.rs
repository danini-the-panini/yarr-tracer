#[cfg(test)]
#[macro_export]
macro_rules! assert_in_delta {
    ($left:expr, $right:expr, $delta:expr) => {
        assert!(
            ($left - $right).abs() < $delta,
            "{:?} - {:?} = {:?}",
            $left,
            $right,
            ($left - $right).abs()
        )
    };
    ($left:expr, $right:expr) => {
        assert_in_delta!($left, $right, crate::util::EPSILON)
    };
}
