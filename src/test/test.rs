pub struct Person {
    pub name: String,
    pub age: u32,
    pub house: House,
}

pub struct Town {
    pub houses: Vec<House>,
    pub residents: Vec<Person>,
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
