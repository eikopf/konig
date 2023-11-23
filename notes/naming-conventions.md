# Naming Conventions for `konig`
This document lists a variety of naming conventions for `konig`, in the particular cases where it's ambiguous. As usual, American standards and conventions are preferred for both code and documentation, e.g. writing *color* instead of *colour*.

## SAN
SAN (Standard Algebraic Notation) is always given as a single contiguous *literal*, and is composed of a leading *body* and several optional trailing *suffixes*.

## FEN
FEN (Forsyth-Edwards Notation) is always given as a full space-delimited *string*; in particular the words *halfmove* and *fullmove* should always be treated as a compound, not separated by a space or dash.

## Pieces
To avoid collision with the `type` keyword, pieces are instead described as having a *color* and a *kind*.
