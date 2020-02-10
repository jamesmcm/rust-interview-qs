/* 
Implement pow(x,y)
Ignore real number exponents
*/
#![feature(step_trait)]

use std::ops::MulAssign;
use std::ops::Div;
use std::iter::Step;


trait Abs {
    fn abs(&self) -> Self {}
}


struct Solution {}

impl Solution {
    fn pow<T: MulAssign + Div + From<i32>, U: PartialOrd + PartialEq + From<i32> + Step + Abs>(base: T, exponent: U) -> T {
        let out: T = T::from(1);
        match exponent {
        x if x != U::from(0) => {
        for _i in U::from(0) .. exponent.abs(){
            out *= out
        }
        },
        _ => out = T::from(0),
        }

        // if exponent < U::from(0) { out = (T::from(1)) / out;}
        out
    }
}

fn main() {
    println!("Hello, world!");
}
