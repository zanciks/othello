use crate::bitboard::*;

pub fn from_stone(square: u8, friendlies: BitBoard, opposites: BitBoard) -> BitBoard {
    let mut placeables: BitBoard = EMPTY_BB;
    let mut piece_board: BitBoard = EMPTY_BB;
    piece_board.set_bit(square);

    let rank = square / 8;
    let file= square % 8;

    // north movement
    if opposites.get_bit(file + (rank + 1) * 8) == 1 {
        for i in rank + 1..8 {
            let index = file + i * 8;
            match (opposites.get_bit(index), friendlies.get_bit(index)) {
                (0, 0) => {placeables.set_bit(index); break},
                (1, 0) => (),
                (0, 1) => break,
                _ => panic!("movegen::from_stone north movement match")
            }
        }
    }

    // south movement 
    if opposites.get_bit(file + (rank - 1) * 8) == 1 {
        for i in (0..rank).rev() {
            let index = file + i * 8;
            match (opposites.get_bit(index), friendlies.get_bit(index)) {
                (0, 0) => {placeables.set_bit(index); break},
                (1, 0) => (),
                (0, 1) => break,
                _ => panic!("movegen::from_stone south movement match")
            }
        }
    }

    // west movement 
    if opposites.get_bit((file - 1) + rank * 8) == 1 {
        for i in (0..file).rev() {
            let index = i + rank * 8;
            match (opposites.get_bit(index), friendlies.get_bit(index)) {
                (0, 0) => {placeables.set_bit(index); break},
                (1, 0) => (),
                (0, 1) => break,
                _ => panic!("movegen::from_stone south movement match")
            }
        }
    }

    // east movement 
    if opposites.get_bit((file + 1) + file * 8) == 1 {
        for i in file + 1..8 {
            let index = i + rank * 8;
            match (opposites.get_bit(index), friendlies.get_bit(index)) {
                (0, 0) => {placeables.set_bit(index); break},
                (1, 0) => (),
                (0, 1) => break,
                _ => panic!("movegen::from_stone south movement match")
            }
        }
    }

    // North-East movement
    if opposites.get_bit((file + 1) + (rank + 1) * 8) == 1 {
        for i in 1..8 {
            if (rank + i) >= 8 || (file + i) >= 8 {break}
            let index = (file + i) + (rank + i) * 8;
            match (opposites.get_bit(index), friendlies.get_bit(index)) {
                (0, 0) => {placeables.set_bit(index); break},
                (1, 0) => (),
                (0, 1) => break,
                _ => panic!("movegen::from_stone south movement match")
            }
        }
    }
    
    // South-East movement 
    if opposites.get_bit((file + 1) + (rank - 1) * 8) == 1 {
        for i in 1..8 {
            if i > rank || (file + i) >= 8 {break}
            let index = (file + i) + (rank - i) * 8;
            match (opposites.get_bit(index), friendlies.get_bit(index)) {
                (0, 0) => {placeables.set_bit(index); break},
                (1, 0) => (),
                (0, 1) => break,
                _ => panic!("movegen::from_stone south movement match")
            }
        }
    }

    // South-West movement
    if opposites.get_bit((file - 1) + (rank - 1) * 8) == 1 {
        for i in 1..8 {
            if i > rank || i > file {break}
            let index = (file - i) + (rank - i) * 8;
            match (opposites.get_bit(index), friendlies.get_bit(index)) {
                (0, 0) => {placeables.set_bit(index); break},
                (1, 0) => (),
                (0, 1) => break,
                _ => panic!("movegen::from_stone south movement match")
            }
        }
    }

    // North-West movement
    if opposites.get_bit((file - 1) + (rank + 1) * 8) == 1 {
        for i in 1..8 {
            if (rank + i) >= 8 || i > file {break}
            let index = (file - i) + (rank + i) * 8;
            match (opposites.get_bit(index), friendlies.get_bit(index)) {
                (0, 0) => {placeables.set_bit(index); break},
                (1, 0) => (),
                (0, 1) => break,
                _ => panic!("movegen::from_stone south movement match")
            }
        }
    }

    return placeables;
}