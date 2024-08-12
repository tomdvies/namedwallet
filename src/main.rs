mod fpelem;
mod eccrypto;
use fpelem::*;

fn main() {
    eccrypto::test();
    let a = FpElem::new(10, 23);
    //let b = FpElem::new(11, 23).unwrap();
    //let c = FpElem::new(1, 23).unwrap();
    //println!("{:?}", a.pow(3) * a.pow(-1) * a.pow(-2));
    //println!("{:?}", c * b);
    //println!("{:?}", a.pow(&b));
    //println!("{:?}", a.powi(2));
    //println!("{:?}", (a, b, c));
}
