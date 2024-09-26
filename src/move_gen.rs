use crate::bitboard::BitBoard;

// movegen function will take in two bitboards
// we then iterate over all occupied squares on the friendly bitboard
// then for each square occupied, we check all lines (vertical, horizontal, diagonal)
// if the next square is on the enemy bitboard, continue
// if the next square is on the friendly bitboard, cancel that line
// if the next square is unoccupied, add it to the possible moves
// however, we have to be sure that one square to place on, we connect 
// lines to all squares with friendly pieces. we could either re-iterate (not good perf)
// or we could find a way to include that data when initially creating the moves list
/*
    Move {
        placement_square: u8,
        connection_squares: [u8] or Vec<u8>
    }
*/
// then we get all squares between those (just some maths and iteration)
// and flip those squares on both bitboards