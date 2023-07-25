const std = @import("std");
const piece = @import("piece.zig");
const board = @import("board.zig");

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

const SanDisambiguationFieldTag = enum {
    fileLetter,
    rankDigit,
    sourceSquare,
    none,
};

/// Describes the optional field
/// used to disambiguate potentially
/// ambiguous moves from one another
/// in SAN, according to section
/// 8.2.3.4 of the PGN spec.
const SanDisambiguationField = union(SanDisambiguationFieldTag) {
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

const SanMoveTag = enum {
    normal,
    kingSideCastle,
    queenSideCastle,
};

const SanMove = union(SanMoveTag) {
    normal: SanMoveData,
    kingSideCastle: void,
    queenSideCastle: void,
};

/// A struct representing the
/// data communicated by a SAN symbol.
const SanMoveData = struct {
    target: u6,                     // 6 bits
    pieceType: piece.PieceType,     // 3 bits
    promotionPieceType: piece.PieceType, // 3 bits
    disambiguationField: SanDisambiguationField = .{ .none = {} }, // 8 bits
    isCapture: bool,
    isCheck: bool,
    isCheckmate: bool,
    isPromotion: bool,
    suffix: SanSuffixAnnotation = .NONE,

};

pub fn parseSanLiteral(source: []const u8) !SanMove {
    // handle simple cases
    if (std.mem.eql(u8, source, "O-O")) return .{ .kingSideCastle = {} };
    if (std.mem.eql(u8, source, "O-O-O")) return .{ .queenSideCastle = {} };

    // TODO: sanitize input

    // otherwise init reasonable defaults
    var result = SanMoveData{
        .target = undefined,
        .pieceType = .PAWN,
        .promotionPieceType = .NONE,
        .disambiguationField = .{ .none = {} },
        .isCapture = false,
        .isCheck = false,
        .isCheckmate = false,
        .isPromotion = false,
        .suffix = .NONE,
    };

    // state
    var positions = [4]u8{0, 0, 0, 0};
    var writeIndex: u2 = 0;

    for (source) |byte| switch (byte) {
        'K' => result.pieceType = .KING,

        'Q' => switch (result.isPromotion) {
            true => result.promotionPieceType = .QUEEN,
            false => result.pieceType = .QUEEN,
        },

        'B' => switch (result.isPromotion) {
            true => result.promotionPieceType = .BISHOP,
            false => result.pieceType = .BISHOP,
        },

        'R' => switch (result.isPromotion) {
            true => result.promotionPieceType = .ROOK,
            false => result.pieceType = .ROOK,
        },

        'N' => switch (result.isPromotion) {
            true => result.promotionPieceType = .KNIGHT,
            false => result.pieceType = .KNIGHT,
        },

        'a'...'h' => |file| {
            if (writeIndex == 1) writeIndex = 2;
            positions[writeIndex] = file;
            writeIndex += 1;
        },

        '1'...'8' => |rank| {
            if (writeIndex == 0) writeIndex = 1;
            positions[writeIndex] = rank;
            writeIndex += 1;
        },

        'x' => result.isCapture = true,
        '=' => result.isPromotion = true,
        '+' => result.isCheck = true,
        '#' => result.isCheckmate = true,

        '!' => result.suffix = switch (result.suffix) {
            .BANG => .BANG_BANG,
            .HOOK => .BANG_HOOK,
            .NONE => .BANG,
            else => return error.InvalidSanLiteral,
        },

        '?' => result.suffix = switch (result.suffix) {
            .BANG => .HOOK_BANG,
            .HOOK => .HOOK_HOOK,
            .NONE => .HOOK,
            else => return error.InvalidSanLiteral,
        },

        else => return error.InvalidSanLiteral,
    };

    // handle dis. field and target
    if (positions[2] == 0 and positions[3] == 0) {                              // case with no disambiguation component
        result.target = try board.algebraicPositionToIndex(positions[0..2]);
    } else {                                                                    // case with disambiguation component
        result.target = try board.algebraicPositionToIndex(positions[2..]);

        if (positions[0] != 0 and positions[1] != 0) {
            result.disambiguationField = .{ .sourceSquare = try board.algebraicPositionToIndex(positions[0..2]) };
        } else if (positions[0] != 0) {
            result.disambiguationField = .{ .fileLetter = positions[0] };
        } else if (positions[1] != 0) {
            result.disambiguationField = .{ .rankDigit = positions[1] + '0' };
        }
    }


    return .{ .normal = result };
}

test "SAN literal parsing" {
    const result = try parseSanLiteral("Qxa5#??");
    try expectEqual(piece.PieceType.QUEEN, result.normal.pieceType);
    try expectEqual(true, result.normal.isCapture);
    try expectEqual(false, result.normal.isCheck);
    try expectEqual(true, result.normal.isCheckmate);
    try expectEqual(@as(u6, 4), result.normal.target);
    try expectEqual(SanSuffixAnnotation.HOOK_HOOK, result.normal.suffix);
}
