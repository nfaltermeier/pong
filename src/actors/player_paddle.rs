use sdl2::{keyboard::Keycode, pixels::Color, rect::Rect};

use crate::actor::*;

const MOVE_SPEED: f32 = 150.0;

pub struct PlayerPaddle {
    position: Vec2,
    collider: RectangleDefinition,
}

impl PlayerPaddle {
    pub fn new(position: &Vec2) -> PlayerPaddle {
        PlayerPaddle {
            position: *position,
            collider: RectangleDefinition {
                width: 15.0,
                height: 50.0,
            },
        }
    }
}

impl Actor for PlayerPaddle {
    fn position(&self) -> &Vec2 {
        &self.position
    }

    fn set_position(&mut self, new_pos: &Vec2) {
        self.position = *new_pos;
    }

    fn update(&mut self, info: &UpdateInfo) {
        if info.keys_pressed.contains(&Keycode::W) {
            self.position.y -= MOVE_SPEED * info.elapsed_sec_f32;
        }
        if info.keys_pressed.contains(&Keycode::S) {
            self.position.y += MOVE_SPEED * info.elapsed_sec_f32;
        }
    }

    fn fixed_update(&mut self, _info: &UpdateInfo) {}

    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String> {
        if let ColliderBounds::Rectangle {
            up,
            down: _,
            left,
            right: _,
        } = ColliderBounds::from(&ColliderType::Rectangle(self.collider), &self.position)
        {
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            canvas.fill_rect(Rect::new(
                left.round() as i32,
                up.round() as i32,
                self.collider.width.round() as u32,
                self.collider.height.round() as u32,
            ))?;
            Result::Ok(())
        } else {
            Result::Err("PlayerPaddle: self.collider is not a Rectangle".to_string())
        }
    }

    fn get_collider(&self) -> Option<Collider> {
        Option::Some(Collider {
            is_static: false,
            collider: ColliderType::Rectangle(self.collider),
        })
    }
}
