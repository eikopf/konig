// PGN spec. (http://www.saremba.de/chessgml/standards/pgn/pgn-complete.htm)
//
// The spec formally defines both import and export formats, with the
// general notion that the import format is a relaxed version of PGN,
// and that the output format is a strictly controlled pretty-printed
// version. (3.1 & 3.2)
//
// * Tokens (7)
// The tokens which constitute PGN data are defined
// as follows:
//
// - White Space (space, newline, tab)
// - String (delimited by double quotes)
//      - Double quotes are escaped with a backslash '\',
//      - Backslashes are escaped by another backslash (i.e. "\\"),
//      - Newline and tab are forbidden within strings,
//      - A string may be at most 255 characters.
// - Integer
//      - Technically a special case of a symbol,
//      - A sequence of one or more decimal digits,
//      - Terminated by the next non-symbol character.
// - Period (".")
// - Asterisk ("*")
// - Left Bracket ("[")
// - Right Bracket ("]")
// - Left Parenthesis ("(")
// - Right Parenthesis (")")
// - Left Angle Bracket ("<")
// - Right Angle Bracket (">")
// - Numeric Annotation Glyph
//      - Begins with a dollar sign "$" followed by a digit sequence,
//      - Terminated by the next non-digit character,
//      - In the range [0, 255] (i.e. it is a u8).
// - Symbol
//      - A symbol begins with a letter or digit character,
//      - A complete symbol is a sequence of the following characters:
//          - "A-Z",    (uppercase letters)
//          - "a-z",    (lowercase letters)
//          - "0-9",    (arabic numerals)
//          - "_",      (underscore)
//          - "+",      (plus)
//          - "#",      (octothorpe)
//          - "=",      (equals)
//          - ":",      (colon)
//          - "-".      (hyphen)
//      - A symbol is terminated by the next non-symbol character,
//      - A symbol may be at most 255 characters.
//
//
// * Tag Pairs (8.1)
// A tag pair is defined as a key-value pair of the
// form [<key> "<value>"]. Regardless of data type,
// all fields appear within double quotes.
//
// Tag names are case-sensitive and must begin with
// an upper-case letter.
//
// PGN import format allows for arbitrary white space
// throughout the tag pair (so long as the fields are preserved),
// but the export format explicitly limits it to the above-described
// form.
//
// Import format permits multiple tag pairs on the same
// line, as well as tag pairs spanning multiple lines. By
// contrast, export format requires each tag to be
// left-justified on a line by itself, with a single empty
// line following the last tag pair.
//
// The colon ":" is reserved as an item delimiter in
// multi-item values, and so should be otherwise prohibited.
//
// * Seven Tag Roster (8.1.1)
// PGN archival data is required to store seven tag pairs,
// which must appear before all other tag-pairs in the
// following order:
//
// 1. Event - A string describing the match or event;
// 2. Site - A string describing the location where the match took place;
// 3. Date - A yyyy.mm.dd date-string with ?? denoting an unknown date;
// 4. Round - The round ordinal of the given game within the event;
// 5. White - The name of the white player in <lastname>, <firstname> format;
// 6. Black - The name of the black player in <lastname>, <firstname> format;
// 7. Result - The score, recorded as <whitescore>-<blackscore>, or * for
//    an ongoing game.
//
// * Optional Tag Pairs (9)
// The standard allows for other optional tag pairs. Some common
// ones are Annotator, PlyCount, Mode, Termination, FEN, but the
// general idea is that there are a large variety of defined data
// fields.
//
// * File Structure (8, 11) [See formal grammar in section 18]
// A file containing a "PGN database" should use the .pgn suffix.
//
// A PGN database is a sequence of zero or more PGN games, hence
// an empty file is a valid PGN database.
//
// A PGN game is a tag pair section followed by a movetext section.
//
// (8.1)
// A tag pair section is composed of a series of zero or more tag pairs,
// as described previously. Duplication is not permitted.
//
// (8.1.1)
// The first seven fields in the tag pair section are obligatory,
// and have a well-defined order and format; this is called the
// Seven Tag Roster (STR).
//
// (8.2)
// The movetext section is composed of chess moves, move number indications,
// optional annotations, and a singular concluding game termination marker.
//
// (NOTE: Deviation From Standard) I make the decision here to permit illegal
// moves in the movetext section.
//
// (8.2.1)
// PGN export format is subject to additional requirements regarding line
// justification.
//
// (8.2.2)
// A move number indication is defined as an integer token, followed by zero
// or more periods. The integer portion of the indication denotes the move number
// of the immediately following white and black moves, if they are present. Export
// format is subject to additional requirements (8.2.2.2).
//
// (8.2.3)
// The moves in a movetext section are represented as SAN notation. In import
// format, a number of additional suffix annotations are available (8.2.3.8),
// which should be translated into the corresponding Numeric Annotation Glyph.
//
// * Comments (5)
// Two forms of comments are defined:
//
// 1. Semicolon comments begin with a semicolon
//    and continue to the end of the line;
// 2. Brace comments appear within curly braces.
//
// Brace comments are not permitted to nest, so a left
// brace is simply ignored. The first right brace encountered
// will terminate a brace comment. Nesting the two kinds
// of comments within one another has so special effect
//
// * Standard Algebraic Notation (SAN) (8.2.3)
// SAN is a representation standard for chess moves,
// stored as a sequence of ASCII characters. The general format
// (excluding special cases) is:
//
//          <piece><x?><target-square><=?><promotion-piece?><+?><#?><suffixes?>
//
// Within the context of PGN, SAN is formally just a symbol with special
// meaning; it should be parsed first into a symbol token, and then
// handled according to whether the preceding tokens imply that it
// should be an SAN token.
//
// * Miscellaneous Details
// - Tab characters are forbidden in the export format. (4.2)
// - All PGN formats may have a maximum line length of 255. (4.3)
// - An escape mechanism is provided for interleaving foreign commands
//   and data into a PGN stream, by beginning a new line with a percent
//   sign "%". (6)
// - A formal PGN file grammar is provided in section 18 of the spec.

