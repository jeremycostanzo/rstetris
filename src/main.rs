extern crate sdl2;

use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

const WIDTH: u32 = 12;
const HEIGTH: u32 = 24;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(
            "rstetris",
            WIDTH * rstetris::block::SIZE,
            HEIGTH * rstetris::block::SIZE,
        )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        use rstetris::pieces::Piece;
        let mut piece: Piece = rand::random();
        let x = rand::thread_rng().gen_range(0..=WIDTH - 4) as i32;
        let y = rand::thread_rng().gen_range(0..=HEIGTH - 4) as i32;

        let random_point = rstetris::Point::new(x, y);

        piece.position = piece.position.add(&random_point);

        piece.draw(&mut canvas, &texture_creator);

        canvas.present();

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
