use sdl2::{
    pixels::Color,
    rect::Rect,
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext},
};

pub const SIZE: u32 = 16;

#[derive(Clone, Copy, Debug)]
pub struct Block {
    color: Color,
}

impl Block {
    pub fn new(color: Color) -> Self {
        Block { color }
    }

    pub fn draw<'a>(
        self,
        canvas: &mut Canvas<Window>,
        point: &super::Point,
        creator: &'a TextureCreator<WindowContext>,
    ) {
        let draw_color = canvas.draw_color();
        let mut texture = creator
            .create_texture(None, sdl2::render::TextureAccess::Target, SIZE, SIZE)
            .unwrap();

        canvas
            .with_texture_canvas(&mut texture, |canvas| {
                canvas.set_draw_color(self.color);
                canvas.clear();
            })
            .expect("Could not create texture");
        canvas.set_draw_color(draw_color);
        let x = point.x as u32;
        let y = point.y as u32;
        canvas
            .copy(
                &texture,
                None,
                Rect::new((x * SIZE) as i32, (y * SIZE) as i32, SIZE, SIZE),
            )
            .expect("Could not draw block");
    }
}
