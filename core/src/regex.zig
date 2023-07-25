//! Sourced from https://www.openmymind.net/Regular-Expressions-in-Zig/
//!
//! As it stands, there are simply no regex libraries available
//! for Zig natively. For the meantime, this is the best solution.

const std = @import("std");
const re = @cImport({
    @cInclude("regez.h");
});

const REGEX_T_ALIGNOF = re.sizeof_regex_t;
const REGEX_T_SIZEOF = re.alignof_regex_t;

pub fn compileRegex(pattern: [:0]const u8, allocator: std.mem.Allocator) !*re.regex_t {
    var slice = try allocator.alignedAlloc(u8, REGEX_T_ALIGNOF, REGEX_T_SIZEOF);
    const regex: *re.regex_t = @ptrCast(slice.ptr);

    // 0 here implies no options
    if (re.regcomp(regex, pattern, 0) != 0) {
        freeRegex(regex);
        return error.InvalidRegexPattern;
    }

    return regex;
}

pub fn freeRegex(regex: *re.regex_t, allocator: std.mem.Allocator) void {
    re.regfree(regex);
    allocator.free(@as([*]u8, regex)[0..REGEX_T_SIZEOF]);
}

test "verify regez.h import" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    const regex = try compileRegex("[ab]c", allocator);
    defer freeRegex(regex, allocator);

    std.debug.print("{any}\n", .{re.isMatch(regex, "ac")});
    std.debug.print("{any}\n", .{re.isMatch(regex, "nope")});
}
