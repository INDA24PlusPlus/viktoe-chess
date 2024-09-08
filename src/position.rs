pub type BoardPosition = (Horizontal, Vertical);

pub enum Horizontal {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

pub enum Vertical {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

pub fn board_position_to_coordinate(position: BoardPosition) -> (u8, u8) {
    let (x, y) = position;

    let x = match x {
        Horizontal::One => 0,
        Horizontal::Two => 1,
        Horizontal::Three => 2,
        Horizontal::Four => 3,
        Horizontal::Five => 4,
        Horizontal::Six => 5,
        Horizontal::Seven => 6,
        Horizontal::Eight => 7,
    };

    let y = match y {
        Vertical::A => 0,
        Vertical::B => 1,
        Vertical::C => 2,
        Vertical::D => 3,
        Vertical::E => 4,
        Vertical::F => 5,
        Vertical::G => 6,
        Vertical::H => 7,
    };

    (x, y)
}

pub fn coordinate_to_board_position(position: (u8, u8)) -> Result<BoardPosition, ()> {
    let (x, y) = position;

    let x = match x {
        0 => Horizontal::One,
        1 => Horizontal::Two,
        2 => Horizontal::Three,
        3 => Horizontal::Four,
        4 => Horizontal::Five,
        5 => Horizontal::Six,
        6 => Horizontal::Seven,
        7 => Horizontal::Eight,
        _ => return Err(())
    };

    let y = match y {
        0 => Vertical::A,
        1 => Vertical::B,
        2 => Vertical::C,
        3 => Vertical::D,
        4 => Vertical::E,
        5 => Vertical::F,
        6 => Vertical::G,
        7 => Vertical::H,
        _ => return Err(())
    };

    Ok((x, y))
}
