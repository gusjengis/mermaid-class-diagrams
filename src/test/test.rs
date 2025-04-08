// src/sample.rs
pub struct Person {
    pub name: String,
    pub age: u32,
    pub house: House,
}

struct House {
    pub addr: String,
    pub color: Color,
}

pub enum Color {
    Red,
    Green,
    Blue,
}

pub trait Greet {
    fn greet(&self) -> String;
}
