mod fpelem;
mod eccrypto;
use fpelem::*;
extern crate primitive_types;
use primitive_types::U512;

fn main() {
    eccrypto::test();
    return;
    let a = FpElem::new(U512::from(10), U512::from(23));
    let b = FpElem::new(U512::from(11), U512::from(23));
    let c = FpElem::new(U512::from(1), U512::from(23));
    println!("{:?}", a.pow(U512::from(3)) * a.inv() * a.inv().pow(U512::from(2)));
    println!("{:?}", c * b.pow(U512::from(90)));
    println!("{:?}", (a, b, c));
}
