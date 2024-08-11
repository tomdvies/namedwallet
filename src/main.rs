use std::ops::{Add, Mul};

// https://stackoverflow.com/questions/45918104/how-to-do-arithmetic-modulo-another-number-without-overflow
fn mul_mod(a: u64, b: u64, m: u64) -> u64 {
    let (a, b, m) = (a as u128, b as u128, m as u128);
    ((a * b) % m) as u64
}

// assumes prime base - see FlittleT
fn mod_pow(mut base: u64, exp: i64, modulus: u64) -> u64 {
    let mut exp = (exp % i64::try_from(modulus).unwrap()) as u128;
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

fn add_mod(a: u64, b: u64, m: u64) -> u64 {
    let (a, b, m) = (a as u128, b as u128, m as u128);
    ((a + b) % m) as u64
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct FieldElement {
    number: u64,
    prime: u64,
}

impl FieldElement {
    fn new(number: u64, prime: u64) -> Result<Self, String> {
        if !(number >= prime) {
            return Ok(FieldElement { number, prime });
        } else {
            return Err("Element must be less than p".to_string());
        }
    }
}

enum Exponent<'a> {
    Int(i64),
    Struct(&'a FieldElement),
}

trait Power {
    fn pow(&self, exponent: Exponent) -> Self;
}

impl Power for FieldElement {
    fn pow(&self, exponent: Exponent) -> Self {
        match exponent{
        Exponent::Int(i) => {
        FieldElement {
            number: mod_pow(self.number, i, self.prime),
            prime: self.prime,
        }
        }
        Exponent::Struct(field_element) => {
        assert!(self.prime == field_element.prime, "Prime base must be the same");
        FieldElement {
            number: mod_pow(self.number, i64::try_from(field_element.number).unwrap(), self.prime),
            prime: self.prime,
        }
        }
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

fn main() {
    let a = FieldElement::new(10, 23).unwrap();
    let b = FieldElement {
        number: 2,
        prime: 23,
    };
    let c = FieldElement {
        number: 3,
        prime: 23,
    };
    println!("{:?}", a + b);
    println!("{:?}", a * c);
    println!("{:?}", a.pow(&b));
    println!("{:?}", (a, b, c));
}
