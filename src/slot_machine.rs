use std::fmt::Display;

use crate::{reel::Reel, symbol::Symbol};

#[derive(Debug, Clone, Copy)]
pub struct Position(usize, usize);

#[derive(Debug, Clone, Copy)]
pub struct WinningSymbol {
    pub symbol: Symbol,
    pub position: Position,
}

#[derive(Debug)]
pub struct SlotMachine {
    pub reels: [Reel; 7],
}

impl Display for SlotMachine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for reel in self.reels.iter() {
            for symbol in reel.symbols.iter() {
                write!(f, "{}", symbol.unwrap().r#type)?;
            }

            writeln!(f, "")?;
        }

        Ok(())
    }
}

impl SlotMachine {
    pub fn new() -> Self {
        let slot_machine = SlotMachine {
            reels: [
                Reel::random(),
                Reel::random(),
                Reel::random(),
                Reel::random(),
                Reel::random(),
                Reel::random(),
                Reel::random(),
            ],
        };

        slot_machine
    }

    pub fn spin(&mut self) {
        self.reels = [
            Reel::random(),
            Reel::random(),
            Reel::random(),
            Reel::random(),
            Reel::random(),
            Reel::random(),
            Reel::random(),
        ];
    }

    fn combination(
        &mut self,
        position: Position,
        previous_position: Position,
    ) -> Vec<WinningSymbol> {
        let vec = vec![];

        vec
    }

    pub fn combinations(&mut self) -> Vec<Vec<WinningSymbol>> {
        let mut winning_combinations = vec![];

        for (reel_index, reel) in self.reels.iter().enumerate() {
            for (symbol_index, symbol) in reel.symbols.iter().enumerate() {
                if symbol.is_some() && symbol.unwrap().viewed == false {
                    let position = Position(reel_index, symbol_index);
                    let previous_position = Position(reel_index, symbol_index);

                    let combination = self.combination(position, previous_position);

                    if 5 < combination.len() {
                        winning_combinations.push(combination);
                    }
                }
            }
        }

        winning_combinations
    }
}
