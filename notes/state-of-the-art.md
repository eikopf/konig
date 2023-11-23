# Current SOTA of Chess Crates
> Last updated 2023-11-21

This document is a review of other public crates which occupy a similar space, with a particular focus on API designs and representations of common types (`Board`, `Game`, `Index`/`Square`, etc.).

## [`chess`](https://crates.io/crates/chess)
- Public since 2016, roughly 39k downloads.
- This project is explicitly a fast move generator, not an engine.

### Structs and Traits
- `Move`s are defined by a pair of `Square`s and an `Option<Piece>` for promotion.
- A `Square` is just a wrapped u8 in the range \[0, 63\]. An unsafe public method `new(sq: u8)` is given to construct one without safety checks.
- A `Piece` is an enum of piece kinds; in combination with the `Color` enum this gives a complete representation equivalent to `konig`'s standard piece.

### Board Representation
The `Board` in this crate is defined as follows,

```rust
pub struct Board {
    pieces: [BitBoard; NUM_PIECES],
    color_combined: [BitBoard; NUM_COLORS],
    combined: BitBoard,
    side_to_move: Color,
    castle_rights: [CastleRights; NUM_COLORS],
    pinned: BitBoard,
    checkers: BitBoard,
    hash: u64,
    en_passant: Option<Square>,
}
```

where a `BitBoard` just wraps a public `u64`. It's important to remember here that `chess` is designed for move *generation*, so a number of the fields here are redundant, and used for efficient lookup.`combined`, `pinned`, `checkers`, and `hash` are all solely relevant from a move generation perspective; they each have to be updated in concert with the canonical board representation in `pieces` and `color_combined` to stay valid.

This representation takes up (and I am admittedly eyeballing this) 79 bytes for the essential board-state information, and 111 bytes in total. This could shrink by a byte with a niche optimization for the en_passant field, but I assume that alignment would basically always push this up to 112 bytes. Compared to my (aligned to) 520 byte `StandardBoard`, this is a much leaner representation, and therefore much more practical to shove into a transposition table. It's probably a good idea to come back around to this when I start implementing move generation.

### San Parsing
- `chess` handles parsing with a handwritten parser.
- The private comments note that "there are only 186624 possible moves in SAN notation," and they estimate that a hashtable of all possible moves would take approximately 2MiB.
- This parser permits an optional "e.p." suffix; it's probably a good idea to replicate the same behaviour in `konig`.
- Is it worth testing this parser against the current `nom` implementation? It'd be heartbreaking if it was faster, but `nom` does claim some impressive perf stats.

### Fen Parsing
- `chess`'s handwritten FEN parser is buried in the `FromStr` impl on the `BoardBuilder` struct.
- There's nothing particularly odd here, FEN is pretty well defined. Might be worth doing some perf tests against this parser?
- Interestingly, `BoardBuilder` has almost the exact same internal representation as `konig`'s StandardBoard, except that an `Option<File>` is used for the en passant target square. `File` is just an enum of the letters `A` to `H`, though, and I think I get all the same niche optimizations when using `StandardIndex` by just wrapping a `NonMaxU8`.

### API Design
- Generally this is the weakest part of the crate.
- The `BoardBuilder` design is an exception to that rule, and is probably worth drawing from. 
- Obviously, the API is designed with move generation as the first priority, and that shows in other places, but even the examples given on the README are hard to read, and can get the user too involved in implementation details.

## [`shakmaty`](https://crates.io/crates/shakmaty)
- Public since 2017, roughly 91k downloads.
- This project focuses primarily on move generation, but includes functionality for all Lichess variants rather than just standard chess.

### Structs and Traits
- A `Chess` struct defines a given position with the same fields (and ordering) as a FEN string, and delegates the board representation itself to a `Board` struct.
- The `Board` struct is composed of 9 bitboards, where 6 correspond to a piecekind (here called a role), 2 correspond to a color, and one is the unified occupation bitboard.
- Individual locations are called `Square`s, and this is given in one large `#[repr(u8)]` enum.
- The interface for `Chess` is given as a single (massive) `Position` trait, containing methods relevant to only some Lichess variants as well as core functionality.

