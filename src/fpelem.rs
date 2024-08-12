use std::ops::{Add, Mul, Sub};
extern crate primitive_types;
use primitive_types::U512;

// none of these are overflow safe - v bad
fn mul_mod(a: U512, b: U512, m: U512) -> U512 {
    if let Some(x) = a.checked_mul(b) {
        return x % m;
    } else {
        panic!("Overflow in mul_mod");
    }
}

// assumes prime base - see FlittleT, also UofCambridge QIC sheet 3 lol
fn pow_mod(mut base: U512, exp: U512, modulus: U512) -> U512 {
    let mut exp = exp % (modulus - 1);
    if modulus == U512::one() {
        return U512::zero();
    }
    let mut result = U512::one();
    base = base % modulus;
    while exp > U512::zero() {
        // apply leading bit
        if exp % U512::from(2) == U512::one() {
            result = mul_mod(result, base, modulus);
        }
        // chop off leading bit
        exp = exp >> 1;
        base = mul_mod(base, base, modulus);
    }
    result
}

// not overflow safe
fn add_mod(a: U512, b: U512, m: U512) -> U512 {
    if let Some(x) = a.checked_add(b) {
        return x % m;
    } else {
        panic!("Overflow in add_mod")
    }
}

fn sub_mod(a: U512, b: U512, m: U512) -> U512 {
    let a = a %m;
    let b = b%m;
    if let Some(x) = a.checked_sub(b){
        return x % m;
    }
    else{
        return a.overflowing_add(m).0.overflowing_sub(b).0 % m;        
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct FpElem {
    number: U512,
    prime: U512,
}

impl FpElem {
    pub fn new(number: U512, prime: U512) -> Self {
        assert!(number < prime, "Elements of Fp must be less than p");
        FpElem { number, prime }
    }
}

pub trait Power {
    fn pow(&self, exponent: U512) -> Self;
    fn inv(&self) -> Self;
    //    fn powm(&self, exponent: &Self) -> Self;
}

impl Power for FpElem {
    fn pow(&self, exponent: U512) -> Self {
        FpElem {
            number: pow_mod(self.number, exponent, self.prime),
            prime: self.prime,
        }
    }

    fn inv(&self) -> Self{
        FpElem{number:pow_mod(self.number, self.prime-U512::from(2), self.prime), prime: self.prime}
    }

    //    fn powm(&self, exponent: &Self) -> Self {
    //        assert!(self.prime == exponent.prime, "Prime base must be the same");
    //        FpElem {
    //            number: pow_mod(
    //                self.number,
    //                i64::try_from(exponent.number).unwrap(),
    //                self.prime,
    //            ),
    //            prime: self.prime
    //        }
    //    }
}

impl Sub for FpElem {
    type Output = FpElem;
    fn sub(self, toadd: FpElem) -> FpElem {
        assert!(self.prime == toadd.prime, "Prime base must be the same");
        FpElem {
            number: sub_mod(self.number, toadd.number, self.prime),
            prime: self.prime,
        }
    }
}

impl Add for FpElem {
    type Output = FpElem;
    fn add(self, toadd: FpElem) -> FpElem {
        assert!(self.prime == toadd.prime, "Prime base must be the same");
        FpElem {
            number: add_mod(self.number, toadd.number, self.prime),
            prime: self.prime,
        }
    }
}

impl Mul for FpElem {
    type Output = FpElem;
    fn mul(self, toadd: FpElem) -> FpElem {
        assert!(self.prime == toadd.prime, "Prime base must be the same");
        FpElem {
            number: mul_mod(self.number, toadd.number, self.prime),
            prime: self.prime,
        }
    }
}

impl Mul<U512> for FpElem {
    type Output = FpElem;
    fn mul(self, toadd: U512) -> FpElem {
        FpElem {
            number: mul_mod(self.number, toadd % self.prime, self.prime),
            prime: self.prime,
        }
    }
}
