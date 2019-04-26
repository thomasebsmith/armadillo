#![allow(dead_code)]
#![allow(unused_variables)]

mod runtime;

use std::error::Error;

use runtime::internals::PropertyDescriptor;
use runtime::internals::types::{
    JSChar,
    JSObject,
    JSObjectKey,
    JSString,
    SymbolFactory,
    Value,
};

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("Hello from Armadillo!");

    // Testing/getting rid of "unused ..." warnings
    let mut value = Value::Null;
    let mut receiver = JSObject::new_from_primitive(Value::Undefined);

    assert!(value.is_primitive());
    value = Value::Undefined;
    assert!(value.is_primitive());
    value = Value::Boolean(true);
    assert!(value.is_primitive());
    value = Value::Number(-35.3);
    assert!(value.is_primitive());
    let mut factory = SymbolFactory::new();
    value = Value::Symbol(factory.create_symbol());

    let mut object = JSObject::new_from_primitive(value);
    let key = JSObjectKey::Symbol(factory.create_symbol());
    object.define_own_property(
        key.clone(),
        PropertyDescriptor::new(Value::Number(58.999))
    );
    assert!(object.has_property(&key));
    assert!(match object.get(&key, &mut receiver) {
        Value::Number(n) => *n == 58.999,
        _ => false,
    });

    let keys = object.own_property_keys();
    assert!(*keys[0] == key);
    value = Value::Object(object);
    assert!(!value.is_primitive());

    value = Value::String(JSString::new(vec![JSChar::new(254)]));
    assert!(value.is_primitive());

    Ok(())
}
