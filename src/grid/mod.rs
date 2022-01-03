use crate::{block::Block, HEIGHT, WIDTH};

pub struct Grid<'a> {
    pub blocks: &'a [&'a [Option<Block>; HEIGHT as usize]; WIDTH as usize],
}

impl<'a> Default for Grid<'a> {
    fn default() -> Self {
        Grid {
            blocks: &[&[None; HEIGHT as usize]; WIDTH as usize],
        }
    }
}
