use time::{Duration, PrimitiveDateTime as DateTime};


fn main() {
    println!("Exercism");
}

pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}

const ONE_GIGA: i64 = 1000000000;
pub fn after(start: DateTime) -> DateTime {
    start + Duration::seconds(ONE_GIGA)
}
 
