pub struct SymbolFactory {
    next_number: u128,
}

impl SymbolFactory {
    pub fn new() -> Self {
        Self {
            next_number: 0
        }
    }
    pub fn create_symbol(&mut self) -> JSSymbol {
        self.next_number += 1;
        JSSymbol {
            raw_number: self.next_number - 1
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct JSSymbol {
    raw_number: u128,
}
