# Konig
> Oder `koenig`? Weiß ich nicht.

A Rust-based chess engine, built to support custom implementations and chess variants. In general, it should be just as easy to implement standard chess, hex chess, or an infinite chess board with the traits and definitions in this crate.

## Top-level `TODO`s for `v0.1.0`
- [ ] `konig::core`
  - The essential traits and definitions for `konig`.
  - [x] Finalise trait definitions.
  - [x] Investigate: should the given traits be object-safe?
    - This would require removing `std::ops::Index` from the supertraits of `core::board::Board`, for example.
  - [ ] Complete and review documentation.
- [ ] `konig::standard`
  - An implementation of standard chess using `konig::core`.
  - [ ] Move validation.
  - [ ] Move processing.
  - [ ] Display implementation.
  - [ ] Significant testing.
  - [ ] Complete and review documentation.
- [ ] `konig::io`
  - Parsing (and streaming?) for common chess formats.
  - [ ] Implement `konig::io::fen`.
    - [x] Review: should `FenData.as_board` return a `StandardBoard`, or a custom `Board`?
    - [ ] Finalise the API on `FenData`.
    - [ ] Add significant testing from real-world datasets.
    - [ ] Add and review documentation.
  - [ ] Implement `konig::io::san`.
    - [ ] Create structs and basic API.
    - [ ] Implement parser with `nom`.
    - [ ] Add significant testing from real-world datasets.
    - [ ] Add and review documentation.
  - Implementations for [EPD](https://www.chessprogramming.org/Extended_Position_Description) and [PGN](https://www.chessprogramming.org/Portable_Game_Notation) are blocked until a later time when a `konig::core::game::Game` trait is implemented (likely deferred to `v0.2.0`)

## Usage
`TODO`, i.e., DON'T USE THIS LIBRARY YET.

It's still very unfinished.
