# [Generating Legal Moves Efficiently](https://peterellisjones.com/posts/generating-legal-chess-moves-efficiently/)
> These notes are a summary (and a derived implementation outline) of the excellent 2017 article by Peter Ellis Jones, linked in the heading. Refer there for details that appear absent here, as well as his generally excellent other writing.

## (Why We Don't Use) Pseudolegal Move Generation
Pseudolegal moves are legal board moves which do not guarantee king safety; typically these are faster to generate than legal board moves but must be played to assess their legality. Ultimately, generating legal moves by first generating pseudolegal moves and then validating them is slower than a (well-written) procedure which is guaranteed to produce fully legal moves.

## Legal Move Generation
Jones outlines a legal move generation procedure based on attack bitboards, handling in sequence:
1. king moves;
2. check evasion;
2. pinned pieces;
3. ordinary moves.

For the sake of simplicity, we assume that an implementor already has a way to compute attack and move bitboards for relevant pieces.

### King Moves
The moves which are most likely to be illegal are king moves, and so we handle those first; note also that if we find the king to be in check that we can disregard all moves that don't move it out of check.

We calculate a *king danger* bitboard by (temporarily) removing the king and generating attack bitboards for all opposing pieces, and this is used to mask off illegal moves from the king's move bitboard. Note that removing the king here is a critical step, since the king might block the attack bitboard of a sliding piece such that a move that keeps the king in that attacking line is treated as legal.

> You can think of this as pieces being able to "see through" the king.

### Check Evasion
If the king is in check, then the set of legal moves is strictly limited to moves which get it out of check. This set is represented as a bitboard, and for each type of piece we pretend that the king is that piece and check to see whether it is possible for that piece to reach an opposing piece of the same type in one move (by ANDing its attack bitboard with the bitboard of opposing pieces of that type, and OR-assigning it to to the bitboard of attacking pieces).

If this set contains more than a single piece, the king is considered to be in *double check*; in this case **the only legal moves are king moves**. Otherwise, the king is in single check, and we have the following options:

1. move the king out of check;
2. capture the checking piece;
3. block the checking piece (if it is a sliding piece).

Option (1) can be calculated as given above, and for (2) and (3) we define two additional bitboards to help: a *capture mask* and a *push mask*. The capture mask represents the set of all capturing moves (i.e. their target squares) that will get the king out of check, and likewise the push mask represents the set of all squares that a piece could move to in order to block a checking piece; these masks default to `u64::MAX` to represent a situation where the king is not in check. To actually extract valid moves from these masks, we must use the same "imaginary piece" technique to find pieces that are allowed to move to these squares.

> Two distinct masks are required here to account for *en passant* captures, since this is the only kind of move in chess where the target square and the square of the captured piece are distinct.

### Pinned Pieces
Whether or not the king is in check, legal move generation must account for pinned pieces; such pieces can only move in the direction they are pinned, i.e. radially inwards and outwards relative to the king. There are at most 8 directional rays to consider for pinned pieces, equivalent to the queen's move bitboard if it were at the location of the king, and we check for the closest sliding pieces that lie along these rays.

For a particular sliding piece, the intersection of its move bitboard and the directional ray bitboard will be the length of the line of attack towards the king up to the pinned piece, called the *pinning line*. This piece is then free to move to anywhere on the intersection of the pinning line and its move board.

> Take care when using this intersection method, as sliding pieces may intersect multiple rays; their move boards therefore have to be masked off to isolate *only* the specific ray along which they can pin pieces to the king.

### Other Pieces
Other moves are calculated as normal, though we can use the information gathered in previous steps to cull some unnecessary work:

- castling is impossible if the king is in check;
- the capture and push masks can be used to filter available moves if the king is in check.

A common bug here is an obscure situation called an *en passant discovered check*, in which an en passant capture by an attacking pawn against an opposing pawn opens up the friendly king to an attack that was blocked by both pieces. These moves are rare enough that Jones recommends just doing a separate check for these each time an en passant move is generated; this check simply involves removing both pawns and checking for a horizontal attack against the king by a queen or rook.

> Recall in particular the rules of castling: neither the king or relevant rook can have moved, the squares between them must be vacant, and the king cannot leave, pass through, or end on a square under attack.

### Optimizations
Jones recommends the following options for increasing performance (benchmarked against qperft):

- SIMD operations on multiple bitboards simueltaneously;
- Kogge-Stone generators for sliding moves;
- Something called the o^(o-2r) trick using the Intel PSHUFB instruction;
- Using the OSX Instruments profiler to find hotspots for optimization.
