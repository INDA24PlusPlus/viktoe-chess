# viktoe-chess

## Example

```
use viktoe-chess::prelude::*;

let game = ChessGame::default();

let game_state = GameState::Ongoing;

while (matches!(game_state, GameState::CheckMate)) {
    let space_to_move_from = BoardPosition::from((E, Two));

    let moves = game.get_valid_moves(&space_to_move_from);

    let space_to_move_to = BoardPosition::from((E, Four));

    game_state = game.move_piece(space_to_move_from, space_to_move_to)

    if matches!(game_state, Promotion(_)) {
        let promotion_target = Piece::Queen;

        game_state = game.promote_pawn(promotion_target);
    }
}
```

# Documentation

Documentation is aquired by cloning the repo and running cargo doc --open

# Rules implemented
- [x] Normal moves
    - [x] King
    - [x] Queen
    - [x] Bishop
    - [x] Knight
    - [x] Rook
    - [x] Pawn
- [x] Check
- [x] Checkmate
- [x] Board repetition
- [x] 50-move rule
- [ ] No mating pieces
- [ ] Patt
- [x] Castling
    - [x] Niether the king or the rook has moved
    - [x] There is no piece between the king and the rook
    - [x] The king does not move over a threatend piece
    - [x] The king is not in check when begining the move
- [x] En passant
- [x] Promotion
