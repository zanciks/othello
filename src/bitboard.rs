use std::fmt::{Display, Error, Formatter};

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
}

impl Display for BitBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
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