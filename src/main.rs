extern crate sdl2;

use rstetris::{grid::Grid, pieces::Direction, HEIGHT, WIDTH};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::collections::HashSet;
use std::time::Duration;

const SPEED_FALLING_DURATION: i32 = 2;

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

    let mut grid = Grid::default();

    piece.draw(&mut canvas, &texture_creator);
    canvas.present();

    let mut prev_keys = HashSet::new();

    let mut update_when_frame_above = 20;
    let mut frame = 0;
    let mut current_speed_falling_value = 0;

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

        if keys.contains(&Keycode::J) {
            current_speed_falling_value += 1;
        }
        if current_speed_falling_value > SPEED_FALLING_DURATION {
            current_speed_falling_value = 0;
            piece.translate(&grid, Direction::Down);
        }

        // Get the difference between the new and old sets.
        let old_keys = &prev_keys - &keys;

        for key in &old_keys {
            match key {
                Keycode::H => {
                    piece.translate(&grid, Direction::Left);
                }
                Keycode::L => {
                    piece.translate(&grid, Direction::Right);
                }
                Keycode::K => {
                    piece.rotate(&grid);
                }
                Keycode::I => {
                    while piece.translate(&grid, Direction::Down).is_some() {}
                    grid.add_piece(&piece);
                    frame = 0;
                    piece = rand::random();
                }
                _ => {}
            }
        }

        prev_keys = keys;

        if frame > update_when_frame_above {
            if piece.translate(&grid, Direction::Down).is_none() {
                grid.add_piece(&piece);
                piece = rand::random();
            }
            frame = 0;
        } else {
            frame += 1;
        }

        canvas.clear();

        grid.draw(&mut canvas, &texture_creator);
        piece.draw(&mut canvas, &texture_creator);

        canvas.present();

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
