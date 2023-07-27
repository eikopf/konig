//! Sourced from https://www.openmymind.net/Regular-Expressions-in-Zig/
//!
//! As it stands, there are simply no regex libraries available
//! for Zig natively. For the meantime, this is the best solution.

const std = @import("std");
const re = @cImport({
    @cInclude("regez.h");
});

const expectEqual = std.testing.expectEqual;
const expect = std.testing.expect;

const REGEX_T_ALIGNOF = re.alignof_regex_t;
const REGEX_T_SIZEOF = re.sizeof_regex_t;
const REG_EXTENDED = re.REG_EXTENDED;

pub const isMatch = re.isMatch;

/// Creates and compiles a regex_t with the given pattern,
/// or fails and returns an error.
pub fn compileRegex(pattern: [:0]const u8, allocator: std.mem.Allocator) !*re.regex_t {
    var slice = try allocator.alignedAlloc(u8, REGEX_T_ALIGNOF, REGEX_T_SIZEOF);
    const regex: *re.regex_t = @ptrCast(slice.ptr);

    // 0 here implies no options
    if (re.regcomp(regex, pattern, REG_EXTENDED) != 0) {
        freeRegex(regex, allocator);
        return error.InvalidRegexPattern;
    }

    return regex;
}

/// Handles allocator destruction as well as cleaning
/// up the memory allocated by re.regcomp.
pub fn freeRegex(regex: *re.regex_t, allocator: std.mem.Allocator) void {
    re.regfree(regex);
    allocator.destroy(regex);
}

test "verify regez.h import" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    const regex = try compileRegex("[ab]c", allocator);
    defer freeRegex(regex, allocator);

    try expectEqual(true, re.isMatch(regex, "ac"));
    try expectEqual(false, re.isMatch(regex, "nope"));
}