const std = @import("std");
const move = @import("move.zig");
const fen = @import("fen.zig");

const mem = std.mem;
const Allocator = std.mem.Allocator;
const expect = std.testing.expect;
const expectEqual = std.testing.expectEqual;
const expectEqualDeep = std.testing.expectEqualDeep;
const expectError = std.testing.expectError;

/// A simple 3-component date
/// struct for storing parsed
/// dates from PGN data.
const Date = struct {
    year: ?u16,
    month: ?u8,
    day: ?u8,
};

/// A simple 3-component time
/// struct for storing parsed
/// time from PGN data.
const Time = struct {
    hour: u8,
    minute: u8,
    second: u8,
};

const PgnError = error {
    InvalidDataFormat,
    InvalidStringLiteral,
    InvalidNumericAnnotationGlyph,
};

/// A tag type used by the
/// Token union.
const TokenTag = enum {
    WHITESPACE,
    STRING,
    INTEGER,
    PERIOD,
    ASTERISK,
    LEFT_BRACKET,
    RIGHT_BRACKET,
    LEFT_PARENTHESIS,
    RIGHT_PARENTHESIS,
    LEFT_ANGLE_BRACKET,
    RIGHT_ANGLE_BRACKET,
    SYMBOL,
    NUMERIC_ANNOTATION_GLYPH,
};

/// A PGN token associated with the
/// appropriate data type.
const Token = union(TokenTag) {
    WHITESPACE: void,
    PERIOD: void,
    ASTERISK: void,
    LEFT_BRACKET: void,
    RIGHT_BRACKET: void,
    LEFT_PARENTHESIS: void,
    RIGHT_PARENTHESIS: void,
    LEFT_ANGLE_BRACKET: void,
    RIGHT_ANGLE_BRACKET: void,

    // data-carrying tokens
    STRING: []const u8,
    SYMBOL: []const u8,
    NUMERIC_ANNOTATION_GLYPH: u8,
    INTEGER: u32,

    pub fn debugPrint(self: *const Token) void {
        std.debug.print("\n{s}: {any}", .{@tagName(self.*), self});
    }
};

