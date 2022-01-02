use sdl2::{
    pixels::Color,
    rect::Rect,
    render::{Canvas, Texture, TextureCreator},
    video::{Window, WindowContext},
};

pub const SIZE: u32 = 16;

pub fn texture_from_color<'a>(
    canvas: &mut Canvas<Window>,
    creator: &'a TextureCreator<WindowContext>,
    color: Color,
) -> Texture<'a> {
    let draw_color = canvas.draw_color();
    let mut texture = creator
        .create_texture(None, sdl2::render::TextureAccess::Target, SIZE, SIZE)
        .unwrap();

    canvas
        .with_texture_canvas(&mut texture, |canvas| {
            canvas.set_draw_color(color);
            canvas.clear();
        })
        .expect("Could not create texture");
    canvas.set_draw_color(draw_color);
    texture
}

pub fn draw(canvas: &mut Canvas<Window>, point: crate::Point, texture: &Texture) {
    let x = point.x as u32;
    let y = point.y as u32;
    canvas
        .copy(
            texture,
            None,
            Rect::new((x * SIZE) as i32, (y * SIZE) as i32, SIZE, SIZE),
        )
        .expect("Could not draw block");
}
