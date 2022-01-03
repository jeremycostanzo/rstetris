extern crate sdl2;

use rand::Rng;
use rstetris::{grid::Grid, HEIGHT, WIDTH};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::collections::HashSet;
use std::time::Duration;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(
            "rstetris",
            WIDTH * rstetris::block::SIZE,
            HEIGHT * rstetris::block::SIZE,
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

    use rstetris::pieces::Piece;

    let mut piece: Piece = rand::random();
    let x = rand::thread_rng().gen_range(0..=WIDTH - 4) as i32;
    let y = rand::thread_rng().gen_range(0..=HEIGHT - 4) as i32;

    let random_point = rstetris::Point::new(x, y);

    piece.position = piece.position.add(&random_point);

    piece.draw(&mut canvas, &texture_creator);

    canvas.present();

    let grid = Grid::default();

    let mut prev_keys = HashSet::new();

    'running: loop {
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'running;
            };
        }

        let keys: std::collections::HashSet<Keycode> = event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        // Get the difference between the new and old sets.
        let old_keys = &prev_keys - &keys;

        for key in &old_keys {
            use rstetris::pieces::Direction;
            match key {
                Keycode::Left => {
                    piece.translate(&grid, Direction::Left);
                }
                Keycode::Right => {
                    piece.translate(&grid, Direction::Right);
                }
                Keycode::Up => {
                    piece.translate(&grid, Direction::Up);
                }
                Keycode::Down => {
                    piece.translate(&grid, Direction::Down);
                }
                _ => {}
            }
        }

        prev_keys = keys;
        // The rest of the game loop goes here...

        canvas.clear();

        piece.draw(&mut canvas, &texture_creator);

        canvas.present();

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
