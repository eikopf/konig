const std = @import("std");
const board = @import("board.zig");
const Piece = board.Piece;

fn pieceToSymbol(piece: Piece) []const u8 {
    return switch(piece) {
        Piece.NONE => " ",

        Piece.BLACK_PAWN => "\u{265F}",
        Piece.BLACK_ROOK => "\u{265C}",
        Piece.BLACK_KNIGHT => "\u{265E}",
        Piece.BLACK_BISHOP => "\u{265D}",
        Piece.BLACK_QUEEN => "\u{265B}",
        Piece.BLACK_KING => "\u{265A}",

        Piece.WHITE_PAWN => "\u{2659}",
        Piece.WHITE_ROOK => "\u{2656}",
        Piece.WHITE_KNIGHT => "\u{2658}",
        Piece.WHITE_BISHOP => "\u{2657}",
        Piece.WHITE_QUEEN => "\u{2655}",
        Piece.WHITE_KING => "\u{2654}",
    };
}

pub fn printBoard(brd: u256) !void {

    var output = [_]u8{};

    var i: u8 = 0;
    while (i < 64) {
        const piece = try board.getPieceAtIndex(brd, i);
        output ++ pieceToSymbol(piece);
        if ((i + 1) % 8 == 0) output ++ "\n";
    }

    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();

    try stdout.print("{s}", .{output});

    try bw.flush(); // don't forget to flush!
}
