#![allow(dead_code)]

use std::fmt;

pub struct BitBoard(u64);


impl Default for BitBoard {
    fn default() -> Self {
        BitBoard(0)
    }
}

impl BitBoard {
    pub fn new(value: u64) -> Self {
        BitBoard(value)
    }
    pub fn value(&self) -> u64 {
        self.0
    }
    pub fn get_bit(&self, index: u8) -> u8 {
        ((self.value() >> index) & 1) as u8
    }
}

impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output: String = String::new();
        output.push_str(&format!("{} \n", self.value()));
        output.push_str(&format!("{:#64b} \n", self.value()));

        for rank in 0..8 {
            for file in 0..8 {
                let index = file + (8 * rank);
                output.push(match self.get_bit(index) {
                    0 => '.',
                    1 => 'X',
                    _ => '?'
                });
                output.push(' ');
            }
            output.push('\n');
        }

        write!(f, "{}", output)
    }
}