const std = @import("std");
const assert = std.debug.assert;
const expectEqual = std.testing.expectEqual;
const expectError = std.testing.expectError;

/// Represents a piece on a chessboard.
///
/// Values are allocated such that the most significant bit represents the color of the piece (1 for white, 0 for black), and such that the lower three bits represent the type of the piece.
pub const Piece = enum(u4) {
    NONE = 0x0,         // 0b0000
    BLACK_PAWN = 0x1,   // 0b0001
    BLACK_ROOK = 0x2,   // 0b0010
    BLACK_KNIGHT = 0x3, // 0b0011
    BLACK_BISHOP = 0x4, // 0b0100
    BLACK_QUEEN = 0x5,  // 0b0101
    BLACK_KING = 0x6,   // 0b0110
    // 0x7 is undefined
    // 0x8 is undefined
    WHITE_PAWN = 0x9,   // 0b1001
    WHITE_ROOK = 0xa,   // 0b1010
    WHITE_KNIGHT = 0xb, // 0b1011
    WHITE_BISHOP = 0xc, // 0b1100
    WHITE_QUEEN = 0xd,  // 0b1101
    WHITE_KING = 0xe,   // 0b1110
    // 0xf is undefined
    _,

    pub fn getType(self: Piece) PieceType {
        return @enumFromInt(PieceType, @truncate(u3, @intFromEnum(self)));
    }

    pub fn getColor(self: Piece) PieceColor {
        return @enumFromInt(bool, @truncate(bool, @intFromEnum(self) >> 3));
    }
};

pub const PieceType = enum(u3) {
    NONE = 0b000,
    PAWN = 0b001,
    ROOK = 0b010,
    KNIGHT = 0b011,
    BISHOP = 0b100,
    QUEEN = 0b101,
    KING = 0b110,
    // 0b111 is undefined
    _,
};

pub const PieceColor = enum(u1) {
    WHITE = 1,
    BLACK = 0,
};

test "piece.Piece.getType" {
    const black_king = Piece.BLACK_KING;
    const white_king = Piece.WHITE_KING;

    try expectEqual(PieceType.KING, black_king.getType());
    try expectEqual(PieceType.KING, white_king.getType());
}
