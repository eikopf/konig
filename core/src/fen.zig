const std = @import("std");
const board = @import("board.zig");
const piece = @import("piece.zig");

const mem = std.mem; // for comparing bytes in strings
const parseInt = std.fmt.parseInt;
const charToDigit = std.fmt.charToDigit;
const expectEqual = std.testing.expectEqual;
const expectError = std.testing.expectError;

const fenStartingPosition = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const validCastlingPermissions = [16][]const u8{"-", "K", "Q", "k", "q", "KQ", "Kk", "Kq", "Qk", "Qq", "kq", "KQk", "KQq", "Kkq", "Qkq", "KQkq"};
const fileLetters = "abcdefgh";
const utf8DigitOffset = 48;

/// Stores the information parsed from a FEN string
/// as a 36-byte struct. Ordered as they appear in
/// a FEN string, the given fields are:
///
/// * the board position as a board.Board (32 bytes);
/// * the side to move as a piece.PieceColor (1 bit);
/// * the castling permissions as a u4 (4 bits), corresponding to an index into fen.validCastlingPermissions;
/// * the en passant target square as an i7 (7 bits), effectively a u6 board index with -1 as a sentinel "null" value;
/// * the halfmove clock as a u6 (6 bits);
/// * the fullmove counter as a u14 (14 bits);
const FenData = packed struct {
    board: board.Board,                     // 256 bits (32 bytes)
    sideToMove: piece.PieceColor,           // 1 bit
    castlingPermissions: u4,                // 4 bits
    enPassantTargetSquare: i7 = -1,         // 7 bits (could be compressed into a ?u3/i4)
    halfmoveClock: u6 = 0,                  // 6 bits
    fullmoveCounter: u14 = 1,               // 14 bits (sum 288 bits, 36 bytes)
    // fullmoveCounter needs to store at most ~9000 (u14 minimum)
};

/// Parses a complete FEN string into its components, and
/// returns the appropriate data in a 36-byte FenData struct.
pub fn parseFenString(str: []const u8) !FenData {
    var fieldIterator = mem.splitScalar(u8, str, ' ');

    return FenData {
        .board = try parsePiecePlacement(fieldIterator.next() orelse return error.InvalidFenStringComponent),
        .sideToMove = try parseSideToMove(fieldIterator.next() orelse return error.InvalidFenStringComponent),
        .castlingPermissions = try parseCastlingPermissions(fieldIterator.next() orelse return error.InvalidFenStringComponent),
        .enPassantTargetSquare = try parseEnPassantTargetSquare(fieldIterator.next() orelse return error.InvalidFenStringComponent),
        .halfmoveClock = try parseHalfmoveClock(fieldIterator.next() orelse return error.InvalidFenStringComponent),
        .fullmoveCounter = try parseFullmoveCounter(fieldIterator.next() orelse return error.InvalidFenStringComponent),
    };
}

/// Parses the "Piece placement" (1st) component of a FEN string.
fn parsePiecePlacement(str: []const u8) !board.Board {
    var index: u6 = 56; // fenIndexToBoardIndex(0);
    var layout: u256 = 0;

    for (str) |byte| {
        switch (byte) {
            'p', 'P', 'r',
            'R', 'b', 'B',
            'q', 'Q', 'k',
            'K', 'n', 'N' => |pieceByte| {
                const parsedPiece = try charToPiece(pieceByte);
                const boardIndex: u8 = @intCast(u8, 63 - fenIndexToBoardIndex(index));

                layout |= @as(u256, @intFromEnum(parsedPiece)) << (4 * boardIndex);
                if (index != 63 and (index + 1) % 8 != 0) index += 1;
            },

            '/' => index -= 15,
            '1'...'8' => |fillSpace| index += @intCast(u6, fillSpace - utf8DigitOffset - 1),
            else => return error.InvalidFenStringComponent,
        }
    }

    return board.Board { .layout = layout };
}

/// Parses the "Side to move" (2nd) component of a FEN string.
fn parseSideToMove(str: []const u8) !piece.PieceColor {
    if (!mem.eql(u8, str, "w") and !mem.eql(u8, str, "b")) return error.InvalidFenStringComponent;

    return switch (mem.eql(u8, str, "w")) {
        true => piece.PieceColor.WHITE,
        false => piece.PieceColor.BLACK,
    };
}

/// Parses the "Castling permissions" (3rd) component of a FEN string.
/// The resulting u4 represents the boolean values of each of the castling permissions,
/// corresponding to an index into validCastlingPermissions and to the particular values
/// of the "*CastleAvailable" fields in FenData.
fn parseCastlingPermissions(str: []const u8) !u4 {
    inline for (validCastlingPermissions, 0..) |perm, i| {
        if (mem.eql(u8, str, perm)) return @truncate(u4, i);
    }

    return error.InvalidFenStringComponent;
}

