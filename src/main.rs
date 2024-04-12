mod bitboard;
mod move_gen;
mod board;
mod color;

fn main() {

    let mut board = board::Board::default();
    println!("{}", board);
    println!("{:?}", board.get_legal_moves())
}
