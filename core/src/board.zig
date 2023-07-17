/// Provides essential functions and structs for handling board states.
///
/// A board state is represented as a u256, and has no notion of the wider game state (i.e. of turns).
/// Each board state can be thought of as a [64]u4, where each u4 encodes the color and type of piece at
/// the given index.
///
/// Pieces are indexed from 0 to 63 in a left-to-right bottom-to-top ordering.

const std = @import("std");
const Piece = @import("piece.zig").Piece;
const assert = std.debug.assert;

const expectEqual = std.testing.expectEqual;
const expectError = std.testing.expectError;

/// The initial chess game state as a sequence of 64 u4 vals, where each u4 is (color << 4) + piece
pub const defaultPieceLayout: u256 = 0x23456432_11111111_00000000_00000000_00000000_00000000_99999999_abcdecba;
pub const initialBoard: Board = Board { .layout = defaultPieceLayout };

/// Represents a chessboard state as a u256
pub const Board = packed struct {
    layout: u256, // effectively a packed [64]u4

    /// Indexes 4n bits into the board layout and returns the corresponding u4 piece code
    fn getCodeAtIndex(self: Board, n: u8) u4 {
        assert(n >= 0 and n < 64);  // TODO: should this be an error?

        return @truncate(u4, 0b1111 & (self.layout >> (4 * n)));
    }

    /// Indexes n places into the board layout and returns the piece at that location.
    pub fn getPieceAtIndex(self: Board, n: u8) !Piece {
        const code: u4 = self.getCodeAtIndex(n);

        switch (code) {
            inline 0x7, 0x8, 0xf => return error.InvalidPieceCode,
            else => return @enumFromInt(Piece, code),
        }
    }

    pub fn writePieceToIndex(self: *Board, piece: Piece, n: u8) void {
        // https://stackoverflow.com/a/27592777
        const code = @as(u256, @intFromEnum(piece));
        self.layout = (self.layout & (@as(u256, 0b1111) << (n * 4))) | (code << (n * 4));
    }
};

/// Converts an algebraic position like "a3" or "G7" into an index from 0 to 63
pub fn algebraicPositionToIndex(str: []const u8) !u8 {

    assert(str.len == 2);

    const major: u8 = switch(str[0]) {
        'a', 'A' => 0,
        'b', 'B' => 1,
        'c', 'C' => 2,
        'd', 'D' => 3,
        'e', 'E' => 4,
        'f', 'F' => 5,
        'g', 'G' => 6,
        'h', 'H' => 7,
        else => return error.AlgebraicNotationConversionError,
    };

    const minor: u8 = switch(str[1]) {
        '1' => 0,
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        else => return error.AlgebraicNotationConversionError,
    };

    return (major * 8) + minor;
}

// TODO: lookup table? (dedicated hash function)
// TODO: single-piece search
// TODO: write piece to index


test "board.Board.getCodeAtIndex" {
    try expectEqual(@intFromEnum(Piece.NONE), initialBoard.getCodeAtIndex(17));
    try expectEqual(@intFromEnum(Piece.WHITE_ROOK), initialBoard.getCodeAtIndex(0));
}

test "board.Board.getPieceAtIndex" {
    const piece0 = try initialBoard.getPieceAtIndex(0);
    const piece12 = try initialBoard.getPieceAtIndex(12);

    try expectEqual(Piece.WHITE_ROOK, piece0);
    try expectEqual(Piece.WHITE_PAWN, piece12);
}
