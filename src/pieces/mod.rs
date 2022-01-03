use crate::{block::Block, grid::Grid, Point};

use sdl2::{
    pixels::Color,
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext},
};

mod create;

#[derive(Debug)]
pub struct Piece {
    position: Point,
    body: Vec<Point>,
    pub color: Color,
}

#[derive(Debug)]
pub enum Direction {
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

    fn bounding_square_size(&self) -> usize {
        let mut size = 0;
        for point in &self.body {
            let max_coord = point.x.max(point.y) as usize;
            if max_coord > size {
                size = max_coord
            }
        }
        size
    }

    pub fn translate(&mut self, grid: &Grid, direction: Direction) -> Option<()> {
        use Direction::*;
        let translation = match direction {
            Left => Point::new(-1, 0),
            Right => Point::new(1, 0),
            Up => Point::new(0, -1),
            Down => Point::new(0, 1),
        };

        for point in &self.body {
            let translated = self.position.add(&point.add(&translation));

            if grid.tile(&translated)?.is_some() {
                return None;
            };
        }

        self.position = self.position.add(&translation);

        Some(())
    }

    pub fn rotate(&mut self, grid: &Grid) -> Option<()> {
        let mut rotated = self.body.clone();

        for point in rotated.iter_mut() {
            *point = Point {
                x: self.bounding_square_size() as i32 - point.y,
                y: point.x,
            };

            let rotated = point.add(&self.position);

            if grid.tile(&rotated)?.is_some() {
                return None;
            }
        }

        self.body = rotated;

        Some(())
    }

    pub fn draw<'a>(
        &self,
        canvas: &mut Canvas<Window>,
        creator: &'a TextureCreator<WindowContext>,
    ) {
        for point in self.points().iter() {
            Block::new(self.color).draw(canvas, point, creator);
        }
    }

    pub fn points(&self) -> Vec<Point> {
        self.body
            .iter()
            .map(|point| point.add(&self.position))
            .collect()
    }
}
