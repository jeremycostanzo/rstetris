use crate::{block::Block, grid::Grid, Point};

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

            let x = usize::try_from(translated.x).ok()?;
            let y = usize::try_from(translated.y).ok()?;

            grid.blocks.get(x).and_then(|column| column.get(y))?;
        }

        for point in self.body.iter_mut() {
            *point = point.add(&translation);
        }

        Some(())
    }

    pub fn draw<'a>(
        &mut self,
        canvas: &mut Canvas<Window>,
        creator: &'a TextureCreator<WindowContext>,
    ) {
        for point in self.body.iter() {
            Block::new(self.color).draw(canvas, &point.add(&self.position), creator);
        }
    }
}
