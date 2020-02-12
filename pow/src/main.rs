/*
Implement pow(x,y)
Ignore real number exponents
*/

#![feature(step_trait)]

extern crate num_integer;
extern crate num_traits;
use num_integer::Integer;
use num_traits::identities::One;
use num_traits::identities::Zero;
use num_traits::sign::abs;
use num_traits::NumAssign;
use num_traits::sign::Signed;
use std::iter::Step;
use num_traits::FromPrimitive;

struct Solution {}

impl Solution {
    fn pow<T: NumAssign + One + Zero + Copy + FromPrimitive, U: Integer + NumAssign + One + Zero + Signed + Step + Copy>(
        base: T,
        exponent: U,
    ) -> T {
        let mut out: T = base;
        match exponent {
            x if x != num_traits::Zero::zero() => {
                for _i in num_traits::One::one()..abs(exponent) {
                    out *= out
                }
            }
            _ => {out = num_traits::One::one()},
        }

        if exponent < num_traits::Zero::zero()  { out = T::from_i32(1).unwrap() / out ;}
        out
    }
}

fn main() {
    println!("{:?}", Solution::pow(2, 4));
}
