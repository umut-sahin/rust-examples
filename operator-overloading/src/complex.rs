#![allow(clippy::suspicious_arithmetic_impl)]

use std::{
    cmp::Ordering,
    fmt::{
        self,
        Display,
    },
    ops::{
        Add,
        AddAssign,
        Div,
        DivAssign,
        Mul,
        MulAssign,
        Neg,
        Sub,
        SubAssign,
    },
};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Complex(pub f64, pub f64);

impl Complex {
    #[inline]
    pub fn new(a: impl Into<f64>, b: impl Into<f64>) -> Complex {
        Complex(a.into(), b.into())
    }
}

impl Complex {
    #[inline]
    pub fn a(&self) -> f64 {
        self.0
    }

    #[inline]
    pub fn set_a(&mut self, value: impl Into<f64>) {
        self.0 = value.into();
    }

    #[inline]
    pub fn b(&self) -> f64 {
        self.1
    }

    #[inline]
    pub fn set_b(&mut self, value: impl Into<f64>) {
        self.1 = value.into();
    }
}

impl Complex {
    #[inline]
    pub fn conjugate(&self) -> Complex {
        Complex::new(self.a(), -self.b())
    }
}

impl Add for Complex {
    type Output = Complex;

    #[inline]
    fn add(self, rhs: Complex) -> Complex {
        Complex::new(self.a() + rhs.a(), self.b() + rhs.b())
    }
}

impl AddAssign for Complex {
    #[inline]
    fn add_assign(&mut self, rhs: Complex) {
        *self = *self + rhs;
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let a = self.a();
        let b = self.b();

        match a.partial_cmp(&0f64).unwrap() {
            Ordering::Equal => {
                match b.partial_cmp(&0f64).unwrap() {
                    Ordering::Equal => write!(f, "0"),
                    _ => write!(f, "{}i", b),
                }
            },
            _ => {
                match b.partial_cmp(&0f64).unwrap() {
                    Ordering::Less => write!(f, "{} - {}i", a, b.abs()),
                    Ordering::Equal => write!(f, "{}", a),
                    Ordering::Greater => write!(f, "{} + {}i", a, b),
                }
            },
        }
    }
}

impl Div for Complex {
    type Output = Complex;

    fn div(self, rhs: Complex) -> Complex {
        let divider = rhs.a() * rhs.a() + rhs.b() * rhs.b();
        assert!(divider.abs() >= std::f64::EPSILON);

        let dividend = self * rhs.conjugate();
        dividend / divider
    }
}

impl<T: Into<f64>> Div<T> for Complex
where
    T: Into<f64>,
{
    type Output = Complex;

    #[inline]
    fn div(self, rhs: T) -> Complex {
        let divider: f64 = rhs.into();
        assert!(divider.abs() >= std::f64::EPSILON);

        Complex::new(self.a() / divider, self.b() / divider)
    }
}

impl DivAssign for Complex {
    #[inline]
    fn div_assign(&mut self, rhs: Complex) {
        *self = *self / rhs;
    }
}

impl<T> DivAssign<T> for Complex
where
    T: Into<f64>,
{
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs;
    }
}

impl Mul for Complex {
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Complex {
        let a1xa2 = self.a() * rhs.a();
        let a1xb2 = self.a() * rhs.b();
        let b1xa2 = self.b() * rhs.a();
        let b1xb2 = self.b() * rhs.b();
        Complex::new(a1xa2 - b1xb2, a1xb2 + b1xa2)
    }
}

impl<T> Mul<T> for Complex
where
    T: Into<f64>,
{
    type Output = Complex;

    #[inline]
    fn mul(self, rhs: T) -> Complex {
        let multiplier = rhs.into();
        Complex::new(self.a() * multiplier, self.b() * multiplier)
    }
}

impl MulAssign for Complex {
    #[inline]
    fn mul_assign(&mut self, rhs: Complex) {
        *self = *self * rhs;
    }
}

impl<T> MulAssign<T> for Complex
where
    T: Into<f64>,
{
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs;
    }
}

impl Neg for Complex {
    type Output = Complex;

    #[inline]
    fn neg(self) -> Complex {
        Complex::new(-self.a(), -self.b())
    }
}

impl Sub for Complex {
    type Output = Complex;

