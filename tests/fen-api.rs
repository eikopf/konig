use konig::core::board::Board;
use konig::io::fen::*;
use konig::standard::index::StandardIndex;
use konig::standard::piece::StandardPiece;

#[test]
fn check_apis() {
    let index = StandardIndex::try_from(4u8).unwrap();
    let fen_board =
        FenData::try_from("rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2")
            .unwrap()
            .as_board();

    let white_king = fen_board.get_piece_at(index.into());

    assert_eq!(
        StandardPiece::WhiteKing,
        white_king.unwrap().to_owned().into()
    )
}
