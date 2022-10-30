use sdl2::{keyboard::Keycode, pixels::Color, rect::Rect};

use crate::{actor::*, collision_helper};

const MOVE_SPEED: f32 = 150.0;

pub struct PlayerPaddle {
    position: Vec2,
    collider: RectangleDefinition,
    main_player: bool,
}

impl PlayerPaddle {
    pub fn new(position: &Vec2, main_player: bool) -> PlayerPaddle {
        PlayerPaddle {
            position: *position,
            collider: RectangleDefinition {
                width: 15.0,
                height: 50.0,
            },
            main_player,
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
        let mut moved = false;
        if (self.main_player && info.keys_pressed.contains(&Keycode::W))
            || (!self.main_player && info.keys_pressed.contains(&Keycode::Up))
        {
            self.position.y -= MOVE_SPEED * info.elapsed_sec_f32;
            moved = true;
        }
        if (self.main_player && info.keys_pressed.contains(&Keycode::S))
            || (!self.main_player && info.keys_pressed.contains(&Keycode::Down))
        {
            self.position.y += MOVE_SPEED * info.elapsed_sec_f32;
            moved = true;
        }

        if moved {
            let my_col = self
                .get_collider()
                .expect("ball did not return a collider for self");
            let mut my_bounds = my_col.to_bounds(self.position);

            let mut i = 0;
            while i < info.actors.len() {
                if let Option::Some(a) = info.actors.get(i) {
                    if let Result::Ok(actor) = a.try_borrow() {
                        if let Option::Some(col) = actor.get_collider() {
                            if col.is_static {
                                let bounds = col.to_bounds(*actor.position());
                                if collision_helper::collides(my_bounds, bounds) {
                                    let sep_vec =
                                        collision_helper::separation_vec(my_bounds, bounds);
                                    self.position.y += sep_vec.y;
                                    my_bounds = my_col.to_bounds(self.position);
                                }
                            }
                        }
                    }
                }
                i += 1;
            }
        }
    }

    fn fixed_update(&mut self, _info: &UpdateInfo) {}

    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String> {
        if let ColliderBounds::Rectangle {
            up,
            down: _,
            left,
            right: _,
            center: _,
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

    fn get_data(&self) -> Option<ActorData> {
        Option::None
    }
}