    #[inline]
    fn sub(self, rhs: Complex) -> Complex {
        Complex::new(self.a() - rhs.a(), self.b() - rhs.b())
    }
}

impl SubAssign for Complex {
    #[inline]
    fn sub_assign(&mut self, rhs: Complex) {
        *self = *self - rhs;
    }
}

#[cfg(test)]
mod complex {
    mod associated_functions {
        mod new {
            use crate::complex::Complex;

            #[test]
            fn float_a_and_float_b() {
                let x = Complex::new(0.42, 10.20);
                assert_eq!(x.a(), 0.42);
                assert_eq!(x.b(), 10.20);
            }

            #[test]
            fn integer_a_and_float_b() {
                let x = Complex::new(42, 10.20);
                assert_eq!(x.a(), 42f64);
                assert_eq!(x.b(), 10.20);
            }

            #[test]
            fn float_a_and_integer_b() {
                let x = Complex::new(0.42, 10);
                assert_eq!(x.a(), 0.42);
                assert_eq!(x.b(), 10f64);
            }

            #[test]
            fn integer_a_and_integer_b() {
                let x = Complex::new(42, 10);
                assert_eq!(x.a(), 42f64);
                assert_eq!(x.b(), 10f64);
            }
        }
    }

    mod getter_setters {
        mod a {
            mod getter {
                use crate::complex::Complex;

                #[test]
                fn negative_a() {
                    let x = Complex::new(-42, 0);
                    assert_eq!(x.a(), -42f64);
                }

                #[test]
                fn zero_a() {
                    let x = Complex::new(0, 0);
                    assert_eq!(x.a(), 0f64);
                }

                #[test]
                fn positive_a() {
                    let x = Complex::new(42, 0);
                    assert_eq!(x.a(), 42f64);
                }
            }

            mod setter {
                use crate::complex::Complex;

                #[test]
                fn from_negative_a_to_negative_a() {
                    let mut x = Complex::new(-10, 0);
                    x.set_a(-42);
                    assert_eq!(x.a(), -42f64);
                }

                #[test]
                fn from_zero_a_to_negative_a() {
                    let mut x = Complex::new(0, 0);
                    x.set_a(-42);
                    assert_eq!(x.a(), -42f64);
                }

                #[test]
                fn from_positive_a_to_negative_a() {
                    let mut x = Complex::new(10, 0);
                    x.set_a(-42);
                    assert_eq!(x.a(), -42f64);
                }

                #[test]
                fn from_negative_a_to_zero_a() {
                    let mut x = Complex::new(-10, 0);
                    x.set_a(0);
                    assert_eq!(x.a(), 0f64);
                }

                #[test]
                fn from_zero_a_to_zero_a() {
                    let mut x = Complex::new(0, 0);
                    x.set_a(0);
                    assert_eq!(x.a(), 0f64);
                }

                #[test]
                fn from_positive_a_to_zero_a() {
                    let mut x = Complex::new(10, 0);
                    x.set_a(0);
                    assert_eq!(x.a(), 0f64);
                }

                #[test]
                fn from_negative_a_to_positive_a() {
                    let mut x = Complex::new(-10, 0);
                    x.set_a(42);
                    assert_eq!(x.a(), 42f64);
                }

                #[test]
                fn from_zero_a_to_positive_a() {
                    let mut x = Complex::new(0, 0);
                    x.set_a(42);
                    assert_eq!(x.a(), 42f64);
                }

                #[test]
                fn from_positive_a_to_positive_a() {
                    let mut x = Complex::new(10, 0);
                    x.set_a(42);
                    assert_eq!(x.a(), 42f64);
                }
            }
        }

        mod b {
            mod getter {
                use crate::complex::Complex;

                #[test]
                fn negative_b() {
                    let x = Complex::new(0, -42);
                    assert_eq!(x.b(), -42f64);
                }

                #[test]
                fn zero_b() {
                    let x = Complex::new(0, 0);
                    assert_eq!(x.b(), 0f64);
                }

                #[test]
                fn positive_b() {
                    let x = Complex::new(0, 42);
                    assert_eq!(x.b(), 42f64);
                }
            }

            mod setter {
                use crate::complex::Complex;