/// Provides an iterator over a []u8 source,
/// which repeatedly advances over it and
/// returns valid PGN tokens.
const TokenIterator = struct {
    allocator: *Allocator,
    source: []const u8,
    index: usize = 0,

    pub fn next(self: *TokenIterator) !?Token {
        defer self.index += 1;

        if (self.index == self.source.len) return null;

        return switch (self.source[self.index]) {
            // iterative cases
            '\n', ' ', 0x9, 0xE => self.parseWhitespace(),
            '"' => try self.parseString(self.allocator),
            'A'...'Z', 'a'...'z', '0'...'9' => |char| try self.parseSymbol(char, self.allocator),
            '%' => try self.parseNumericAnnotationGlyph(),

            // single-char cases
            '.' => Token{ .PERIOD = {} },
            '*' => Token{ .ASTERISK = {} },
            '[' => Token{ .LEFT_BRACKET = {} },
            ']' => Token{ .RIGHT_BRACKET = {} },
            '(' => Token{ .LEFT_PARENTHESIS = {} },
            ')' => Token{ .RIGHT_PARENTHESIS = {} },
            '<' => Token{ .LEFT_ANGLE_BRACKET = {} },
            '>' => Token{ .RIGHT_ANGLE_BRACKET = {} },

            else => error.InvalidDataFormat,
        };
    }

    pub fn reset(self: *TokenIterator) void {
        self.index = 0;
    }

    /// Iterates over a whitespace region in the
    /// source and emits a WHITESPACE token when
    /// complete.
    ///
    /// This function assumes that when it is
    /// invoked, source[index] is a whitespace
    /// character.
    fn parseWhitespace(self: *TokenIterator) Token {
        while (self.index + 1 != self.source.len) switch (self.source[self.index + 1]) {
            '\n', ' ', 0x9, 0xE => self.index += 1,
            else => return Token{ .WHITESPACE = {} },
        };

        // terminal whitespace:
        // permitted in import format
        return Token{ .WHITESPACE = {} };
    }

    /// Iterates over a string region in the
    /// source and emits a STRING token when
    /// complete.
    ///
    /// This function assumes that when it is
    /// invoked, source[index] == '"'.
    fn parseString(self: *TokenIterator, allocator: *Allocator) !Token {
        var value: [255]u8 = undefined;
        var writeIndex: u8 = 0;
        var escapeNextChar: bool = false;

        // on return, advance beyond the closing double quote
        defer self.index += 1;

        // we have already handled the opening '"',
        // so we just need to store the actual content
        // of the string literal.

        while (self.index + 1 != self.source.len) : (
            self.index += 1
        ) switch (self.source[self.index + 1]) {
            '"' => {
                if (escapeNextChar) {
                    value[writeIndex] = '"';
                    writeIndex += 1;
                    escapeNextChar = false;
                } else {
                    const result = try allocator.dupe(u8, value[0..writeIndex]);
                    return .{ .STRING = result };
                }
            },
            '\\' => {
                if (escapeNextChar) {
                    value[writeIndex] = '\\';
                    writeIndex += 1;
                    escapeNextChar = false;
                } else {
                    escapeNextChar = true;
                }
            },
            '\n', 0x9, 0xE => return error.InvalidStringLiteral,
            else => |char| {
                value[writeIndex] = char;
                writeIndex += 1;
            },
        };

        // if we don't hit a closing double quote,
        // then that's an error in both formats.

        return error.InvalidStringLiteral;
    }

    /// Iterates over a symbol region in the source
    /// and emits a SYMBOL or INTEGER token when
    /// complete.
    ///
    /// This function assumes that when it is invoked,
    /// source[index] is an alphanumeric character.
    fn parseSymbol(self: *TokenIterator, firstChar: u8, allocator: *Allocator) !Token {
        var value: [255]u8 = undefined; // max symbol length is 255
        value[0] = firstChar;

        var writeIndex: u8 = 1;
        var isIntegerSymbol: bool = true;

        while (self.index + 1 < self.source.len) : (
            self.index += 1
        ) switch (self.source[self.index + 1]) {
            'A'...'Z',
            'a'...'z',
            '+', '=',
            ':', '-',
            '_', '#' => |char| {
                value[writeIndex] = char;
                writeIndex += 1;
                isIntegerSymbol = false;
            },

            '0'...'9' => |digit| {
                value[writeIndex] = digit;
                writeIndex += 1;
            },

            else => switch (isIntegerSymbol) {
                true => return .{ .INTEGER = try std.fmt.parseInt(u32, value[0..writeIndex], 10) },
                false => {
                    const result = try allocator.dupe(u8, value[0..writeIndex]);
                    return .{ .SYMBOL = result };
                }
            },

        };

        // we assume that a terminal symbol is still
        // valid in input format, and so return it

        return switch (isIntegerSymbol) {
            true => Token{ .INTEGER = try std.fmt.parseInt(u32, value[0..writeIndex], 10) },
            false => Token{ .SYMBOL = value[0..writeIndex] },
        };
    }

    /// Iterates over an NAG region in the source
    /// and emits a NUMERIC_ANNOTATION_GLYPH token
    /// when complete.
    ///
    /// This function assumes that when it is invoked,
    /// source[index] == '%'.
    fn parseNumericAnnotationGlyph(self: *TokenIterator) !Token {
        // the result will have at most 3 digits
        var parsedResult: [3]u8 = undefined;
        var writeIndex: usize = 0;

        while (self.index + 1 < self.source.len) : ({
            if (writeIndex > 3) return error.InvalidNumericAnnotationGlyph;
            self.index += 1;
        }) switch (self.source[self.index + 1]) {
            '0'...'9' => |digit| {
                parsedResult[writeIndex] = digit;
                writeIndex += 1;
            },
            else => {
                const result: u8 = try std.fmt.parseInt(u8, parsedResult[0..writeIndex], 10);
                return .{ .NUMERIC_ANNOTATION_GLYPH = result };
            },
        };

        // terminal glyphs are presumably permissible
        const result: u8 = try std.fmt.parseInt(u8, parsedResult[0..writeIndex], 10);
        return .{ .NUMERIC_ANNOTATION_GLYPH = result };
    }
};

