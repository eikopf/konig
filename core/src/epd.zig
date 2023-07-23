//! Refer to section 16.2 of http://www.saremba.de/chessgml/standards/pgn/pgn-complete.htm
//!
//! EPD (Extended Position Description) is a format for describing a given chess position,
//! in a similar manner as FEN; it is distinguished by the usage of distinct operations
//! and opcodes. Files containing only EPD records should have a file extension of `.epd`.
//!
//! An EPD string or record consists of a single line, broken into four mandatory
//! space-separated fields and zero or more additional "operations." The mandatory
//! fields (in order) are:
//!
//! 1. Piece Placement;
//! 2. Side To Move;
//! 3. Castling Ability;
//! 4. En Passant Target Square.
//!
//! These mandatory fields are identical in position, order, and format as their
//! FEN equivalents. The last two FEN fields (the halfmove clock and fullmove
//! counter) are instead replaced by the optional opcodes `hmvc` and `fmvn`.
//!
//! EPD operations are defined as a singular "opcode," followed by zero or
//! more operands and terminated by a semicolon.
//!
//! Opcodes must begin with a letter character, be composed of only alphanumeric
//! characters and the underscore, and be at most 15 characters in length. Where
//! an opcode has no operands, it is immediately suffixed with the semicolon. Some
//! opcodes expect particular formats and orderings of their parameters.
//!
//! Operands are either sequences of printing characters delimited by whitespace (symbols),
//! or strings delimited by double quotes. Strings may be at most 255 bytes. Symbols which
//! denote chess moves should be represented using SAN. Symbols representing numbers may
//! be signed or unsigned integers, or floating-point numbers, all of which may be subject
//! to range restrictions.

const std = @import("std");
const move = @import("move.zig");
const fen = @import("fen.zig");

/// A tag type used by the
/// Operation union, representing
/// the commonly used EPD opcodes.
const Opcode = enum {
    acn,        // analysis count: nodes (u64)
    acs,        // analysis count: seconds (u64)
    am,         // avoid move(s) ([]Move)
    bm,         // best move(s) ([]Move)
    c0,         // comment 0 (?[]const u8)
    c1,         // comment 1 (?[]const u8)
    c2,         // comment 2 (?[]const u8)
    c3,         // comment 3 (?[]const u8)
    c4,         //      ...
    c5,         //      ...
    c6,         //      ...
    c7,         //      ...
    c8,         //      ...
    c9,         //      ...
    ce,         // centipawn evaluation (i16)
    dm,         // direct mate fullmove count (u64)
    draw_accept,// accept a draw offer (void)
    draw_claim, // claim a draw (void)
    draw_offer, // offer a draw (void)
    draw_reject,// reject a draw offer (void)
    eco,        // Encylopedia of Chess Openings opening code (?[]const u8)
    fmvn,       // fullmove number (u16)
    hmvc,       // halfmove clock (u8)
    id,         // position identificaton ([]const u8)
    nic,        // New In Chess opening code (?[]const u8)
    noop,       // no operation (void)
    pm,         // predicted move (Move)
    pv,         // predicted variation (?[]Move)
    rc,         // repitition count (u8)
    resign,     // game resignation (void)
    sm,         // supplied move (Move)
    tcgs,       // telecommunication: game selector (u64)
    tcri,       // telecommunication: receiver identification ([2][]const u8)
    tcsi,       // telecommunication: sender identification ([2][]const u8)
    v0,         // variation name 0 (?[]const u8)
    v1,         // variation name 1 (?[]const u8)
    v2,         // variation name 2 (?[]const u8)
    v3,         // variation name 3 (?[]const u8)
    v4,         //      ...
    v5,         //      ...
    v6,         //      ...
    v7,         //      ...
    v8,         //      ...
    v9,         //      ...
};

const Operation = union(Opcode) {
    acn: u64,
    acs: u64,
    am: []move.Move,
    bm: []move.Move,
    c0: ?[]const u8,
    c1: ?[]const u8,
    c2: ?[]const u8,
    c3: ?[]const u8,
    c4: ?[]const u8,
    c5: ?[]const u8,
    c6: ?[]const u8,
    c7: ?[]const u8,
    c8: ?[]const u8,
    c9: ?[]const u8,
    ce: i16,
    dm: u64,
    draw_accept: void,
    draw_claim: void,
    draw_offer: void,
    draw_reject: void,
    eco: ?[]const u8,
    fmvn: u16,
    hmvc: u8,
    id: []const u8,
    nic: ?[]const u8,
    noop: void,
    pm: move.Move,
    pv: ?[]move.Move,
    rc: u8,
    resign: void,
    sm: move.Move,
    tcgs: u64,
    tcri: [2][]const u8,
    tcsi: [2][]const u8,
    v0: ?[]const u8,
    v1: ?[]const u8,
    v2: ?[]const u8,
    v3: ?[]const u8,
    v4: ?[]const u8,
    v5: ?[]const u8,
    v6: ?[]const u8,
    v7: ?[]const u8,
    v8: ?[]const u8,
    v9: ?[]const u8,
};
