use std::ops::{Add, Mul, Sub};

macro_rules! assert_equal_len {
    ($a:ident, $b:ident, $function:ident, $operator:tt) => {
        assert!(
            $a.len() == $b.len(),
            "{:?} dimension mismatch: {:?} {:?} {:?}",
            stringify!($function),
            ($a.len(),),
            stringify!($operator),
            ($b.len(),)
        );
    };
}

macro_rules! operator {
    ($function:ident, $bound:ident, $operator:tt, $method:ident) => {
        fn $function<T: $bound<T, Output = T> + Copy>(xs: &mut Vec<T>, ys: &Vec<T>) {
            assert_equal_len!(xs, ys, $function, $operator);
            for (x, y) in xs.iter_mut().zip(ys.iter()) {
                *x = $bound::$method(*x, *y);
            }
        }
    };
}

operator!(add_assign, Add, +=, add);
operator!(mul_assign, Mul, *=, mul);
operator!(sub_assign, Sub, -=, sub);

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter;
    macro_rules! test {
        ($function:ident, $x:expr, $y:expr, $z:expr) => {
            #[test]
            fn $function() {
                for size in 0usize..10 {
                    let mut x: Vec<_> = iter::repeat($x).take(size).collect();
                    let y: Vec<_> = iter::repeat($y).take(size).collect();
                    let z: Vec<_> = iter::repeat($z).take(size).collect();

                    super::$function(&mut x, &y);

                    assert_eq!(x, z);
                }
            }
        };
    }
//
//    #[test]
//    fn test_example_17_2_dry() {
////        example_17_2_dry();
//    }
//
    test!(add_assign, 1u32, 2u32, 3u32);
    test!(mul_assign, 2u32, 3u32, 6u32);
    test!(sub_assign, 3u32, 2u32, 1u32);

}