const std = @import("std");
pub const regex = @import("regex.zig");
const move = @import("move.zig");
const board = @import("board.zig");
const fen = @import("fen.zig");
const pgn = @import("pgn.zig");

test "all" {
    std.testing.refAllDecls(@This());
}
