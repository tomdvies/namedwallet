mod fieldelem;
use fieldelem::*;

fn main() {
    let a = FieldElement::new(10, 23).unwrap();
    let b = FieldElement::new(11, 23).unwrap();
    let c = FieldElement::new(1, 23).unwrap();
    println!("{:?}", a.powi(3) * a.powi(-1) * a.powi(-2));
    println!("{:?}", c * b);
    println!("{:?}", a.pow(&b));
    println!("{:?}", a.powi(2));
    println!("{:?}", (a, b, c));
}
