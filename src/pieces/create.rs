use crate::Point;
use sdl2::pixels::Color;

use super::Piece;

pub fn t() -> Piece {
    Piece::new(
        vec![
            Point::new(0, 0),
            Point::new(0, 1),
            Point::new(0, 2),
            Point::new(1, 1),
        ],
        Color::YELLOW,
    )
}

pub fn i() -> Piece {
    Piece::new(
        vec![
            Point::new(0, 0),
            Point::new(0, 1),
            Point::new(0, 2),
            Point::new(0, 3),
        ],
        Color::RED,
    )
}

pub fn l() -> Piece {
    Piece::new(
        vec![
            Point::new(0, 0),
            Point::new(0, 1),
            Point::new(0, 2),
            Point::new(1, 2),
        ],
        Color::GREEN,
    )
}

pub fn reverse_l() -> Piece {
    Piece::new(
        vec![
            Point::new(0, 0),
            Point::new(0, 1),
            Point::new(0, 2),
            Point::new(1, 2),
        ],
        Color::CYAN,
    )
}

pub fn square() -> Piece {
    Piece::new(
        vec![
            Point::new(0, 0),
            Point::new(0, 1),
            Point::new(1, 0),
            Point::new(1, 1),
        ],
        Color::RGB(100, 100, 100),
    )
}

pub fn s() -> Piece {
    Piece::new(
        vec![
            Point::new(0, 2),
            Point::new(1, 2),
            Point::new(1, 1),
            Point::new(2, 1),
        ],
        Color::RGB(90, 12, 230),
    )
}

pub fn z() -> Piece {
    Piece::new(
        vec![
            Point::new(0, 1),
            Point::new(1, 1),
            Point::new(1, 2),
            Point::new(2, 2),
        ],
        Color::RGB(200, 50, 100),
    )
}
