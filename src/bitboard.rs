use std::fmt::{Display, Formatter};

#[derive(Copy, Clone)]
pub struct BitBoard(u64);

impl Default for BitBoard {
    fn default() -> BitBoard {
        BitBoard(0)
    }
}

impl BitBoard {
    pub fn new(value: u64) -> BitBoard {
        BitBoard(value)
    }
    pub fn value(&self) -> u64 {
        self.0
    }
    pub fn get_bit(&self, index: u8) -> u8 {
        ((self.value() >> index) & 1) as u8
    }
    pub fn set_bit(&mut self, index: u8) {
        self.0 = self.value() | (1 << index)
    }
    pub fn reset_bit(&mut self, index: u8) {
        self.0 = self.value() & !(1 << index);
    }
    pub fn get_all_set(&self) -> Vec<u8> {
        let mut result = Vec::new();
        let mut mask: u64 = 1;
        for index in 0..64 {
            if self.0 & mask != 0 {
                result.push(index as u8);
            }
            mask <<= 1;
        }
        result
    }
    pub fn set_all(&mut self, indices: Vec<u8>) {
        for i in indices {
            self.set_bit(i);
        }
    }
    pub fn reset_all(&mut self, indices: Vec<u8>) {
        for i in indices {
            self.reset_bit(i);
        }
    }
}

impl Display for BitBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut output: String = "".to_owned();
        for rank in (0..8).rev() {
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

pub const EMPTY_BB: BitBoard = BitBoard(0);