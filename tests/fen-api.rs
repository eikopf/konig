use konig::core::board::Board;
use konig::io::fen::Fen;
use konig::standard::index::StandardIndex;
use konig::standard::piece::StandardPiece;

#[test]
fn check_apis() {
    let index = StandardIndex::new(4);
    let fen_board = Fen::default().into_board();
    let white_king = fen_board.get_piece_at(index.into());

    assert_eq!(
        StandardPiece::WhiteKing,
        white_king.unwrap().to_owned().into()
    );
}