### Hashing and Transpositions
- Most of this functionality is contained in the `zobrist` module. The headline note in its documentation is that Zobrist hashes are guaranteed to be stable, and that changing hash values would constitute a semver breaking change.
- A macro is defined to implement the `ZobristValue` trait on `ZobristN` structs for `N = 8, 16, 32, 64, 128`.
- `ZobristHash` is a SAM trait for hashing `&self`, taking an `EnPassantMode` parameter, and returning some generic `ZobristValue`.

### At Compile Time
- There's a lot of hidden detail in the `bootstrap` and `magics`, where const functions are used to declare static lookup tables at compile time, and also to provide magic bitboards.

### References
- Some documentation on the `Position` trait points to FIDE's Laws of Chess §9.2 to define what is precisely meant by a "repeated position." Roughly, FIDE defines this as "the literal board representation matches exactly, and all the same moves are possible." This implies in particular that the castling permissions must be the same, and that there can be no en passant target square (since they cannot reoccur within the same game).
- [This](https://chasolver.org) is a reference dynamic unwinnability solver, used to check if one player's current prospects are entirely hopeless.
- A [TalkChess thread](http://www.talkchess.com/forum3/viewtopic.php?p=727500&t=64790) is linked to provide some good compact magic bitboards.

### Notes
- This is an extremely good library to crib off of for performance tricks and fine implementation details; I particularly like the use of const functions and heavy inlining to do large chunks of work at compile time.
- If I had a major critique, it's that `shakmaty` isn't very *rusty*. Traits are treated as literally just interfaces, and a lot of functionality could be moved from `impl Struct` blocks to `impl Trait for Struct` blocks with some attention and redesigning.
- As with `chess`, the API for invoking moves on a position is quite clunky.
- On the topic of magic bitboards, I really dislike just sticking them into a private module with no context and a single link. Maybe I could provide a binary with `konig` to generate and format them, so that a user who wanted to could read the implementation themselves.

## [`pgn_reader`](https://crates.io/crates/pgn-reader)
- Public since 2017, roughly 78k downloads.
- A streaming non-allocating reader for PGN games.
- Significant dependency on `shakmaty`.

### SAN Representation
- The data derived from a SAN literal is placed into a `SanPlus` struct; the annotation suffix is instead converted into a numeric annotation glyph (NAG) and returned as a separate `Nag` struct.
- This `Nag` separation is reasonable, as a NAG can be any value of `u8` in PGN data, and so may or may not correspond to one of the traditional suffix annotations.

### PGN Representation
- Large chunks of the validation process are delegated to `shakmaty`, and some of its structs show up in `pgn_reader`s public API.
- The user implements a `Visitor`, initializes a `BufferedReader` over the source data, and then hands a mutable reference to an instance of `Visitor` into the `read_game` method. This provides a streaming view over the data, while delegating how it accesses that data to the `Visitor`. It's somewhat ambiguous as to whether this handles full PGN databases, or just individual games.

### Notes
- This is licensed under GPL-3, so I can't copy anything from the actual implementation. Still, I'm happy to agree that the Visitor pattern is the right one to use here, and I'll likely do something similar in my own PGN implementation.
- Streaming PGN data is probably the correct move, as the data sizes can get large enough to warrant it. Rather than rolling my own, though, I'll try to use `nom`'s streaming parsers.
- I absolutely need to provide a `Pgn` struct that someone can use to just parse an individual game, and possibly a `PgnDatabase` struct for the database files, but should I also expose `Visitor` and (some equivalent to) `BufferedReader` for allowing users to create custom implementations. I'm leaning towards yes, but it needs more attention.