                #[test]
                fn from_negative_b_to_negative_b() {
                    let mut x = Complex::new(0, -10);
                    x.set_b(-42);
                    assert_eq!(x.b(), -42f64);
                }

                #[test]
                fn from_zero_b_to_negative_b() {
                    let mut x = Complex::new(0, 0);
                    x.set_b(-42);
                    assert_eq!(x.b(), -42f64);
                }

                #[test]
                fn from_positive_b_to_negative_b() {
                    let mut x = Complex::new(0, 10);
                    x.set_b(-42);
                    assert_eq!(x.b(), -42f64);
                }

                #[test]
                fn from_negative_b_to_zero_b() {
                    let mut x = Complex::new(0, -10);
                    x.set_b(0);
                    assert_eq!(x.b(), 0f64);
                }

                #[test]
                fn from_zero_b_to_zero_b() {
                    let mut x = Complex::new(0, 0);
                    x.set_b(0);
                    assert_eq!(x.b(), 0f64);
                }

                #[test]
                fn from_positive_b_to_zero_b() {
                    let mut x = Complex::new(0, 10);
                    x.set_b(0);
                    assert_eq!(x.b(), 0f64);
                }

                #[test]
                fn from_negative_b_to_positive_b() {
                    let mut x = Complex::new(0, -10);
                    x.set_b(42);
                    assert_eq!(x.b(), 42f64);
                }

                #[test]
                fn from_zero_b_to_positive_b() {
                    let mut x = Complex::new(0, 0);
                    x.set_b(42);
                    assert_eq!(x.b(), 42f64);
                }

                #[test]
                fn from_positive_b_to_positive_b() {
                    let mut x = Complex::new(0, 10);
                    x.set_b(42);
                    assert_eq!(x.b(), 42f64);
                }
            }
        }
    }

    mod methods {
        mod conjugate {
            use crate::complex::Complex;

            #[test]
            fn zero_a_and_zero_b() {
                let x = Complex::new(0, 0);
                assert_eq!(x.conjugate(), Complex::new(0, 0));
            }

            #[test]
            fn zero_a_and_positive_b() {
                let x = Complex::new(0, 42);
                assert_eq!(x.conjugate(), Complex::new(0, -42));
            }

            #[test]
            fn zero_a_and_negative_b() {
                let x = Complex::new(0, -42);
                assert_eq!(x.conjugate(), Complex::new(0, 42));
            }

            #[test]
            fn positive_a_and_zero_b() {
                let x = Complex::new(42, 0);
                assert_eq!(x.conjugate(), Complex::new(42, 0));
            }

            #[test]
            fn negative_a_and_zero_b() {
                let x = Complex::new(-42, 0);
                assert_eq!(x.conjugate(), Complex::new(-42, 0));
            }

            #[test]
            fn positive_a_and_positive_b() {
                let x = Complex::new(10, 20);
                assert_eq!(x.conjugate(), Complex::new(10, -20));
            }

            #[test]
            fn positive_a_and_negative_b() {
                let x = Complex::new(10, -20);
                assert_eq!(x.conjugate(), Complex::new(10, 20));
            }

            #[test]
            fn negative_a_and_positive_b() {
                let x = Complex::new(-10, 20);
                assert_eq!(x.conjugate(), Complex::new(-10, -20));
            }

            #[test]
            fn negative_a_and_negative_b() {
                let x = Complex::new(-10, -20);
                assert_eq!(x.conjugate(), Complex::new(-10, 20));
            }
        }
    }

    mod traits {
        mod add {
            mod complex {
                use crate::complex::Complex;

                #[test]
                fn range_between_minus_ten_to_ten() {
                    for a1 in -10..=10 {
                        for b1 in -10..=10 {
                            for a2 in -10..=10 {
                                for b2 in -10..=10 {
                                    let x = Complex::new(a1, b1);
                                    let y = Complex::new(a2, b2);
                                    assert_eq!(x + y, Complex::new(a1 + a2, b1 + b2));
                                }
                            }
                        }
                    }
                }
            }
        }

        mod add_assign {
            mod complex {
                use crate::complex::Complex;

