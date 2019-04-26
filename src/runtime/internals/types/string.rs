#[derive(PartialEq, Eq, Hash, Clone)]
pub struct JSChar(u16);

impl JSChar {
    pub fn new(character: u16) -> Self {
        Self(character)
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct JSString(Vec<JSChar>);

impl JSString {
    pub fn new(characters: Vec<JSChar>) -> Self {
        Self(characters)
    }
}
