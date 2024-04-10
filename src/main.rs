mod bitboard;
mod move_gen;

fn main() {
    let mut friendlies = bitboard::BitBoard::default();
    friendlies.set_bit(51);

    let mut opposites = bitboard::BitBoard::default();
    opposites.set_bit(35);
    opposites.set_bit(19);
    opposites.set_bit(11);
    opposites.set_bit(26);
    opposites.set_bit(28);
    opposites.set_bit(29);
    opposites.set_bit(30);
    opposites.set_bit(36);
    opposites.set_bit(45);
    opposites.set_bit(20);
    opposites.set_bit(13);
    opposites.set_bit(18);
    opposites.set_bit(34);
    opposites.set_bit(41);

    println!("{}", move_gen::from_stone(27, friendlies, opposites));
}