                #[test]
                fn range_between_minus_ten_to_ten() {
                    for a1 in -10..=10 {
                        for b1 in -10..=10 {
                            for a2 in -10..=10 {
                                for b2 in -10..=10 {
                                    let mut x = Complex::new(a1, b1);
                                    x += Complex::new(a2, b2);
                                    assert_eq!(x, Complex::new(a1 + a2, b1 + b2));
                                }
                            }
                        }
                    }
                }
            }
        }

        mod default {
            use crate::complex::Complex;

            #[test]
            fn default() {
                let x = Complex::default();
                assert_eq!(x.a(), 0f64);
                assert_eq!(x.b(), 0f64);
            }
        }

        mod display {
            use crate::complex::Complex;

            #[test]
            fn zero_a_and_zero_b() {
                let x = Complex::new(0, 0);
                assert_eq!(format!("{}", x), "0");
            }

            #[test]
            fn zero_a_and_positive_b() {
                let x = Complex::new(0, 42);
                assert_eq!(format!("{}", x), "42i");
            }

            #[test]
            fn zero_a_and_negative_b() {
                let x = Complex::new(0, -42);
                assert_eq!(format!("{}", x), "-42i");
            }

            #[test]
            fn positive_a_and_zero_b() {
                let x = Complex::new(42, 0);
                assert_eq!(format!("{}", x), "42");
            }

            #[test]
            fn negative_a_and_zero_b() {
                let x = Complex::new(-42, 0);
                assert_eq!(format!("{}", x), "-42");
            }

            #[test]
            fn positive_a_and_positive_b() {
                let x = Complex::new(10, 20);
                assert_eq!(format!("{}", x), "10 + 20i");
            }

            #[test]
            fn positive_a_and_negative_b() {
                let x = Complex::new(10, -20);
                assert_eq!(format!("{}", x), "10 - 20i");
            }

            #[test]
            fn negative_a_and_positive_b() {
                let x = Complex::new(-10, 20);
                assert_eq!(format!("{}", x), "-10 + 20i");
            }

            #[test]
            fn negative_a_and_negative_b() {
                let x = Complex::new(-10, -20);
                assert_eq!(format!("{}", x), "-10 - 20i");
            }
        }

        mod div {
            mod into_f64 {
                use crate::complex::Complex;

                #[test]
                fn range_between_minus_ten_to_ten() {
                    for a in -10..=10 {
                        for b in -10..=10 {
                            for amount in -10..=10 {
                                if amount == 0 {
                                    continue;
                                }

                                let x = Complex::new(a, b);
                                assert_eq!(
                                    x / amount,
                                    Complex::new(
                                        f64::from(a) / f64::from(amount),
                                        f64::from(b) / f64::from(amount),
                                    ),
                                );
                            }
                        }
                    }
                }

                #[test]
                #[should_panic]
                fn with_exactly_zero() {
                    let x = Complex::new(10, 20);
                    let _ = x / 0;
                }

                #[test]
                #[should_panic]
                fn with_positive_epsilon_over_two() {
                    let x = Complex::new(10, 20);
                    let _ = x / (0.5 * std::f64::EPSILON);
                }

                #[test]
                #[should_panic]
                fn with_negative_epsilon_over_two() {
                    let x = Complex::new(10, 20);
                    let _ = x / (-0.5 * std::f64::EPSILON);
                }
            }

            mod complex {
                use crate::complex::Complex;

                #[test]
                fn four_plus_two_i_and_one_plus_one_i() {
                    let x = Complex::new(4, 2);
                    let y = Complex::new(1, 1);
                    assert_eq!(x / y, Complex::new(3, -1));
                }

                #[test]
                fn five_and_three_minus_four_i() {
                    let x = Complex::new(5, 0);
                    let y = Complex::new(3, -4);
                    assert_eq!(x / y, Complex::new(0.6, 0.8));
                }

                #[test]
                fn three_minus_two_i_and_i() {
                    let x = Complex::new(3, -2);
                    let y = Complex::new(0, 1);
                    assert_eq!(x / y, Complex::new(-2, -3));
                }

                #[test]
                fn six_i_and_three_minus_twelve_i() {
                    let x = Complex::new(0, 6);
                    let y = Complex::new(3, -12);
                    assert_eq!(
                        x / y,
                        Complex::new(-0.470_588_235_294_117_64f64, 0.117_647_058_823_529_41f64),
                    );
                }

