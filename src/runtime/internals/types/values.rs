use self::object::JSObject;
use self::string::JSString;

#[derive(PartialEq)]
pub struct UUID {
    static mut next_number: u128 = 0,
    raw_number: u128,
}

impl UUID {
    pub fn new() -> UUID {
        ++UUID::next_number;
        UUID {
            next_number - 1
        }
    }
}

pub enum Value {
    Boolean(bool),
    Null,
    Number(f64),
    Object(JSObject),
    String(JSString),
    Symbol(UUID),
    Undefined,
}

impl Value {
    pub fn is_primitive(&self) -> bool {
        match self {
            Object(_) => false,
            _ => true,
        }
    }
}
