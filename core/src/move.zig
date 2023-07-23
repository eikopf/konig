const std = @import("std");
const piece = @import("piece.zig");

const Piece = piece.Piece;
const assert = std.debug.assert;

const expectEqual = std.testing.expectEqual;
const expectError = std.testing.expectError;

/// Stores metadata associated with a given move. The *_PROMOTION values are defined
/// such that the lower three bits correspond to the particular PieceType that they
/// represent promotion to.
const MoveFlag = enum(u4) {
    NONE = 0b0000,
    EN_PASSANT_CAPTURE = 0b0001,
    CASTLE = 0b0010,
    PAWN_TWO_UP = 0b0011,
    // undefined values
    ROOK_PROMOTION = 0b1010,
    KNIGHT_PROMOTION = 0b1011,
    BISHOP_PROMOTION = 0b1100,
    QUEEN_PROMOTION = 0b1101,
    // undefined values,
    _,
};

/// A 2-byte struct representing a simple
/// move, with additional metadata.
pub const Move = packed struct {
    source: u6,
    target: u6,
    flag: MoveFlag = MoveFlag.NONE,
};

/// Describes the optional field
/// used to disambiguate potentially
/// ambiguous moves from one another
/// in SAN, according to section
/// 8.2.3.4 of the PGN spec.
const SanDisambiguationField = packed union {
    fileLetter: u8,
    rankDigit: u8,
    sourceSquare: u6,
    none: void,
};

/// Describes the optional traditional
/// suffix annotation used to describe
/// qualitative aspects of a move.
const SanSuffixAnnotation = enum {
    NONE,
    BANG,
    HOOK,
    BANG_BANG,
    BANG_HOOK,
    HOOK_BANG,
    HOOK_HOOK,
};

/// A 4-byte struct representing the
/// data communicated by a SAN symbol.
pub const SanMove = packed struct {
    target: u6,                     // 6 bits
    pieceType: piece.PieceType,     // 3 bits
    flag: MoveFlag = .NONE,         // 4 bits
    disambiguationField: SanDisambiguationField = .{ .none = {} }, // 8 bits
    isCheck: bool,
    isCheckMate: bool,
    suffix: SanSuffixAnnotation = .NONE,
};
