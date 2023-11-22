use konig::core::Position;
use konig::io::Fen;
use konig::standard::piece::StandardPiece;
use konig::standard::Square;

#[test]
fn get_piece_from_into_board() {
    let index = Square::try_from("e1").unwrap(); // should be 4
    let fen_board = Fen::default().into_board();
    let white_king = fen_board.get_piece_at(index.into());

    assert_eq!(
        StandardPiece::WhiteKing,
        white_king.unwrap().to_owned().into()
    );
}

#[test]
fn get_piece_from_standard_board() {
    let index = Square::try_from("h1").unwrap();
    let std_board = Fen::default().to_standard_board();

    assert_eq!(std_board[index], Some(StandardPiece::WhiteRook));
}

#[test]
fn index_fen_board_with_standard_index() {
    let position = "rn1k1b1r/1pp1ppp1/8/pBp2b1p/4P3/1P5N/P1PB2PP/RN1Q1RK1 w Kkq - 3 6";
    let fen_board = Fen::try_from(position).unwrap().into_board();

    let index = Square::try_from("b5").unwrap(); // 33
    assert_eq!(fen_board[index], Some(StandardPiece::WhiteBishop));

    let index = Square::try_from("a5").unwrap(); // 32
    assert_eq!(fen_board[index], Some(StandardPiece::BlackPawn));

    let index = Square::try_from("b6").unwrap(); // 41
    assert_eq!(fen_board[index], None);

    let index = Square::try_from("h3").unwrap(); // 23
    assert_eq!(fen_board[index], Some(StandardPiece::WhiteKnight));
}
