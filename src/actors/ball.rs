use sdl2::{pixels::Color, rect::Point};

use crate::actor::*;
use crate::math_helper;

const INITIAL_SPEED: f32 = 100.0;

pub struct Ball {
    position: Vec2,
    velocity: Vec2,
    radius: f32,
}

impl Ball {
    pub fn new(position: &Vec2) -> Ball {
        let v = math_helper::get_point_on_unit_circle();
        println!("{:?}", v);
        println!("{}", (v.0 * INITIAL_SPEED) as i16);
        println!("{}", (v.1 * INITIAL_SPEED) as i16);
        Ball {
            position: *position,
            velocity: Vec2 { x: v.0 * INITIAL_SPEED, y: v.1 * INITIAL_SPEED },
            radius: 23.0,
        }
    }
}

impl Actor for Ball {
    fn position(&self) -> &Vec2 {
        &self.position
    }

    fn set_position(&mut self, new_pos: &Vec2) {
        self.position = *new_pos;
    }

    fn update(&mut self, _info: &UpdateInfo) {}

    fn fixed_update(&mut self, info: &UpdateInfo) {
        self.position += self.velocity * (info.elapsed.as_micros() as f32 / 1_000_000.0);
    }

    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        let _ = draw_circle(
            canvas,
            self.position.x.round() as i32,
            self.position.y.round() as i32,
            self.radius.round() as i32,
        );
    }

    fn get_collider(&self) -> Option<Collider> {
        Option::Some(Collider {
            is_static: false,
            collider: ColliderType::Circle {
                radius: self.radius,
            },
        })
    }
}

// Midpoint circle algorithm, adapted from https://stackoverflow.com/a/48291620, modified to draw a filled circle
fn draw_circle(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    center_x: i32,
    center_y: i32,
    radius: i32,
) -> Result<(), String> {
    let diameter = radius * 2;

    let mut x = radius - 1;
    let mut y = 0;
    let mut tx = 1;
    let mut ty = 1;
    let mut error = tx - diameter;

    while x >= y {
        canvas.draw_line(Point::new(center_x + x, center_y + y), Point::new(center_x - x, center_y + y))?;
        canvas.draw_line(Point::new(center_x + x, center_y - y), Point::new(center_x - x, center_y - y))?;

        if error <= 0 {
            y += 1;
            error += ty;
            ty += 2;
        }

        if error > 0 {
            canvas.draw_line(Point::new(center_x + y, center_y + x), Point::new(center_x - y, center_y + x))?;
            canvas.draw_line(Point::new(center_x + y, center_y - x), Point::new(center_x - y, center_y - x))?;

            x -= 1;
            tx += 2;
            error += tx - diameter;
        }
    }

    Result::Ok(())
}