/// A general union type for
/// all tag pairs.
const TagPair = union {
    StrTagPair: StrTagPair,
    OptionalTagPair: OptionalTagPair,
};

/// A tag type used by the
/// StrTagName union.
const StrTagName = enum {
    Event,
    Site,
    Date,
    Round,
    White,
    Black,
    Result,
};

/// A tag pair from the standard
/// Seven Tag Roster (STR) as
/// defined by the PGN spec.
const StrTagPair = union(StrTagName) {
    Event: []const u8,
    Site: []const u8,
    Date: Date,
    Round: []const u8,
    White: []const u8,
    Black: []const u8,
    Result: []const u8,
};

/// A tag type used by the
/// OptionalTagPair union.
const OptionalTagName = enum {
    // player information (9.1)
    WhiteTitle,
    BlackTitle,
    WhiteElo,
    BlackElo,
    WhiteUSCF,
    BlackUSCF,
    WhiteNA,
    BlackNA,
    WhiteType,
    BlackType,

    // event information (9.2)
    EventDate,
    EventSponsor,
    Section,
    Stage,
    Board,

    // opening information (9.3, 9.4)
    Opening,
    Variation,
    SubVariation,
    ECO,
    NIC,

    // time & date information (9.5)
    Time,
    UTCTime,
    UTCDate,

    // time control (9.6)
    TimeControl, // NOTE: this has a very specific description, see 9.6.1

    // alternative starting positions (9.7)
    SetUp,
    FEN,

    // game conclusion (9.8)
    Termination,

    // miscellaneous (9.9)
    Annotator,
    Mode,
    PlyCount,
};

/// An optional tag pair
/// as described by section 9
/// of the PGN spec.
const OptionalTagPair = union(OptionalTagName) {
    WhiteTitle: []const u8,
    BlackTitle: []const u8,
    WhiteElo: u16,
    BlackElo: u16,
    WhiteUSCF: u16,
    BlackUSCF: u16,
    WhiteNA: []const u8,
    BlackNA: []const u8,
    WhiteType: []const u8,
    BlackType: []const u8,

    EventDate: Date,
    EventSponsor: []const u8,
    Section: []const u8,
    Stage: []const u8,
    Board: u16,

    Opening: []const u8,
    Variation: []const u8,
    SubVariation: []const u8,
    ECO: []const u8,
    NIC: []const u8,

    Time: Time,
    UTCTime: Time,
    UTCDate: Date,

    TimeControl: []const u8,

    SetUp: bool,
    FEN: fen.FenData,

    Termination: []const u8,

    Annotator: []const u8,
    Mode: []const u8,
    PlyCount: u16,
};


