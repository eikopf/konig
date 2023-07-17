//! Provides the essential components and definitions for moves and move validation.
//!
//! This module explicitly *does not* provide utilities for move generation or evalution.

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

/// A 16 bit struct representing a move, with additional metadata.
pub const Move = packed struct {
    source: u6,
    target: u6,
    flag: MoveFlag = MoveFlag.NONE,
};
