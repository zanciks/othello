use crate::{bitboard::*, board::Board};

pub fn get_legal_moves(square: u8, friendlies: BitBoard, opposites: BitBoard) -> BitBoard {
    let mut placeables: BitBoard = EMPTY_BB;

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
                _ => panic!("movegen::from_stone west movement match")
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
                _ => panic!("movegen::from_stone east movement match")
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
                _ => panic!("movegen::from_stone northeast movement match")
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
                _ => panic!("movegen::from_stone southeast movement match")
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
                _ => panic!("movegen::from_stone southwest movement match")
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
                _ => panic!("movegen::from_stone northwest movement match")
            }
        }
    }

    return placeables;
}

pub fn make_move_new(square: u8, board: Board) -> Board {
    let mut output_board = board.clone();

    let friendlies = board.get_friendlies(board.side_to_move());
    let opposites = board.get_opposites(board.side_to_move());

    output_board.flip_stone(square);
    let mut temp: Vec<u8>;

    let rank = square / 8;
    let file= square % 8;

    // north
    temp = vec![];
    for i in rank + 1..8 {
        let index = file + i * 8;
        match (opposites.get_bit(index), friendlies.get_bit(index)) {
            (0, 0) => break,
            (1, 0) => temp.push(index),
            (0, 1) => output_board.flip_all_stones(&temp),
            _ => panic!("movegen::from_stone north movement match")
        }
    }

    // south movement 
    for i in (0..rank).rev() {
        let index = file + i * 8;
        match (opposites.get_bit(index), friendlies.get_bit(index)) {
            (0, 0) => break,
            (1, 0) => temp.push(index),
            (0, 1) => output_board.flip_all_stones(&temp),
            _ => panic!("movegen::from_stone north movement match")
        }
    }
    
    // west movement 
    for i in (0..file).rev() {
        let index = i + rank * 8;
        match (opposites.get_bit(index), friendlies.get_bit(index)) {
            (0, 0) => break,
            (1, 0) => temp.push(index),
            (0, 1) => output_board.flip_all_stones(&temp),
            _ => panic!("movegen::from_stone north movement match")
        }
    }
    
    // east movement 
    for i in file + 1..8 {
        let index = i + rank * 8;
        match (opposites.get_bit(index), friendlies.get_bit(index)) {
            (0, 0) => break,
            (1, 0) => temp.push(index),
            (0, 1) => output_board.flip_all_stones(&temp),
            _ => panic!("movegen::from_stone north movement match")
        }
    }

    // North-East movement
    for i in 1..8 {
        if (rank + i) >= 8 || (file + i) >= 8 {break}
        let index = (file + i) + (rank + i) * 8;
        match (opposites.get_bit(index), friendlies.get_bit(index)) {
            (0, 0) => break,
            (1, 0) => temp.push(index),
            (0, 1) => output_board.flip_all_stones(&temp),
            _ => panic!("movegen::from_stone north movement match")
        }
    }
        
    // South-East movement 
    for i in 1..8 {
        if i > rank || (file + i) >= 8 {break}
        let index = (file + i) + (rank - i) * 8;
        match (opposites.get_bit(index), friendlies.get_bit(index)) {
            (0, 0) => break,
            (1, 0) => temp.push(index),
            (0, 1) => output_board.flip_all_stones(&temp),
            _ => panic!("movegen::from_stone north movement match")
        }
    }
    
    // South-West movement
    for i in 1..8 {
        if i > rank || i > file {break}
        let index = (file - i) + (rank - i) * 8;
        match (opposites.get_bit(index), friendlies.get_bit(index)) {
            (0, 0) => break,
            (1, 0) => temp.push(index),
            (0, 1) => output_board.flip_all_stones(&temp),
            _ => panic!("movegen::from_stone north movement match")
        }
    }
    
    // North-West movement
    for i in 1..8 {
        if (rank + i) >= 8 || i > file {break}
        let index = (file - i) + (rank + i) * 8;
        match (opposites.get_bit(index), friendlies.get_bit(index)) {
            (0, 0) => break,
            (1, 0) => temp.push(index),
            (0, 1) => output_board.flip_all_stones(&temp),
            _ => panic!("movegen::from_stone north movement match")
        }
    }

    output_board.set_side_to_move(!output_board.side_to_move());

    output_board
}