use super::{
    block::{self, texture_from_color},
    Point,
};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use sdl2::{
    pixels::Color,
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext},
};

mod create;

#[derive(Debug)]
pub struct Piece {
    pub position: Point,
    body: Vec<Point>,
    color: Color,
}

#[derive(Debug)]
pub enum CollisionDirection {
    Left,
    Right,
    Up,
    Down,
}

impl Piece {
    fn new(body: Vec<Point>, color: Color) -> Piece {
        Piece {
            position: Point::new(0, 0),
            body,
            color,
        }
    }

    pub fn draw<'a>(
        &mut self,
        canvas: &mut Canvas<Window>,
        creator: &'a TextureCreator<WindowContext>,
    ) {
        let position = &self.position;
        let texture = texture_from_color(canvas, creator, self.color);
        for point in self.body.iter() {
            block::draw(canvas, point.add(position), &texture)
        }
    }
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

        match variant {
            Variant::T => create::t(),
            Variant::Z => create::z(),
            Variant::S => create::s(),
            Variant::I => create::i(),
            Variant::Square => create::square(),
            Variant::L => create::l(),
            Variant::ReverseL => create::reverse_l(),
        }
    }
}