                #[test]
                fn four_plus_sixteen_i_and_four() {
                    let x = Complex::new(4, 16);
                    let y = Complex::new(4, 0);
                    assert_eq!(x / y, Complex::new(1, 4));
                }

                #[test]
                fn one_plus_one_i_and_three_plus_four_i() {
                    let x = Complex::new(1, 1);
                    let y = Complex::new(3, 4);
                    assert_eq!(x / y, Complex::new(7. / 25., -(1. / 25.)));
                }

                #[test]
                fn two_plus_eight_i_and_one_plus_two_i() {
                    let x = Complex::new(2, 8);
                    let y = Complex::new(1, 2);
                    assert_eq!(x / y, Complex::new(3.6, 0.8));
                }
            }
        }

        mod div_assign {
            mod into_f64 {
                use crate::complex::Complex;

                #[test]
                fn range_between_minus_ten_to_ten() {
                    for a in -10..=10 {
                        for b in -10..=10 {
                            for amount in -10..=10 {
                                if amount == 0 {
                                    continue;
                                }

                                let mut x = Complex::new(a, b);
                                x /= amount;
                                assert_eq!(
                                    x,
                                    Complex::new(
                                        f64::from(a) / f64::from(amount),
                                        f64::from(b) / f64::from(amount),
                                    ),
                                );
                            }
                        }
                    }
                }

                #[test]
                #[should_panic]
                fn with_exactly_zero() {
                    let mut x = Complex::new(10, 20);
                    x /= 0;
                }

                #[test]
                #[should_panic]
                fn with_positive_epsilon_over_two() {
                    let mut x = Complex::new(10, 20);
                    x /= 0.5 * std::f64::EPSILON;
                }

                #[test]
                #[should_panic]
                fn with_negative_epsilon_over_two() {
                    let mut x = Complex::new(10, 20);
                    x /= -0.5 * std::f64::EPSILON;
                }
            }

            mod complex {
                use crate::complex::Complex;

                #[test]
                fn four_plus_two_i_and_one_plus_one_i() {
                    let mut x = Complex::new(4, 2);
                    x /= Complex::new(1, 1);
                    assert_eq!(x, Complex::new(3, -1));
                }

                #[test]
                fn five_and_three_minus_four_i() {
                    let mut x = Complex::new(5, 0);
                    x /= Complex::new(3, -4);
                    assert_eq!(x, Complex::new(0.6, 0.8));
                }

                #[test]
                fn three_minus_two_i_and_i() {
                    let mut x = Complex::new(3, -2);
                    x /= Complex::new(0, 1);
                    assert_eq!(x, Complex::new(-2, -3));
                }

                #[test]
                fn six_i_and_three_minus_twelve_i() {
                    let mut x = Complex::new(0, 6);
                    x /= Complex::new(3, -12);
                    assert_eq!(
                        x,
                        Complex::new(-0.470_588_235_294_117_64f64, 0.117_647_058_823_529_41f64),
                    );
                }

                #[test]
                fn four_plus_sixteen_i_and_four() {
                    let mut x = Complex::new(4, 16);
                    x /= Complex::new(4, 0);
                    assert_eq!(x, Complex::new(1, 4));
                }

                #[test]
                fn one_plus_one_i_and_three_plus_four_i() {
                    let mut x = Complex::new(1, 1);
                    x /= Complex::new(3, 4);
                    assert_eq!(x, Complex::new(7. / 25., -(1. / 25.)));
                }

                #[test]
                fn two_plus_eight_i_and_one_plus_two_i() {
                    let mut x = Complex::new(2, 8);
                    x /= Complex::new(1, 2);
                    assert_eq!(x, Complex::new(3.6, 0.8));
                }
            }
        }

        mod mul {
            mod into_f64 {
                use crate::complex::Complex;

                #[test]
                fn range_between_minus_ten_to_ten() {
                    for a in -10..=10 {
                        for b in -10..=10 {
                            for amount in -10..=10 {
                                let x = Complex::new(a, b);
                                assert_eq!(
                                    x * amount,
                                    Complex::new(
                                        f64::from(a) * f64::from(amount),
                                        f64::from(b) * f64::from(amount),
                                    ),
                                );
                            }
                        }
                    }
                }
            }

