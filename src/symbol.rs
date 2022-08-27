use std::fmt::Display;

use rand::{distributions::Distribution, distributions::Standard};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SymbolType {
    Watermelon, // 🍉
    Coconut,    // 🥥
    Strawberry, // 🍓
    Tangerin,   // 🍊
    Lemon,      // 🍋
    Cherry,     // 🍒
    Kiwi,       // 🥝
}

impl Display for SymbolType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            SymbolType::Watermelon => write!(f, "🍉"),
            SymbolType::Coconut => write!(f, "🥥"),
            SymbolType::Strawberry => write!(f, "🍓"),
            SymbolType::Tangerin => write!(f, "🍊"),
            SymbolType::Lemon => write!(f, "🍋"),
            SymbolType::Cherry => write!(f, "🍒"),
            SymbolType::Kiwi => write!(f, "🥝"),
        }
    }
}

impl Distribution<SymbolType> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> SymbolType {
        match rng.gen_range(0..7) {
            0 => SymbolType::Watermelon,
            1 => SymbolType::Coconut,
            2 => SymbolType::Strawberry,
            3 => SymbolType::Tangerin,
            4 => SymbolType::Lemon,
            5 => SymbolType::Cherry,
            6 => SymbolType::Kiwi,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Symbol {
    pub r#type: SymbolType,
    pub multiplier: i32,
    pub viewed: bool,
}

impl Symbol {
    pub fn random() -> Self {
        let r#type = rand::random::<SymbolType>();
        let multiplier = if rand::random::<f64>() < 0.05 { 2 } else { 1 };
        let viewed = false;

        let symbol = Symbol {
            r#type,
            multiplier,
            viewed,
        };

        symbol
    }
}
