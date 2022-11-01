extern crate rand;
extern crate sdl2;

use actor::{UpdateInfo, Vec2};
use actors::ball::Ball;
use actors::player_paddle::PlayerPaddle;
use actors::scoreboard::Scoreboard;
use actors::wall::{Wall, WallType};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels;

use std::cell::RefCell;
use std::collections::HashSet;
use std::time::{Duration, Instant};

mod actor;
mod actors;
mod collision_helper;
mod math_helper;

const SCREEN_WIDTH: i16 = 800;
const SCREEN_HEIGHT: i16 = 600;

const TARGET_FRAMERATE: f32 = 60.0;
const TARGET_FRAMETIME_MICROS: u128 = (1.0 / TARGET_FRAMERATE * 1_000_000.0) as u128;

const FIXED_UPDATE_RATE: f32 = 60.0;
const FIXED_UPDATE_TIME_SECS_F32: f32 = 1.0 / FIXED_UPDATE_RATE;
const FIXED_UPDATE_TIME_MICROS_U64: u64 = (FIXED_UPDATE_TIME_SECS_F32 * 1_000_000.0) as u64;
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

    let ttf_context = sdl2::ttf::init().expect("f");
    let font = ttf_context
        .load_font("fonts/roboto_mono/RobotoMono-VariableFont_wght.ttf", 40)
        .expect(
            "Failed to load font at fonts/roboto_mono/RobotoMono-VariableFont_wght.ttf",
        );

    let mut events = sdl_context.event_pump()?;

    let mut last_frame = Instant::now();
    let mut time_since_fixed_update = Duration::from_micros(0);

    let mut update_info = UpdateInfo {
        keys_pressed: HashSet::new(),
        actors: Vec::new(),
        elapsed: Duration::from_nanos(0),
        elapsed_sec_f32: 0.0,
    };

    let half_width = SCREEN_WIDTH as f32 / 2.0;
    let half_height = SCREEN_HEIGHT as f32 / 2.0;

    let player = PlayerPaddle::new(
        &Vec2 {
            x: 40.0,
            y: half_height,
        },
        true,
    );
    update_info.actors.push(RefCell::new(Box::new(player)));

    let player = PlayerPaddle::new(
        &Vec2 {
            x: SCREEN_WIDTH as f32 - 40.0,
            y: half_height,
        },
        false,
    );
    update_info.actors.push(RefCell::new(Box::new(player)));

    let ball = Ball::new(&Vec2 {
        x: half_width,
        y: half_height,
    });
    update_info.actors.push(RefCell::new(Box::new(ball)));

    let wall = Wall::new(
        &Vec2 {
            x: half_width,
            y: -half_height,
        },
        SCREEN_WIDTH as f32,
        SCREEN_HEIGHT as f32,
        WallType::Regular,
    );
    update_info.actors.push(RefCell::new(Box::new(wall)));

    let wall = Wall::new(
        &Vec2 {
            x: half_width,
            y: 3.0 * half_height,
        },
        SCREEN_WIDTH as f32,
        SCREEN_HEIGHT as f32,
        WallType::Regular,
    );
    update_info.actors.push(RefCell::new(Box::new(wall)));

    let wall = Wall::new(
        &Vec2 {
            x: -half_width,
            y: half_height,
        },
        SCREEN_WIDTH as f32,
        SCREEN_HEIGHT as f32,
        WallType::Left,
    );
    update_info.actors.push(RefCell::new(Box::new(wall)));

    let wall = Wall::new(
        &Vec2 {
            x: 3.0 * half_width,
            y: half_height,
        },
        SCREEN_WIDTH as f32,
        SCREEN_HEIGHT as f32,
        WallType::Right,
    );
    update_info.actors.push(RefCell::new(Box::new(wall)));

    let scoreboard = Scoreboard::new(
        &Vec2 {
            x: half_width,
            y: 50.0,
        },
        &font,
    );
    update_info.actors.push(RefCell::new(Box::new(scoreboard)));

    'main: loop {
        let now = Instant::now();
        update_info.elapsed = now.duration_since(last_frame);
        if update_info.elapsed.as_micros() >= TARGET_FRAMETIME_MICROS {
            last_frame = now;

            update_info.elapsed_sec_f32 = update_info.elapsed.as_micros() as f32 / 1_000_000.0;

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
            update_info.elapsed_sec_f32 = FIXED_UPDATE_TIME_SECS_F32;
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
                    let _ = a.borrow().draw(&mut canvas);
                }
                i += 1;
            }

            canvas.present();
        }
    }

    Ok(())
}
