// src/sample.rs
pub struct Person {
    pub name: String,
    pub age: u32,
}

pub enum Color {
    Red,
    Green,
    Blue,
}

pub trait Greet {
    fn greet(&self) -> String;
}
