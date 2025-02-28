/*
   Operator Overloading in Rust:

   In Rust, operator overloading is achieved by implementing specific traits defined in the standard library.
   This allows developers to define custom behavior for operators (e.g., +, +=, -) on user-defined types.
   In this module, we illustrate operator overloading through a generic Complex number type.
*/

use std::cmp::PartialEq;
use std::fmt::Debug;
use std::ops::{Add, AddAssign, Neg};

/// A generic Complex number struct representing a number with a real and imaginary part.
///
/// The #[derive] attribute automatically implements common traits such as Debug, PartialEq, Eq, Ord, and PartialOrd,
/// allowing for easy printing, comparison, and ordering of Complex numbers.
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
struct Complex<T> {
    re: T,
    im: T,
}

impl<T> Complex<T> {
    /// Constructs a new Complex number with the provided real (re) and imaginary (im) components.
    fn new(re: T, im: T) -> Complex<T> {
        Complex { re, im }
    }
}

/// Implements addition for Complex numbers by overloading the `+` operator through the Add trait.
///
/// This lets us add two Complex values with the `+` operator. The generic type T must implement
/// the Add trait where T + T yields a T.
impl<T> Add for Complex<T>
where
    T: Add<Output = T>, // T must support addition with an output of the same type.
{
    type Output = Complex<T>;

    /// Consumes `self` and `rhs` (right-hand side) to produce a new Complex number.
    /// Both the real and imaginary parts are added component-wise.
    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

/// Implements the addition assignment operator (`+=`) for Complex numbers via the AddAssign trait.
///
/// This allows an existing Complex value to be incremented by another Complex value using the `+=` syntax.
impl<T> AddAssign for Complex<T>
where
    T: AddAssign<T>,
{
    /// Modifies `self` by adding the corresponding parts from `rhs`.
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

/// Implements the unary negation operator (`-`) by overloading the Neg trait.
///
/// This provides the ability to negate a Complex number, producing a new Complex number with both
/// the real and imaginary parts negated.
impl<T> Neg for Complex<T>
where
    T: Neg<Output = T>,
{
    type Output = Complex<T>;

    /// Returns a new Complex number with both the real and imaginary components negated.
    fn neg(self) -> Self::Output {
        Complex {
            re: -self.re,
            im: -self.im,
        }
    }
}

// Partial Equality Explanation:
// The PartialEq trait is used to compare instances for equality. For the Complex struct, equality is based on
// the condition that both the real part and imaginary part of two Complex numbers are equal.
// We can implement PartialEq manually (as shown in the commented-out code below), but deriving it is concise
// and sufficient for most cases.
// Note: Special values like NAN (Not a Number) have unique behavior because, according to IEEE standards,
// NAN is not considered equal to itself.

/*
impl<T> PartialEq for Complex<T>
where
    T: PartialEq,
{
    fn eq(&self, rhs: &Self) -> bool {
        self.re == rhs.re && self.im == rhs.im
    }
}
*/

/// Demonstrates operator overloading for Complex numbers.
///
/// This demo function shows usage of overloaded operators including addition, addition assignment,
/// negation, and equality checks in Rust.
pub fn operator_overloading_demo() {
    println!("Operator Overloading Demonstration:");

    let a = Complex::new(1, 2);
    let b = Complex::new(3, 4);

    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("a + b: {:?}", a + b);

    let c = Complex::new(1.0, 2.0);
    let d = Complex::new(3.0, 4.0);
    println!("c + d: {:?}", c + d);

    let mut e = Complex::new(1.0, 2.0);
    let d = Complex::new(3.0, 4.0);
    e += d;
    println!("After e += d, e: {:?}", e);

    let f = Complex::new(1, 2);
    println!("-f: {:?}", -f);

    let h = Complex::new(1, 2);
    let i = Complex::new(3, 4);
    let j = Complex::new(1, 2);
    println!("h: {:?} , i: {:?} => h == i: {}", h, i, h == i);
    println!("h: {:?} , j: {:?} => h == j: {}", h, j, h == j);
}
