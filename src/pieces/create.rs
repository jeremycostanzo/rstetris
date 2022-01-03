use crate::Point;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

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
            Point::new(2, 0),
            Point::new(2, 1),
            Point::new(2, 2),
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

impl Distribution<Piece> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Piece {
        enum Variant {
            T,
            Z,
            S,
            I,
            Square,
            L,
            ReverseL,
        }

        let variant = match rng.gen_range(0..=6) {
            // rand 0.8
            0 => Variant::T,
            1 => Variant::Z,
            2 => Variant::S,
            3 => Variant::I,
            4 => Variant::Square,
            5 => Variant::L,
            _ => Variant::ReverseL,
        };

        let mut piece = match variant {
            Variant::T => t(),
            Variant::Z => z(),
            Variant::S => s(),
            Variant::I => i(),
            Variant::Square => square(),
            Variant::L => l(),
            Variant::ReverseL => reverse_l(),
        };

        piece.position.x +=
            rng.gen_range(0..(crate::WIDTH as i32 - piece.bounding_square_size() as i32));

        piece
    }
}
