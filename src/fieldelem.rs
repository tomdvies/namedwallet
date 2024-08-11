use std::ops::{Add, Mul, Sub};

// https://stackoverflow.com/questions/45918104/how-to-do-arithmetic-modulo-another-number-without-overflow
fn mul_mod(a: u64, b: u64, m: u64) -> u64 {
    let (a, b, m) = (a as u128, b as u128, m as u128);
    ((a * b) % m) as u64
}

// assumes prime base - see FlittleT, also UofCambridge QIC sheet 3 lol
fn pow_mod(mut base: u64, exp: i64, modulus: u64) -> u64 {
    let mut exp = (exp.rem_euclid(i64::try_from(modulus).unwrap()-1)) as u128;
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp = exp >> 1;
        base = base * base % modulus
    }
    result
}

// u128 to avoid overflow as in mul_mod
fn add_mod(a: u64, b: u64, m: u64) -> u64 {
    let (a, b, m) = (a as u128, b as u128, m as u128);
    ((a + b) % m) as u64
}

fn sub_mod(a: u64, b: u64, m: u64) -> u64 {
    let (a, b, m) = (a as i128, b as i128, m as i128);
    ((a - b).rem_euclid(m)) as u64
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub struct FieldElement {
    number: u64,
    prime: u64,
}

impl FieldElement {
    pub fn new(number: u64, prime: u64) -> Result<Self, String> {
        if !(number >= prime) {
            return Ok(FieldElement { number, prime });
        } else {
            return Err("Element must be less than p".to_string());
        }
    }
}

pub trait Power {
    fn powi(&self, exponent: i64) -> Self;
    fn pow(&self, exponent: &Self) -> Self;
}

impl Power for FieldElement {
    fn powi(&self, exponent: i64) -> Self {
        FieldElement {
            number: pow_mod(self.number, exponent, self.prime),
            prime: self.prime,
        }
    }

    fn pow(&self, exponent: &Self) -> Self {
        assert!(self.prime == exponent.prime, "Prime base must be the same");
        FieldElement {
            number: pow_mod(
                self.number,
                i64::try_from(exponent.number).unwrap(),
                self.prime,
            ),
            prime: self.prime
        }
    }
}

impl Sub for FieldElement {
    type Output = FieldElement;
    fn sub(self, toadd: FieldElement) -> FieldElement {
        assert!(self.prime == toadd.prime, "Prime base must be the same");
        FieldElement {
            number: sub_mod(self.number, toadd.number, self.prime),
            prime: self.prime,
        }
    }
}

impl Add for FieldElement {
    type Output = FieldElement;
    fn add(self, toadd: FieldElement) -> FieldElement {
        assert!(self.prime == toadd.prime, "Prime base must be the same");
        FieldElement {
            number: add_mod(self.number, toadd.number, self.prime),
            prime: self.prime,
        }
    }
}

impl Mul for FieldElement {
    type Output = FieldElement;
    fn mul(self, toadd: FieldElement) -> FieldElement {
        assert!(self.prime == toadd.prime, "Prime base must be the same");
        FieldElement {
            number: mul_mod(self.number, toadd.number, self.prime),
            prime: self.prime,
        }
    }
}
