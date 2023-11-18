use konig::core::board::Board;
use konig::io::fen::Fen;
use konig::standard::index::StandardIndex;
use konig::standard::piece::StandardPiece;

#[test]
fn get_piece_from_into_board() {
    let index = StandardIndex::new(4);
    let fen_board = Fen::default().into_board();
    let white_king = fen_board.get_piece_at(index.into());

    assert_eq!(
        StandardPiece::WhiteKing,
        white_king.unwrap().to_owned().into()
    );
}

#[test]
fn get_piece_from_standard_board() {
    let index = StandardIndex::new(7);
    let std_board = Fen::default().to_standard_board();

    assert_eq!(std_board[index], Some(StandardPiece::WhiteRook));
}

#[test]
fn index_fen_board_with_standard_index() {
    let position = "rn1k1b1r/1pp1ppp1/8/pBp2b1p/4P3/1P5N/P1PB2PP/RN1Q1RK1 w Kkq - 3 6";
    let fen_board = Fen::try_from(position).unwrap().into_board();

    let index = StandardIndex::new(33);
    assert_eq!(fen_board[index], Some(StandardPiece::WhiteBishop));

    let index = StandardIndex::new(32);
    assert_eq!(fen_board[index], Some(StandardPiece::BlackPawn));

    let index = StandardIndex::new(41);
    assert_eq!(fen_board[index], None);

    let index = StandardIndex::new(23);
    assert_eq!(fen_board[index], Some(StandardPiece::WhiteKnight));
}
