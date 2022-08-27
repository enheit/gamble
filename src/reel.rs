use crate::symbol::Symbol;

#[derive(Debug)]
pub struct Reel {
    pub symbols: [Option<Symbol>; 7],
}

impl Reel {
    pub fn random() -> Self {
        let reel = Reel {
            symbols: [
                Some(Symbol::random()),
                Some(Symbol::random()),
                Some(Symbol::random()),
                Some(Symbol::random()),
                Some(Symbol::random()),
                Some(Symbol::random()),
                Some(Symbol::random()),
            ],
        };

        reel
    }

    pub fn clear_position(&mut self, index: usize) {
        if index < self.symbols.len() {
            self.symbols[index] = None;
        }
    }

    pub fn squash(&mut self) {
        self.symbols.sort_by_key(|a| a.is_some());
    }

    pub fn refill(&mut self) {
        for symbol in &mut self.symbols {
            if symbol.is_none() {
                *symbol = Some(Symbol::random());
            }
        }
    }
}