            mod complex {
                use crate::complex::Complex;

                #[test]
                fn range_between_minus_ten_to_ten() {
                    for a1 in -10..=10 {
                        for b1 in -10..=10 {
                            for a2 in -10..=10 {
                                for b2 in -10..=10 {
                                    let x = Complex::new(a1, b1);
                                    let y = Complex::new(a2, b2);
                                    assert_eq!(
                                        x * y,
                                        Complex::new(a1 * a2 - b1 * b2, a1 * b2 + b1 * a2),
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }

        mod mul_assign {
            mod into_f64 {
                use crate::complex::Complex;

                #[test]
                fn range_between_minus_ten_to_ten() {
                    for a in -10..=10 {
                        for b in -10..=10 {
                            for amount in -10..=10 {
                                let mut x = Complex::new(a, b);
                                x *= amount;
                                assert_eq!(
                                    x,
                                    Complex::new(
                                        f64::from(a) * f64::from(amount),
                                        f64::from(b) * f64::from(amount),
                                    ),
                                );
                            }
                        }
                    }
                }
            }

            mod complex {
                use crate::complex::Complex;

                #[test]
                fn range_between_minus_ten_to_ten() {
                    for a1 in -10..=10 {
                        for b1 in -10..=10 {
                            for a2 in -10..=10 {
                                for b2 in -10..=10 {
                                    let mut x = Complex::new(a1, b1);
                                    x *= Complex::new(a2, b2);
                                    assert_eq!(
                                        x,
                                        Complex::new(a1 * a2 - b1 * b2, a1 * b2 + b1 * a2),
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }

        mod neg {
            mod complex {
                use crate::complex::Complex;

                #[test]
                fn zero_a_and_zero_b() {
                    let x = Complex::new(0, 0);
                    assert_eq!(-x, Complex::new(0, 0));
                }

                #[test]
                fn zero_a_and_positive_b() {
                    let x = Complex::new(0, 42);
                    assert_eq!(-x, Complex::new(0, -42));
                }

                #[test]
                fn zero_a_and_negative_b() {
                    let x = Complex::new(0, -42);
                    assert_eq!(-x, Complex::new(0, 42));
                }

                #[test]
                fn positive_a_and_zero_b() {
                    let x = Complex::new(42, 0);
                    assert_eq!(-x, Complex::new(-42, 0));
                }

                #[test]
                fn negative_a_and_zero_b() {
                    let x = Complex::new(-42, 0);
                    assert_eq!(-x, Complex::new(42, 0));
                }

                #[test]
                fn positive_a_and_positive_b() {
                    let x = Complex::new(10, 20);
                    assert_eq!(-x, Complex::new(-10, -20));
                }

                #[test]
                fn positive_a_and_negative_b() {
                    let x = Complex::new(10, -20);
                    assert_eq!(-x, Complex::new(-10, 20));
                }

                #[test]
                fn negative_a_and_positive_b() {
                    let x = Complex::new(-10, 20);
                    assert_eq!(-x, Complex::new(10, -20));
                }

                #[test]
                fn negative_a_and_negative_b() {
                    let x = Complex::new(-10, -20);
                    assert_eq!(-x, Complex::new(10, 20));
                }
            }
        }

        mod sub {
            mod complex {
                use crate::complex::Complex;

                #[test]
                fn range_between_minus_ten_to_ten() {
                    for a1 in -10..=10 {
                        for b1 in -10..=10 {
                            for a2 in -10..=10 {
                                for b2 in -10..=10 {
                                    let x = Complex::new(a1, b1);
                                    let y = Complex::new(a2, b2);
                                    assert_eq!(x - y, Complex::new(a1 - a2, b1 - b2));
                                }
                            }
                        }
                    }
                }
            }
        }

        mod sub_assign {
            mod complex {
                use crate::complex::Complex;

                #[test]
                fn range_between_minus_ten_to_ten() {
                    for a1 in -10..=10 {
                        for b1 in -10..=10 {
                            for a2 in -10..=10 {
                                for b2 in -10..=10 {
                                    let mut x = Complex::new(a1, b1);
                                    x -= Complex::new(a2, b2);
                                    assert_eq!(x, Complex::new(a1 - a2, b1 - b2));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
