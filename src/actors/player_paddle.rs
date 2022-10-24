use sdl2::{keyboard::Keycode, pixels::Color, rect::Rect};

use crate::actor::*;

const MOVE_SPEED: f64 = 150.0;

pub struct PlayerPaddle {
    position: Position,
    collider: RectangleDefinition,
}

impl PlayerPaddle {
    pub fn new(position: &Position) -> PlayerPaddle {
        PlayerPaddle {
            position: *position,
            collider: RectangleDefinition {
                width: 15,
                height: 50,
            },
        }
    }
}

impl Actor for PlayerPaddle {
    fn position(&self) -> &Position {
        &self.position
    }

    fn set_position(&mut self, new_pos: &Position) {
        self.position = *new_pos;
    }

    fn update(&mut self, info: &UpdateInfo) {
        if info.keys_pressed.contains(&Keycode::W) {
            self.position.y -=
                (MOVE_SPEED * (info.elapsed.as_micros() as f64 / 1_000_000.0)) as i16;
        }
        if info.keys_pressed.contains(&Keycode::S) {
            self.position.y +=
                (MOVE_SPEED * (info.elapsed.as_micros() as f64 / 1_000_000.0)) as i16;
        }
    }

    fn fixed_update(&mut self, _info: &UpdateInfo) {}

    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        if let ColliderBounds::Rectangle {
            up,
            down: _,
            left,
            right: _,
        } = ColliderBounds::from(&ColliderType::Rectangle(self.collider), &self.position)
        {
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            let _ = canvas.fill_rect(Rect::new(
                left.into(),
                up.into(),
                self.collider.width.try_into().unwrap(),
                self.collider.height.try_into().unwrap(),
            ));
        }
    }

    fn get_collider(&self) -> Option<Collider> {
        Option::Some(Collider {
            is_static: false,
            collider: ColliderType::Rectangle(self.collider),
        })
    }
}
