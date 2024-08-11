#[derive(Debug, PartialEq)]
struct FieldElement {
    number: u32,
    prime: u32,
}

impl FieldElement {
    fn new(number: u32, prime:u32) -> Result<Self, String> {
        if !(number>=prime){
            return Ok(FieldElement{number,prime});
        } else {
            return Err("Element must be less than p".to_string());
        }
    }
}

fn main(){
    let a = FieldElement::new(10,17).unwrap();
    let b = FieldElement {
        number: 12,
        prime: 17,
    };
    let c = FieldElement {
        number: 10,
        prime: 17,
    };
    println!("{:?}", a == b);
    println!("{:?}", a == c);
}
