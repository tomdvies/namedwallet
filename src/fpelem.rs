use std::ops::{Add, Mul, Sub};
extern crate primitive_types;
use primitive_types::U512;

// brainrot but works
#[derive(Clone, Copy, Debug)]
struct U512AndSign {
    value: U512,
    is_negative: bool,
}

fn mul_inv(mut a: U512, mut b: U512) -> U512 {
    if b <= U512::one() {
        return U512::zero();
    }

    let b0 = b;
    let mut x0 = U512AndSign {
        value: U512::zero(),
        is_negative: false,
    }; // b = 1*b + 0*a
    let mut x1 = U512AndSign {
        value: U512::one(),
        is_negative: false,
    }; // a = 0*b + 1*a

    while a > U512::one() {
        if b == U512::zero() {
            // Means original A and B were not co-prime, so there is no inverse
            return U512::zero();
        }

        let q = a / b;
        let t = b;
        b = a % b;
        a = t;

        // (x0, x1) := (x1 - q * x0, x0)
        let t2 = x0;
        let qx0 = q.saturating_mul(x0.value); // Handle multiplication safely

        if x0.is_negative != x1.is_negative {
            x0.value = x1.value.saturating_add(qx0);
            x0.is_negative = x1.is_negative;
        } else {
            if x1.value >= qx0 {
                x0.value = x1.value - qx0;
                x0.is_negative = x1.is_negative;
            } else {
                x0.value = qx0 - x1.value;
                x0.is_negative = !x1.is_negative;
            }
        }
        x1 = t2;
    }
    if x1.is_negative {
        b0.saturating_sub(x1.value)
    } else {
        x1.value
    }
}

// not overflow safe - v bad
// fine for now as all things in btc at 256 bit
// as of now unused, but is much faster
//fn mul_mod(a: U512, b: U512, m: U512) -> U512 {
//    if let Some(x) = a.checked_mul(b) {
//        return x % m;
//    } else {
//        panic!("Overflow in mul_mod");
//    }
//}

// overflow safe multiplciation - too slow
fn mul_mod_safe(a: U512, b: U512, m: U512) -> U512 {
    let mut b = b % m;
    let a = a % m;
    let mut result = U512::zero();
    let mut current = a;
    while b != U512::zero() {
        if (b & U512::one()) != U512::zero() {
            result = add_mod(result, current, m);
        }
        current = add_mod(current, current, m);
        b = b >> 1;
    }
    result
}

// assumes prime modulus - see FlittleT, also UofCambridge QIC sheet 3 lol
// overflow safe iff mul_mod is
fn pow_mod(mut base: U512, exp: U512, modulus: U512) -> U512 {
    let mut exp = exp % (modulus - 1);
    if modulus == U512::one() {
        return U512::zero();
    }
    let mut result = U512::one();
    base = base % modulus;
    while exp > U512::zero() {
        // apply leading bit
        if (exp & U512::one()) != U512::zero() {
            result = mul_mod_safe(result, base, modulus);
        }
        // chop off leading bit
        exp = exp >> 1;
        base = mul_mod_safe(base, base, modulus);
    }
    result
}

// overflow safe
fn add_mod(a: U512, b: U512, m: U512) -> U512 {
    let (a, b) = (a % m, b % m);
    let (x, reduce) = a.overflowing_add(b);
    if reduce {
        return x.overflowing_sub(m).0 % m;
    } else {
        return x % m;
    }
    //if let Some(x) = a.checked_add(b) {
    //    return x % m;
    //} else {
    //    return a.overflowing_sub(m).0.overflowing_add(b).0 % m;
    //}
}

// overflow safe
fn sub_mod(a: U512, b: U512, m: U512) -> U512 {
    let (a, b) = (a % m, b % m);
    let (x, reduce) = a.overflowing_sub(b);
    if reduce {
        return x.overflowing_add(m).0 % m;
    } else {
        return x % m;
    }
    //let a = a % m;
    //let b = b % m;
    //if let Some(x) = a.checked_sub(b) {
    //    return x % m;
    //} else {
    //    return a.overflowing_add(m).0.overflowing_sub(b).0 % m;
    //}
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct FpElem {
    pub number: U512,
    pub prime: U512,
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

    fn inv(&self) -> Self {
        FpElem {
            number: mul_inv(self.number, self.prime),
            prime: self.prime,
        }
        //FpElem {
        //    number: pow_mod(self.number, self.prime - U512::from(2), self.prime),
        //    prime: self.prime,
        //}
    }
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
            number: mul_mod_safe(self.number, toadd.number, self.prime),
            prime: self.prime,
        }
    }
}

impl Mul<U512> for FpElem {
    type Output = FpElem;
    fn mul(self, toadd: U512) -> FpElem {
        FpElem {
            number: mul_mod_safe(self.number, toadd % self.prime, self.prime),
            prime: self.prime,
        }
    }
}
