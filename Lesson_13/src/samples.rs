#[allow(unused_macros)]
macro_rules! max_of {
    ($x:expr) => {
        $x
    };
    ($x:expr,$y:expr) => {
        if $x > $y {
            $x
        } else {
            $y
        }
    };
    ($x:expr,$($y:expr),+) => {
        max_of!($x,max_of!($($y),+))
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_max_macro() {
        assert_eq!(max_of!(1), 1);
        assert_eq!(max_of!(2, 7), 7);
        assert_eq!(max_of!(10, 0, 6, 19, 20, 3, 2, 7), 20);
        assert_eq!(max_of!(-8, -5, -3, -99), -3);
    }
}
