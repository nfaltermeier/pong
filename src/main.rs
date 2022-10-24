extern crate sdl2;
extern crate rand;

use actor::{Position, UpdateInfo};
use actors::ball::Ball;
use actors::player_paddle::PlayerPaddle;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels;

use std::cell::RefCell;
use std::collections::HashSet;
use std::time::{Duration, Instant};

mod actor;
mod actors;
mod math_helper;

const SCREEN_WIDTH: i16 = 800;
const SCREEN_HEIGHT: i16 = 600;

const TARGET_FRAMERATE: f64 = 60.0;
const TARGET_FRAMETIME_MICROS: u128 = (1.0 / TARGET_FRAMERATE * 1_000_000.0) as u128;

const FIXED_UPDATE_RATE: f64 = 60.0;
const FIXED_UPDATE_TIME_SECS_F64: f64 = 1.0 / FIXED_UPDATE_RATE;
const FIXED_UPDATE_TIME_MICROS_U64: u64 = (FIXED_UPDATE_TIME_SECS_F64 * 1_000_000.0) as u64;
const FIXED_UPDATE_TIME_MICROS_U128: u128 = FIXED_UPDATE_TIME_MICROS_U64 as u128;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;
    let window = video_subsys
        .window("Pong", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut events = sdl_context.event_pump()?;

    let mut last_frame = Instant::now();
    let mut time_since_fixed_update = Duration::from_micros(0);

    let mut update_info = UpdateInfo {
        keys_pressed: HashSet::new(),
        actors: Vec::new(),
        elapsed: Duration::from_nanos(0),
        elapsed_sec_f64: 0.0,
    };

    let player = PlayerPaddle::new(&Position {
        x: 40,
        y: SCREEN_HEIGHT / 2,
    });
    update_info.actors.push(RefCell::new(Box::new(player)));

    let ball = Ball::new(&Position {
        x: SCREEN_WIDTH / 2,
        y: SCREEN_HEIGHT / 2,
    });
    update_info.actors.push(RefCell::new(Box::new(ball)));

    'main: loop {
        let now = Instant::now();
        update_info.elapsed = now.duration_since(last_frame);
        if update_info.elapsed.as_micros() >= TARGET_FRAMETIME_MICROS {
            last_frame = now;

            update_info.elapsed_sec_f64 = update_info.elapsed.as_micros() as f64 / 1_000_000.0;

            canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
            canvas.clear();

            for event in events.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'main,

                    Event::KeyDown {
                        keycode: Some(keycode),
                        ..
                    } => {
                        if keycode == Keycode::Escape {
                            break 'main;
                        }
                        update_info.keys_pressed.insert(keycode);
                    }

                    Event::KeyUp {
                        keycode: Some(keycode),
                        ..
                    } => {
                        if keycode == Keycode::Escape {
                            break 'main;
                        }
                        update_info.keys_pressed.remove(&keycode);
                    }

                    _ => {}
                }
            }

            let mut i = 0;
            while i < update_info.actors.len() {
                if let Option::Some(a) = update_info.actors.get(i) {
                    a.borrow_mut().update(&update_info);
                }
                i += 1;
            }

            time_since_fixed_update += update_info.elapsed;
            update_info.elapsed = Duration::from_micros(FIXED_UPDATE_TIME_MICROS_U64);
            update_info.elapsed_sec_f64 = FIXED_UPDATE_TIME_SECS_F64;
            while time_since_fixed_update.as_micros() > FIXED_UPDATE_TIME_MICROS_U128 {
                i = 0;
                while i < update_info.actors.len() {
                    if let Option::Some(a) = update_info.actors.get(i) {
                        a.borrow_mut().fixed_update(&update_info);
                    }
                    i += 1;
                }
                time_since_fixed_update -= update_info.elapsed;
            }

            i = 0;
            while i < update_info.actors.len() {
                if let Option::Some(a) = update_info.actors.get(i) {
                    a.borrow().draw(&mut canvas);
                }
                i += 1;
            }

            canvas.present();
        }
    }

    Ok(())
}