/// Parses the "En passant target square" (4th) component of a FEN string.
/// The resulting i7 is an index into the board in the range [0, 63].
/// The character '-' is parsed as the sentinel -1.
fn parseEnPassantTargetSquare(str: []const u8) !i7 {
    if (mem.eql(u8, "-", str)) return -1;
    if (str.len != 2) return error.InvalidFenStringComponent;
    if (str[1] != '3' and str[1] != '6') return error.InvalidFenStringComponent;

    inline for (fileLetters, 0..) |file, i| {
        if (str[0] == file) return switch (str[1] == '3') {
            true => @intCast(i7, @truncate(u6, 24 + i)),
            false => @intCast(i7, @truncate(u6, 48 + i)),
        };
    }

    return error.InvalidFenStringComponent;
}

/// Parses the "Halfmove clock" (5th) component of a FEN string.
/// The resulting u6 is guaranteed to be in the inclusive range [0, 50].
fn parseHalfmoveClock(str: []const u8) !u6 {
    const clockValue = try parseInt(u6, str, 10);

    if (clockValue < 0 or clockValue > 50) return error.InvalidFenStringComponent;
    return clockValue;
}

/// Parses the "Fullmove counter" (6th) component of a FEN string.
/// The resulting u14 is can hold a maximum value well above the
/// theoretical maximum game length.
fn parseFullmoveCounter(str: []const u8) !u14 {
    const counterValue = try parseInt(u14, str, 10);

    if (counterValue < 1) return error.InvalidFenStringComponent;
    return counterValue;
}

/// Converts the given FEN character to the appropriate piece where possible.
fn charToPiece(char: u8) !piece.Piece {
    return switch (char) {
        inline 48...56 => piece.Piece.NONE, // char values 1-8
        'p' => piece.Piece.BLACK_PAWN,
        'r' => piece.Piece.BLACK_ROOK,
        'n' => piece.Piece.BLACK_KNIGHT,
        'b' => piece.Piece.BLACK_BISHOP,
        'q' => piece.Piece.BLACK_QUEEN,
        'k' => piece.Piece.BLACK_KING,
        'P' => piece.Piece.WHITE_PAWN,
        'R' => piece.Piece.WHITE_ROOK,
        'N' => piece.Piece.WHITE_KNIGHT,
        'B' => piece.Piece.WHITE_BISHOP,
        'Q' => piece.Piece.WHITE_QUEEN,
        'K' => piece.Piece.WHITE_KING,
        else => error.InvalidFenStringComponent,
    };
}

/// Converts the index of a position on a FEN string
/// board representation into an index in the engine's
/// internal representation.
///
/// This function corresponds to a rank-wise reflection
/// about the center of the board, and therefore it is
/// its own inverse.
fn fenIndexToBoardIndex(index: u6) u6 {
    return switch (index) {
        56...63 => |i| i - 56,
        48...55 => |i| i - 40,
        40...47 => |i| i - 24,
        32...39 => |i| i - 8,
        24...31 => |i| i + 8,
        16...23 => |i| i + 24,
        8...15  => |i| i + 40,
        0...7 =>   |i| i + 56,
    };
}

/// Converts the index of a position in the internal
/// engine representation to an index in a FEN string.
///
/// This function corresponds to a rank-wise reflection
/// about the center of the board, and therefore is its
/// own inverse. Hence, this function is just an alias
/// for fenIndexToBoardIndex.
const boardIndexToFenIndex = fenIndexToBoardIndex;

test "parsing initial gamestate into FenData" {
    const initialState: FenData = try parseFenString(fenStartingPosition);
    try expectEqual(board.defaultPieceLayout, initialState.board.layout); // this is the only failing case
    try expectEqual(piece.PieceColor.WHITE, initialState.sideToMove);
    try expectEqual(@as(u4, 0b1111), initialState.castlingPermissions);
    try expectEqual(@as(i7, -1), initialState.enPassantTargetSquare);
    try expectEqual(@as(u6, 0), initialState.halfmoveClock);
    try expectEqual(@as(u14, 1), initialState.fullmoveCounter);
}

test "fen.fenIndexToBoardIndex" {
    var arr: [64]u6 = undefined;

    for (0..64) |i| {
        arr[i] = fenIndexToBoardIndex(@truncate(u6, i));
    }

    const fenIndices = [64]u6{56, 57, 58, 59, 60, 61, 62, 63,
                              48, 49, 50, 51, 52, 53, 54, 55,
                              40, 41, 42, 43, 44, 45, 46, 47,
                              32, 33, 34, 35, 36, 37, 38, 39,
                              24, 25, 26, 27, 28, 29, 30, 31,
                              16, 17, 18, 19, 20, 21, 22, 23,
                              8, 9, 10, 11, 12, 13, 14, 15,
                              0, 1, 2, 3, 4, 5, 6, 7 };

    try expectEqual(fenIndices, arr);
}

test "fen.fenIndexToBoardIndex is its own inverse" {
    for(0..64) |i| {
        try expectEqual(@intCast(u6, i), fenIndexToBoardIndex(fenIndexToBoardIndex(@intCast(u6, i))));
    }
}
