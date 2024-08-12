// https://math.uchicago.edu/~may/REU2020/REUPapers/Shevchuk.pdf
// based loosely on the above paper

use crate::fpelem::*;
use std::fmt;
use std::fmt::Debug;
use std::ops::{Add, Mul, Sub};
extern crate primitive_types;
use primitive_types::U512;

const SECP256K1_A:u32 = 0;
const SECP256K1_B:u32 = 7;
const SECP256K1_GX: &str = "0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798";
const SECP256K1_GY: &str = "0x483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8";
const SECP256K1_P: &str = "0xfffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f";
const SECP256K1_N: &str = "0xfffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141";

pub fn test() {
    let n = U512::from(SECP256K1_N);
    let p = U512::from(SECP256K1_P);
    let gx = U512::from(SECP256K1_GX);
    let gy = U512::from(SECP256K1_GY);
    let a = U512::from(SECP256K1_A);
    let b = U512::from(SECP256K1_B);
    let G = ECPoint::new(Some((FpElem::new(gx,p),FpElem::new(gy,p))), FpElem::new(a,p), FpElem::new(b,p));
    println!("{:?}",G);
    println!("{:?}",G*n);

    //println!("hello");
    //let p = U512::from(23);
    //let prime = U512::from(223);
    //let a = FpElem::new(U512::from(0), prime);
    //let b = FpElem::new(U512::from(7), prime);
    //// (x1,y1)
    //let x1 = FpElem::new(U512::from(192), prime);
    //let y1 = FpElem::new(U512::from(105), prime);
    //let p1 = ECPoint::new(Some((x1, y1)), a, b);
    //// (x2,y2)
    //let x2 = FpElem::new(U512::from(17), prime);
    //let y2 = FpElem::new(U512::from(56), prime);
    //let p2 = ECPoint::new(Some((x2, y2)), a, b);

    //let k = ECPoint::new(Some((-1.0, -1.0)), 5.0, 7.0);
    //let k2 = ECPoint::new(Some((-1.0, 1.0)), 5.0, 7.0);
    //println!("{:?}", p1 * U512::from(170000));
    //println!("{:?}", p2);
    //let mut p3 = ECPoint::new(None,a,b);
    //for _ in 0..170000{ p3 = p3 + p1 }
    //println!("{:?}", p3);
}

// I feel there must be a way to define generics generically, and reuse them but im not sure what
// that would look like :(

#[derive(PartialEq, Clone, Copy)]
pub struct ECPoint<FE>
where
    FE: Add<Output = FE>
        + Sub<Output = FE>
        + Mul<U512, Output = FE>
        + Mul<Output = FE>
        + Power
        + Copy
        + Debug
        + PartialEq,
{
    // None = infinity here
    position: Option<(FE, FE)>,
    a: FE,
    b: FE,
}

impl<FE> Debug for ECPoint<FE>
where
    FE: Add<Output = FE>
        + Sub<Output = FE>
        + Mul<U512, Output = FE>
        + Mul<Output = FE>
        + Power
        + Copy
        + Debug
        + PartialEq,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some((x, y)) = self.position {
            return f
                .debug_struct("Point")
                .field("pos", &(x, y))
                .field("a", &self.a)
                .field("b", &self.b)
                .finish();
            //.field("b":self.b)
        } else {
            return f
                .debug_struct("Point")
                .field("pos", &"infinity")
                .field("a", &self.a)
                .field("b", &self.b)
                .finish();
        }
    }
}

impl<FE> ECPoint<FE>
where
    FE: Add<Output = FE>
        + Sub<Output = FE>
        + Mul<U512, Output = FE>
        + Mul<Output = FE>
        + Power
        + Copy
        + Debug
        + PartialEq,
{
    pub fn new(position: Option<(FE, FE)>, a: FE, b: FE) -> Self {
        if let Some((x, y)) = position {
            assert!(
                y.pow(U512::from(2)) == x.pow(U512::from(3)) + a * x + b,
                "Point must lie on the curve"
            );
            ECPoint { position, a, b }
        } else {
            ECPoint { position, a, b }
        }
    }

//    pub fn new_S256
}

impl<FE> Add for ECPoint<FE>
where
    FE: Add<Output = FE>
        + Sub<Output = FE>
        + Mul<U512, Output = FE>
        + Mul<Output = FE>
        + Power
        + Copy
        + Debug
        + PartialEq,
{
    type Output = ECPoint<FE>;
    fn add(self, toadd: Self) -> ECPoint<FE> {
        assert!(
            self.a == toadd.a && self.b == toadd.b,
            "Curves must be the same"
        );
        if let (Some((x1, y1)), Some((x2, y2))) = (self.position, toadd.position) {
            if (x1, y1) == (x2, y2) {
                let s =
                    (x1.pow(U512::from(2)) * U512::from(3) + self.a) * ((y1 * U512::from(2)).inv());
                let x3 = s.pow(U512::from(2)) - (x1 * U512::from(2));
                let y3 = s * (x1 - x3) - y1;
                return ECPoint {
                    position: Some((x3, y3)),
                    a: self.a,
                    b: self.b,
                };
            } else if x1 == x2 {
                return ECPoint {
                    position: None,
                    a: self.a,
                    b: self.b,
                };
            } else {
                let s = (y2 - y1) * ((x2 - x1).inv());
                let x3 = s.pow(U512::from(2)) - x1 - x2;
                let y3 = s * (x1 - x3) - y1;
                return ECPoint {
                    position: Some((x3, y3)),
                    a: self.a,
                    b: self.b,
                };
            }
        } else if let Some((_x, _y)) = self.position {
            return ECPoint {
                position: self.position.clone(),
                a: self.a,
                b: self.b,
            };
        } else if let Some((_x, _y)) = toadd.position {
            return ECPoint {
                position: toadd.position.clone(),
                a: self.a,
                b: self.b,
            };
        }
        self.clone()
    }
}

impl<FE> Mul<U512> for ECPoint<FE>
where
    FE: Add<Output = FE>
        + Sub<Output = FE>
        + Mul<U512, Output = FE>
        + Mul<Output = FE>
        + Power
        + Copy
        + Debug
        + PartialEq,
{
    type Output = ECPoint<FE>;
    fn mul(self, tomul: U512) -> ECPoint<FE> {
        let mut exp = tomul;
        let mut result = ECPoint{position:None, a:self.a, b:self.b};
        let mut current = self.clone();
        while exp != U512::zero(){
            if exp % U512::from(2) == U512::one(){
                result = result + current;
            }
            current = current +  current;
            exp = exp >> 1;
        }
        result
    }
}
