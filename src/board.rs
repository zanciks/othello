use std::fmt::{Display, Formatter};
use crate::bitboard::{BitBoard, EMPTY_BB};
use crate::color::Color;
use crate::move_gen::get_legal_moves;

#[derive(Clone)]
pub struct Board {
    stones: [BitBoard; 2],
    turn: Color
}

impl Default for Board {
    fn default() -> Self {
        let mut black = BitBoard::default();
        let mut white = BitBoard::default();

        black.set_bit(27);
        black.set_bit(36);

        white.set_bit(35);
        white.set_bit(28);

        Board {
            stones: [black, white],
            turn: Color::Black
        }
    }
}

impl Board {
    pub fn color_at(&self, index: u8) -> Option<Color> {
        if self.stones[0].get_bit(index) == 1 {return Some(Color::Black)}
        else if self.stones[1].get_bit(index) == 1 {return Some(Color::White)}
        else {return None}
    }
    pub fn get_friendlies(&self, color: Color) -> BitBoard {
        match color {
            Color::Black => self.stones[0],
            Color::White => self.stones[1]
        }
    }
    pub fn get_opposites(&self, color: Color) -> BitBoard {
        match color {
            Color::Black => self.stones[1],
            Color::White => self.stones[0]
        }
    }
    pub fn get_legal_moves(&self) -> Vec<u8> {
        let mut output: Vec<u8> = vec![];

        let friendlies = self.get_friendlies(self.turn);
        let opposites = self.get_opposites(self.turn);

        for i in friendlies.get_all_set() {
            let legals = get_legal_moves(i, friendlies, opposites);
            output.append(&mut legals.get_all_set());
        }

        return output;
    }
    pub fn side_to_move(&self) -> Color {
        self.turn
    }
    pub fn flip_stone(&mut self, stone: u8) {
        let color = self.color_at(stone);
        match color {
            None => {
                match self.side_to_move() {
                    Color::Black => self.stones[0].set_bit(stone),
                    Color::White => self.stones[1].set_bit(stone)
                }
            },
            Some(Color::Black) => {
                self.stones[0].set_bit(stone);
                self.stones[1].reset_bit(stone);
            },
            Some(Color::White) => {
                self.stones[0].reset_bit(stone);
                self.stones[1].set_bit(stone);
            }
        }
    } 
    pub fn flip_all_stones(&mut self, stones: &Vec<u8>) {
        for stone in stones {
            self.flip_stone(*stone)
        }
    }
    pub fn set_side_to_move(&mut self, color: Color) {
        self.turn = color;
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut output: String = "".to_owned();
        for rank in (0..8).rev() {
            for file in 0..8 {
                let index = file + (8 * rank);
                output.push(match self.color_at(index) {
                    Some(Color::Black) => 'X',
                    Some(Color::White) => 'O',
                    None => '.'
                });
                output.push(' ');
            }
            output.push('\n');
        }

        write!(f, "{}", output)
    }
}