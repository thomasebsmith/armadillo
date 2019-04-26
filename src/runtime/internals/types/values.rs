use super::JSObject;
use super::JSString;
use super::JSSymbol;

pub enum Value<'a> {
    Boolean(bool),
    Null,
    Number(f64),
    Object(JSObject<'a>),
    String(JSString),
    Symbol(JSSymbol),
    Undefined,
}

impl<'a> Value<'a> {
    pub fn is_primitive(&self) -> bool {
        match self {
            Value::Object(_) => false,
            _ => true,
        }
    }
}
