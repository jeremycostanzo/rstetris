use crate::{block::Block, pieces::Piece, Point, HEIGHT, WIDTH};
use itertools::Itertools;
use sdl2::{
    pixels::Color,
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext},
};

pub struct Grid {
    blocks: [[Option<Block>; HEIGHT as usize]; WIDTH as usize],
}

impl Default for Grid {
    fn default() -> Self {
        Grid {
            blocks: [[None; HEIGHT as usize]; WIDTH as usize],
        }
    }
}

impl Grid {
    pub fn tile(&self, point: &Point) -> Option<&Option<Block>> {
        let x = usize::try_from(point.x).ok()?;
        let y = usize::try_from(point.y).ok()?;
        self.blocks.get(x).and_then(|column| column.get(y))
    }

    fn tile_mut(&mut self, point: &Point) -> Option<&mut Option<Block>> {
        let x = usize::try_from(point.x).ok()?;
        let y = usize::try_from(point.y).ok()?;
        self.blocks.get_mut(x).and_then(|column| column.get_mut(y))
    }

    pub fn add_piece(&mut self, piece: &Piece) -> Option<()> {
        for point in piece.points() {
            if self.tile(&point)?.is_some() {
                return None;
            };
        }

        for point in piece.points() {
            let tile = self.tile_mut(&point)?;
            *tile = Some(Block::new(piece.color));
        }

        piece
            .points()
            .iter()
            .map(|point| point.y)
            .sorted()
            .dedup()
            .for_each(|y| {
                self.complete_line(y);
            });
        Some(())
    }

    pub fn draw<'a>(
        &self,
        canvas: &mut Canvas<Window>,
        creator: &'a TextureCreator<WindowContext>,
    ) {
        for (x, column) in self.blocks.iter().enumerate() {
            for (y, block) in column.iter().enumerate() {
                block
                    .unwrap_or_else(|| {
                        Block::new(if (x + y) % 2 == 0 {
                            Color::BLACK
                        } else {
                            Color::RGB(30, 0, 0)
                        })
                    })
                    .draw(canvas, &Point::new(x as i32, y as i32), creator);
            }
        }
    }

    fn complete_line(&mut self, y: i32) -> Option<bool> {
        for x in 0..self.blocks.len() {
            if self.tile(&Point::new(x as i32, y))?.is_none() {
                return Some(false);
            }
        }
        for height in (0..=y).rev() {
            for x in 0..self.blocks.len() {
                let x = x as i32;
                let y = height as i32;
                *self.tile_mut(&Point::new(x, y))? = *self.tile(&Point::new(x, y - 1))?
            }
        }
        Some(true)
    }
}
