use sdl2::{pixels::Color, rect::Point};

use crate::actor::*;
use crate::collision_helper;
use crate::math_helper;

use super::wall::WallType;

const INITIAL_SPEED: f32 = 125.0;

pub struct Ball {
    position: Vec2,
    initial_position: Vec2,
    velocity: Vec2,
    radius: f32,
}

impl Ball {
    pub fn new(position: &Vec2) -> Ball {
        Ball {
            position: *position,
            initial_position: *position,
            velocity: Ball::get_random_starting_velocity(),
            radius: 23.0,
        }
    }

    fn get_random_starting_velocity() -> Vec2 {
        let v = math_helper::get_point_on_unit_circle();
        Vec2 {
            x: v.0 * INITIAL_SPEED,
            y: v.1 * INITIAL_SPEED,
        }
    }

    fn update_scoreboard(&mut self, info: &UpdateInfo, wall: WallType) {
        let mut i = 0;
        while i < info.actors.len() {
            if let Option::Some(a) = info.actors.get(i) {
                if let Result::Ok(mut actor) = a.try_borrow_mut() {
                    if let Option::Some(data) = actor.get_data() {
                        match data {
                            ActorData::Scoreboard(mut s) => {
                                match wall {
                                    WallType::Left => {
                                        s.left_score += 1;
                                    }
                                    WallType::Right => {
                                        s.right_score += 1;
                                    }
                                    WallType::Regular => {}
                                }
                                actor.set_data(ActorData::Scoreboard(s));
                                return;
                            }
                            _ => {}
                        }
                    }
                }
            }
            i += 1;
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
        self.position += self.velocity * info.elapsed_sec_f32;

        let my_col = self
            .get_collider()
            .expect("ball did not return a collider for self");
        let mut my_bounds = my_col.to_bounds(self.position);

        let mut i = 0;
        while i < info.actors.len() {
            if let Option::Some(a) = info.actors.get(i) {
                if let Result::Ok(actor) = a.try_borrow() {
                    if let Option::Some(col) = actor.get_collider() {
                        let bounds = col.to_bounds(*actor.position());
                        let sep_vec = collision_helper::separation_vec(my_bounds, bounds);
                        let sep = sep_vec.length() - self.radius;

                        if sep < 0.0 {
                            if let Option::Some(d) = actor.get_data() {
                                if let ActorData::Wall(wd) = d {
                                    if matches!(wd, WallType::Left) || matches!(wd, WallType::Right)
                                    {
                                        self.position = self.initial_position;
                                        self.velocity = Ball::get_random_starting_velocity();
                                        self.update_scoreboard(info, wd);
                                        return;
                                    }
                                }
                            }

                            let speed = self.velocity.length();
                            let t = -sep / speed;

                            if sep_vec.x.abs() > sep_vec.y.abs() {
                                self.velocity.x *= -1.0;
                            } else {
                                self.velocity.y *= -1.0;
                            }
                            self.velocity *= 1.05;

                            self.position += self.velocity * t;
                            my_bounds = my_col.to_bounds(self.position);
                        }
                    }
                }
            }
            i += 1;
        }
    }

    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        draw_circle(
            canvas,
            self.position.x.round() as i32,
            self.position.y.round() as i32,
            self.radius.round() as i32,
        )?;
        Result::Ok(())
    }

    fn get_collider(&self) -> Option<Collider> {
        Option::Some(Collider {
            is_static: false,
            collider: ColliderType::Circle {
                radius: self.radius,
            },
        })
    }

    fn get_data(&self) -> Option<ActorData> {
        Option::None
    }

    fn set_data(&mut self, _data: ActorData) {}
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
        canvas.draw_line(
            Point::new(center_x + x, center_y + y),
            Point::new(center_x - x, center_y + y),
        )?;
        canvas.draw_line(
            Point::new(center_x + x, center_y - y),
            Point::new(center_x - x, center_y - y),
        )?;

        if error <= 0 {
            y += 1;
            error += ty;
            ty += 2;
        }

        if error > 0 {
            canvas.draw_line(
                Point::new(center_x + y, center_y + x),
                Point::new(center_x - y, center_y + x),
            )?;
            canvas.draw_line(
                Point::new(center_x + y, center_y - x),
                Point::new(center_x - y, center_y - x),
            )?;

            x -= 1;
            tx += 2;
            error += tx - diameter;
        }
    }

    Result::Ok(())
}
