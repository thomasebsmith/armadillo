use super::types::{JSFunction, JSObject, Value};

pub struct PropertyDescriptor<'a> {
    value: Value<'a>,
    writable: bool,
    enumerable: bool,
    configurable: bool,
    get: Option<JSFunction>,
    set: Option<JSFunction>,
}

impl<'a> PropertyDescriptor<'a> {
    pub fn new(value: Value<'a>) -> Self {
        Self {
            value: value,
            writable: false,
            enumerable: false,
            configurable: false,
            get: None,
            set: None,
        }
    }

    pub fn get_value(&self, receiver: &mut JSObject) -> &Value<'a> {
        // TODO
        &self.value
    }

    pub fn set_value(
        &mut self,
        value: Value<'a>, 
        receiver: &mut JSObject
    ) -> bool {
        // TODO
        self.value = value;
        true
    }
}
