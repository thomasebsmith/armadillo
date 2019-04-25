use std::collections::HashMap;

use self::function::JSFunction;
use self::object::JSObject;
use self::string::JSString;
use self::values::UUID;
use self::values::Value;

enum Key {
    String(JSString),
    Symbol(UUID),
}

pub struct AttributedValue {
    value: Value,
    writable: bool,
    enumerable: bool,
    configurable: bool,
    get: Option<JSFunction>,
    set: Option<JSFunction>,
}

impl AttributedValue {
    pub fn new(value: Value = Undefined) -> AttributedValue {
        AttributedValue {
            value: value,
            writable: false,
            enumerable: false,
            configurable: false,
            get: None,
            set: None,
        }
    }

    pub fn get_value(&self) -> &Value {
        // TODO
        self.value
    }

    pub fn set_value(&mut self, value: Value, receiver: &mut JSObject) {
        // TODO
        self.value = value
        true
    }
}

pub enum Prototype {
    Null,
    Object(&JSObject),
}

pub struct JSObject {
    function: Option<JSFunction>,
    map: HashMap<Key, AttributedValue>,
    prototype: Prototype,
}

impl JSObject {
    pub fn from_prototype(prototype: Prototype) -> Object {
        Object {
            prototype: prototype
        }
    }

    pub fn from_primitive(primitive: Value) -> Object {
        Object {
            prototype: Null // TODO
        }
    }

    pub fn get_prototype_of(&self) -> Prototype {
        self.prototype
    }

    pub fn set_prototype_of(&mut self, prototype: Prototype) -> bool {
        // TODO
        self.prototype = prototype;
        true
    }

    pub fn is_extensible(&self) -> bool {
        // TODO
        true
    }

    pub fn prevent_extensions(&mut self) -> bool {
        // TODO
        false
    }

    pub fn get_own_property(&self, key: &Key) -> Option<&AttributedValue> {
        self.map[key]
    }

    pub fn define_own_property(&mut self, key: &Key, value: AttributedValue) {
        // TODO
        self.map.insert(key, value);
        true
    }

    pub fn has_property(&self, key: &Key) {
        self.map.contains(key)
    }

    pub fn get(&self, key: &Key, receiver: &mut JSObject) -> &Value {
        if let attributed_value = &self.map[key] {
            attributed_value.get_value(receiver)
        }
        else {
            Undefined
        }
    }

    pub fn set(
        &mut self,
        key: &Key,
        value: Value,
        receiver: &mut JSObject
    ) -> bool {
        // TODO
        if let mut attributed_value = &self.map[key] {
            attributed_value.set_value(value, receiver)
        }
        else {
            false
        }
    }

    pub fn delete(&mut self, key: &Key) -> bool {
        // TODO
        self.map.remove(key);
        true
    }

    pub fn own_property_keys(&self) -> Vec<Key> {
        self.map.keys().collect()
    }
}
