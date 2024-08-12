// https://math.uchicago.edu/~may/REU2020/REUPapers/Shevchuk.pdf
// based loosely on the above paper

use crate::fpelem::*;
use std::fmt;
use std::fmt::Debug;
use std::ops::{Add, Mul, Sub};

pub fn test() {
    println!("hello");
    let p: u64 = 23;
    let prime = 223;
    let a = FpElem::new(0,prime);
    let b = FpElem::new(7,prime);
    // (x1,y1)
    let x1= FpElem::new(192,prime);
    let y1= FpElem::new(105,prime);
    let p1 = ECPoint::new(Some((x1,y1)),a,b);
    // (x2,y2)
    let x2= FpElem::new(17,prime);
    let y2= FpElem::new(56,prime);
    let p2 = ECPoint::new(Some((x2,y2)),a,b);

    //let k = ECPoint::new(Some((-1.0, -1.0)), 5.0, 7.0);
    //let k2 = ECPoint::new(Some((-1.0, 1.0)), 5.0, 7.0);
    println!("{:?}", p1);
    println!("{:?}", p2);
    println!("{:?}", p1+p2);
}

// I feel there must be a way to define generics generically, and reuse them but im not sure what
// that would look like :(

// for testing with elliptic curves on R
impl Power for f64 {
    fn pow(&self, exponent: i64) -> Self {
        f64::powi(*self, i32::try_from(exponent).unwrap())
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct ECPoint<FE>
where
    FE: Add<Output = FE> + Sub<Output = FE> + Mul<Output = FE> + Power + Copy + Debug + PartialEq,
{
    // None = infinity here
    position: Option<(FE, FE)>,
    a: FE,
    b: FE,
}

impl<FE> Debug for ECPoint<FE>
where
    FE: Add<Output = FE> + Sub<Output = FE> + Mul<Output = FE> + Power + Copy + Debug + PartialEq,
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
    FE: Add<Output = FE> + Sub<Output = FE> + Mul<Output = FE> + Power + Copy + Debug + PartialEq,
{
    pub fn new(position: Option<(FE, FE)>, a: FE, b: FE) -> Self {
        if let Some((x, y)) = position {
            assert!(
                y.pow(2) == x.pow(3) + a * x + b,
                "Point must lie on the curve"
            );
            ECPoint { position, a, b }
        } else {
            ECPoint { position, a, b }
        }
    }

    fn mul_field_elem(self, elem: FE, scalar: u64) -> FE {
        let mut out = elem.clone();
        for _ in 0..scalar - 1 {
            out = out + elem;
        }
        out
    }
}

impl<FE> Add for ECPoint<FE>
where
    FE: Add<Output = FE> + Sub<Output = FE> + Mul<Output = FE> + Power + Copy + Debug + PartialEq,
{
    type Output = ECPoint<FE>;
    fn add(self, toadd: Self) -> ECPoint<FE> {
        assert!(
            self.a == toadd.a && self.b == toadd.b,
            "Curves must be the same"
        );
        if let (Some((x1, y1)), Some((x2, y2))) = (self.position, toadd.position) {
            if (x1, y1) == (x2, y2) {
                let s = (self.mul_field_elem(x1.pow(2), 3) + self.a)
                    * (self.mul_field_elem(y1, 2).pow(-1));
                let x3 = s.pow(2) - self.mul_field_elem(x1, 2);
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
                let s = (y2 - y1) * ((x2 - x1).pow(-1));
                let x3 = s.pow(2) - x1 - x2;
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
