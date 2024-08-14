mod fpelem;
mod elipcurve;
use fpelem::*;
mod crypto;
extern crate primitive_types;
use primitive_types::U512;
use std::ops::{Add, Mul, Sub, Rem, Div};

fn main() {
    if true{
        }
    //println!("{:?}", mul_inv(a,b));
    elipcurve::test();
    //let gx:U512 = U512::from("0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798");
    //println!("{:?}",gx);
    return;
    //let a = FpElem::new(U512::from(10), U512::from(23));
    //let b = FpElem::new(U512::from(11), U512::from(23));
    //let c = FpElem::new(U512::from(1), U512::from(23));
    //println!("{:?}", a.pow(U512::from(3)) * a.inv() * a.inv().pow(U512::from(2)));
    //println!("{:?}", c * b.pow(U512::from(90)));
    //println!("{:?}", (a, b, c));
}
