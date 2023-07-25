const std = @import("std");
const Piece = @import("piece.zig").Piece;
const assert = std.debug.assert;

const expectEqual = std.testing.expectEqual;
const expectError = std.testing.expectError;

/// The initial chess game state as a sequence of 64 u4 vals, where each u4 is (color << 4) + piece
pub const defaultPieceLayout: u256 = 0x23465432_11111111_00000000_00000000_00000000_00000000_99999999_abcedcba;
pub const initialBoard: Board = Board { .layout = defaultPieceLayout };
const utf8DigitOffset = 48;

/// A general interface for iterators
/// over the elements of a board.
const LayoutIterator = union(enum) {
    ForwardLayoutIterator,
    ReverseLayoutIterator,
    FenOrderedLayoutIterator,

    pub fn next(self: *LayoutIterator) ?u4 {
       return switch (self) {
           inline else => |iter| iter.next(),
        };
    }

    pub fn reset(self: *LayoutIterator) void {
        switch (self) {
            inline else => |iter| iter.reset(),
        }
    }
};

/// Provides an in-order iterator over the given board layout,
/// from 0 to 63 (252).
const ForwardLayoutIterator = struct {
    index: i16 = -4,
    layout: *const u256,

    pub fn next(self: *ForwardLayoutIterator) ?u4 {
        if (self.index == 252) return null;
        self.index += 4;
        return @intCast(0b1111 & (self.layout.* >> @intCast(self.index)));
    }

    pub fn reset(self: *ForwardLayoutIterator) void {
        self.index = -4;
    }
};

/// Provides a reverse-order iterator over the given board layout,
/// from 63 (252) to 0.
const ReverseLayoutIterator = struct {
    index: u16 = 256,
    layout: *const u256,

    pub fn next(self: *ReverseLayoutIterator) ?u4 {
        if (self.index == 0) return null;
        self.index -= 4;
        return @intCast(0b1111 & (self.layout.* >> @intCast(self.index)));
    }

    pub fn reset(self: *ReverseLayoutIterator) void {
        self.index = 256;
    }
};

/// Provides an iterator over the given board layout
/// matching the order in which FEN strings represent
/// a position (i.e. left-to-right, high-rank-to-low-rank).
const FenOrderedLayoutIterator = struct {
    index: u16 = 220, // board index 56
    rankIndex: u8 = 0,
    layout: *const u256,

    pub fn next(self: *FenOrderedLayoutIterator) ?u4 {
        if (self.index == 28) return null;

        self.index += 4;
        if (self.rankIndex == 8) {
            self.rankIndex = 1;
            self.index -= 64;
        } else {
            self.rankIndex += 1;
        }

        return @intCast(0b1111 & (self.layout.* >> @intCast(self.index)));
    }

    pub fn reset(self: *FenOrderedLayoutIterator) void {
        self.index = 220;
        self.rankIndex = 0;
        self.reachedEnd = false;
    }
};

/// Represents a chessboard state as a u256
pub const Board = packed struct {
    layout: u256, // effectively a packed [64]u4

    /// Indexes 4n bits into the board layout and returns the corresponding u4 piece code
    fn getCodeAtIndex(self: *const Board, n: u8) u4 {
        assert(n >= 0 and n < 64);  // TODO: should this be an error?

        return @truncate(0b1111 & (self.layout >> (4 * n)));
    }

    /// Indexes n places into the board layout and returns the piece at that location.
    pub fn getPieceAtIndex(self: *const Board, n: u8) !Piece {
        const code: u4 = self.getCodeAtIndex(n);

        switch (code) {
            inline 0x7, 0x8, 0xf => return error.InvalidPieceCode,
            else => return @enumFromInt(code),
        }
    }

    /// Writes a piece to the nth index in the board.
    pub fn writePieceToIndex(self: *Board, piece: Piece, n: u8) void {
        // https://stackoverflow.com/a/27592777
        const code = @as(u256, @intFromEnum(piece));
        self.layout = (self.layout & (@as(u256, 0b1111) << (n * 4))) | (code << (n * 4));
    }

    /// Returns a new LayoutIterator over the board's layout.
    pub fn forwardLayoutIterator(self: *const Board) ForwardLayoutIterator {
        return ForwardLayoutIterator{ .layout = &self.layout};
    }

    /// Returns a new ReverseLayoutIterator over the board's layout.
    pub fn reverseLayoutIterator(self: *const Board) ReverseLayoutIterator {
        return ReverseLayoutIterator{ .layout = &self.layout };
    }

    /// Returns a new FenOrderedLayoutIterator over the board's layout.
    pub fn fenOrderedLayoutIterator(self: *const Board) FenOrderedLayoutIterator {
        return FenOrderedLayoutIterator{ .layout = &self.layout };
    }
};

/// Converts an algebraic position like "a3" or "G7" into an index from 0 to 63
pub fn algebraicPositionToIndex(str: []const u8) !u6 {

    assert(str.len == 2);

    const rank: u8 = switch(str[0]) {
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

    const file: u8 = switch(str[1]) {
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

    return @intCast((rank * 8) + file);
}

pub fn indexToAlgebraicPosition(index: u6) []const u8 {
    const rank: u8 = switch (@divFloor(index, 8)) {
        0 => 'a',
        1 => 'b',
        2 => 'c',
        3 => 'd',
        4 => 'e',
        5 => 'f',
        6 => 'g',
        7 => 'h',
        else => unreachable,
    };

    const file: u8 = (index % 8) + utf8DigitOffset;

    return &[2]u8{rank, file};
}


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

test "board.Board.forwardLayoutIterator on initialState" {
    var fli = initialBoard.forwardLayoutIterator();

    for (0..13) |_| {
        _ = fli.next();
    }

    try expectEqual(@as(?u4, 0b1001), fli.next()); // WHITE_PAWN
    try expectEqual(@as(?u4, 0b1001), fli.next()); // WHITE_PAWN
}

test "board.Board.reverseLayoutIterator on initialState" {
    var rli = initialBoard.reverseLayoutIterator();

    try expectEqual(@as(?u4, 0b0010), rli.next());  // BLACK_ROOK
    try expectEqual(@as(?u4, 0b0011), rli.next());  // BLACK_KNIGHT
    try expectEqual(@as(?u4, 0b0100), rli.next());  // BLACK_BISHOP
    try expectEqual(@as(?u4, 0b0110), rli.next());  // BLACK_KING

    for (0..20) |_| {
        _ = rli.next();
    }

    try expectEqual(@as(?u4, 0b0000), rli.next());  // NONE
}

test "board.Board.fenOrderedLayoutIterator on initialState" {
    var foli = initialBoard.fenOrderedLayoutIterator();

    try expectEqual(@as(?u4, 0b0010), foli.next());  // BLACK_ROOK
    try expectEqual(@as(?u4, 0b0011), foli.next());  // BLACK_KNIGHT
    try expectEqual(@as(?u4, 0b0100), foli.next());  // BLACK_BISHOP
    try expectEqual(@as(?u4, 0b0101), foli.next());  // BLACK_QUEEN
}
