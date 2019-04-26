use std::collections::HashMap;

use super::super::property::PropertyDescriptor;
use super::JSFunction;
use super::JSString;
use super::JSSymbol;
use super::Value;

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Key {
    String(JSString),
    Symbol(JSSymbol),
}

pub enum Prototype<'a> {
    Null,
    Object(&'a JSObject<'a>),
}

pub struct JSObject<'a> {
    function: Option<JSFunction>,
    map: HashMap<Key, PropertyDescriptor<'a>>,
    prototype: Prototype<'a>,
}

impl<'a> JSObject<'a> {
    pub fn new_from_prototype(prototype: Prototype<'a>) -> Self {
        Self {
            function: None,
            map: HashMap::new(),
            prototype: prototype
        }
    }

    pub fn new_from_primitive(primitive: Value) -> Self {
        Self {
            function: None,
            map: HashMap::new(),
            prototype: Prototype::Null // TODO
        }
    }

    pub fn get_prototype_of(&self) -> &Prototype {
        &self.prototype
    }

    pub fn set_prototype_of(&mut self, prototype: Prototype<'a>) -> bool {
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

    pub fn get_own_property(&self, key: &Key) -> Option<&PropertyDescriptor> {
        self.map.get(key)
    }

    pub fn define_own_property(
        &mut self,
        key: Key,
        value: PropertyDescriptor<'a>
    ) -> bool {
        // TODO
        self.map.insert(key, value);
        true
    }

    pub fn has_property(&self, key: &Key) -> bool {
        self.map.contains_key(key)
    }

    pub fn get(&self, key: &Key, receiver: &mut JSObject) -> &Value {
        if let Some(attributed_value) = &self.map.get(key) {
            attributed_value.get_value(receiver)
        }
        else {
            &Value::Undefined
        }
    }

    pub fn set(
        &mut self,
        key: &Key,
        value: Value<'a>,
        receiver: &mut JSObject
    ) -> bool {
        // TODO
        if let Some(descriptor) = self.map.get_mut(key) {
            descriptor.set_value(value, receiver)
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

    pub fn own_property_keys(&self) -> Vec<&Key> {
        self.map.keys().collect()
    }
}
