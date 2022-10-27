use crate::actor::*;

pub struct Wall {
    position: Vec2,
    collider: RectangleDefinition,
}

impl Wall {
    pub fn new(position: &Vec2, width: f32, height: f32) -> Wall {
        Wall {
            position: *position,
            collider: RectangleDefinition {
                height,
                width,
            },
        }
    }
}

impl Actor for Wall {
    fn position(&self) -> &Vec2 {
        &self.position
    }

    fn set_position(&mut self, new_pos: &Vec2) {
        self.position = *new_pos;
    }

    fn update(&mut self, _info: &UpdateInfo) {}

    fn fixed_update(&mut self, _info: &UpdateInfo) {}

    fn draw(&self, _canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String> {
        Result::Ok(())
    }

    fn get_collider(&self) -> Option<Collider> {
        Option::Some(Collider {
            is_static: true,
            collider: ColliderType::Rectangle(self.collider),
        })
    }
}
