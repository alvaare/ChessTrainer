use chess_trainer::*;

fn main() {
    // let game = Game::from_pgn("[Event \"?\"]
    // [Site \"?\"]
    // [Date \"????.??.??\"]
    // [Round \"?\"]
    // [White \"?\"]
    // [Black \"?\"]
    // [Result \"*\"]
    // [WhiteElo \"2666\"]
    // [BlackElo \"2804\"]
    // [ECO \"B43b\"]
    
    // 1.e4 c5 2.Nf3 e6 3.d4 cxd4 4.Nxd4 a6 5.Nc3 d6 6.g4 Ne7 7.a3 Nbc6 8.Nb3 b5 
    // 9.h4 Bb7 10.Bf4 Ne5 11.Bxe5 dxe5 12.Qxd8+ Rxd8 13.Nc5 Rb8 14.Nxb7 Nc6 15.
    // a4 b4 16.Bxa6 bxc3 17.Bb5 Kd7 18.Na5 cxb2 19.Rd1+ Kc7 20.Nxc6 *");

    // let opening_file = "myOpening.txt";
    // let mut my_opening = OpeningTree::load(opening_file);
    // my_opening.add_d4();
    // my_opening.save(opening_file);
    let board = Board::default();
    let av_moves = board.available_moves();
    println!("{:?}", av_moves);
    println!("{:?}", av_moves.len());
}