// general tasks
// TODO: write appropriate structs for data
// TODO: robust testing on a larger data set

test "pgn.TokenIterator basic parsing" {
    var buffer: [20]u8 = undefined;
    var fba = std.heap.FixedBufferAllocator.init(&buffer);
    var allocator = fba.allocator();
    // implicit free at end of scope

    var ti = TokenIterator{ .source = "[Name \"hello, world\"]",
                            .allocator = &allocator};

    try expectEqualDeep(Token{ .LEFT_BRACKET = {} }, (try ti.next()).?);
    try expectEqualDeep(Token{ .SYMBOL = &[_]u8{78, 97, 109, 101} }, (try ti.next()).?);
    try expectEqualDeep(Token{ .WHITESPACE = {} }, (try ti.next()).?);
    try expectEqualDeep(Token{ .STRING = &[_]u8{104, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100}}, (try ti.next()).?);
    try expectEqualDeep(Token{ .RIGHT_BRACKET = {} }, (try ti.next()).?);
    try expectEqual(@as(?Token, null), try ti.next());
}

test "pgn.TokenIterator parsing NAGs" {
    var buffer: [20]u8 = undefined;
    var fba = std.heap.FixedBufferAllocator.init(&buffer);
    var allocator = fba.allocator();
    // implicit free at end of scope

    var ti1 = TokenIterator{ .source = "%236", .allocator = &allocator };
    try expectEqualDeep(Token{ .NUMERIC_ANNOTATION_GLYPH = 236 }, (try ti1.next()).?);

    var ti2 = TokenIterator{ .source = "%4", .allocator = &allocator };
    try expectEqualDeep(Token{ .NUMERIC_ANNOTATION_GLYPH = 4 }, (try ti2.next()).?);
}

test "pgn.TokenIterator parsing whitespace" {
    var buffer: [20]u8 = undefined;
    var fba = std.heap.FixedBufferAllocator.init(&buffer);
    var allocator = fba.allocator();
    // implicit free at end of scope

    var ti = TokenIterator{ .source = "  [  ]\n\n   <>", .allocator = &allocator };

    try expectEqualDeep(Token{ .WHITESPACE = {} }, (try ti.next()).?);
    try expectEqualDeep(Token{ .LEFT_BRACKET = {} }, (try ti.next()).?);
    try expectEqualDeep(Token{ .WHITESPACE = {} }, (try ti.next()).?);
    try expectEqualDeep(Token{ .RIGHT_BRACKET = {} }, (try ti.next()).?);
    try expectEqualDeep(Token{ .WHITESPACE = {} }, (try ti.next()).?);
    try expectEqualDeep(Token{ .LEFT_ANGLE_BRACKET = {} }, (try ti.next()).?);
    try expectEqualDeep(Token{ .RIGHT_ANGLE_BRACKET = {} }, (try ti.next()).?);
    try expectEqual(@as(?Token, null), try ti.next());
}

test "pgn.TokenIterator parsing unit tokens" {
    var buffer: [20]u8 = undefined;
    var fba = std.heap.FixedBufferAllocator.init(&buffer);
    var allocator = fba.allocator();
    // implicit free at end of scope

    var ti = TokenIterator{ .source = "<>[]().*", .allocator = &allocator };

    try expectEqualDeep(Token{ .LEFT_ANGLE_BRACKET = {} }, (try ti.next()).?);
    try expectEqualDeep(Token{ .RIGHT_ANGLE_BRACKET = {} }, (try ti.next()).?);
    try expectEqualDeep(Token{ .LEFT_BRACKET = {} }, (try ti.next()).?);
    try expectEqualDeep(Token{ .RIGHT_BRACKET = {} }, (try ti.next()).?);
    try expectEqualDeep(Token{ .LEFT_PARENTHESIS = {} }, (try ti.next()).?);
    try expectEqualDeep(Token{ .RIGHT_PARENTHESIS = {} }, (try ti.next()).?);
    try expectEqualDeep(Token{ .PERIOD = {} }, (try ti.next()).?);
    try expectEqualDeep(Token{ .ASTERISK = {} }, (try ti.next()).?);
    try expectEqual(@as(?Token, null), try ti.next());
}
