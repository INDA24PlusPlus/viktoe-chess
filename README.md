# viktoe-chess

## Example

```
let game = ChessGame::default();

let game_state = GameState::Ongoing;

while (matches!(game_state, GameState::CheckMate)) {
    let space_to_move_from = BoardPosition::from((E, Two));

    let moves = game.get_valid_moves(&piece_to_move);

    let space_to_move_to = BoardPosition::from((E, Four));

    game_state = game.move_piece(space_to_move_from, space_to_move_to)

    if matches!(game_state, Promotion(_)) {
        let promotion_target = Piece::Queen;

        game_state = game.promotion_target(space_to_move_to, promotion_target);
    }
}
```

# Structs
## ChessGame

struct ChessGame {
    private fields
}

### Methods

fn get_valid_moves(&self, position: &BoardPosition) -> Option<Board<MoveType>>

fn move_piece(
    initial_position: &BoardPosition,
    desired_position: &BoardPosition,
) -> Result<GameState, ChessError>

fn request_draw_due_to_repeated_position() -> bool

fn promote_pawn(
    pawn_position: &BoardPosition,
    promotion_target: Piece,
    move_type: &MoveType,
) -> Result<GameState, ChessError>

fn get_player_turn() -> &Turn

fn get_game_state() -> &GameState

### Traits

#### impl Default for ChessGame
    fn default() -> Self

## Board

struct Board<T> {
    private fields
}

### Methods

fn get(position: &BoardPosition) -> &Option<T>

### Traits

#### impl<T> Default for Board<T>
    fn default() -> Board<T>

## BoardPosition

struct BoardPosition {
    private fields
}

### Methods

fn get_file(&self) -> &File

fn get_rank(&self) -> &Rank

fn add(&self, vector: (i8, i8)) -> Result<Self, ChessError>

### Traits

#### impl From<BoardPosition> for (u8, u8)
    fn from(value: BoardPosition) -> Self

#### impl From<&BoardPosition> for (u8, u8)
    fn from(value: &BoardPosition) -> Self

#### impl TryFrom<(u8, u8)> for BoardPosition
    type Error = ChessError;

    fn try_from(value: (u8, u8)) -> Result<Self, Self::Error>

#### impl From<(File, Rank)> for BoardPosition
    fn from(value: (File, Rank)) -> Self

# Enums

## Turn

enum Turn {
    White,
    Black,
}

## GameState

enum GameState {
    Ongoing,
    Check,
    CheckMate,
    Draw,
    Promotion(BoardPosition, Board<MoveType>),
}

## MoveType

enum MoveType {
    Move,
    Capture,
}

## Color

enum Color<T> {
    White(T),
    Black(T),
}

### Methods

fn get_internal() -> &T

### Traits

#### impl<T> Default for Color<T> where T: Default,
    fn default() -> Self

## Piece

enum Piece {
    King { private fields },
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn { private fields },
}

## File

enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

### Traits

#### impl From<File> for u8
    fn from(value: File) -> Self

#### impl From<&File> for u8
    fn from(value: &File) -> Self

#### impl TryFrom<u8> for File
    type Error = ChessError;

    fn try_from(value: u8) -> Result<Self, Self::Error>

## Rank

enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

### Traits

#### impl From<Rank> for u8
    fn from(value: Rank) -> Self

#### impl From<&Rank> for u8
    fn from(value: &Rank) -> Self

#### impl TryFrom<u8> for Rank
    type Error = ChessError;

    fn try_from(value: u8) -> Result<Self, Self::Error>

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

# Known errors

Castling state does not change
When castling, rook does not move
King can move into each others space
Pawn is not taken by en passant
