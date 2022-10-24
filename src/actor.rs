use std::{
    cell::RefCell,
    collections::HashSet,
    ops::{AddAssign, Mul},
    time::Duration,
};

use sdl2::{keyboard::Keycode, render::Canvas, video::Window};

#[derive(Copy, Clone)]
pub struct Position {
    pub x: i16,
    pub y: i16,
}

impl Mul<f64> for Position {
    type Output = Position;

    fn mul(mut self, rhs: f64) -> Self::Output {
        self.x = (self.x as f64 * rhs) as i16;
        self.y = (self.y as f64 * rhs) as i16;
        self
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[derive(Copy, Clone)]
pub struct RectangleDefinition {
    pub height: i16,
    pub width: i16,
}

#[derive(Copy, Clone)]
pub enum ColliderType {
    Rectangle(RectangleDefinition),
    Circle { radius: i16 },
}

#[derive(Copy, Clone)]
pub enum ColliderBounds {
    Rectangle {
        up: i16,
        down: i16,
        left: i16,
        right: i16,
    },
    Circle {
        radius: i16,
        center: Position,
    },
}

impl ColliderBounds {
    pub fn from(c: &ColliderType, pos: &Position) -> ColliderBounds {
        match c {
            ColliderType::Circle { radius } => ColliderBounds::Circle {
                radius: *radius,
                center: *pos,
            },
            ColliderType::Rectangle(r) => {
                let half_height = r.height / 2;
                let half_width = r.width / 2;

                ColliderBounds::Rectangle {
                    up: pos.y + half_height,
                    down: pos.y - half_height,
                    left: pos.x + half_width,
                    right: pos.x - half_width,
                }
            }
        }
    }
}

pub struct Collider {
    pub collider: ColliderType,
    pub is_static: bool,
}

pub struct UpdateInfo {
    pub keys_pressed: HashSet<Keycode>,
    pub elapsed: Duration,
    pub elapsed_sec_f64: f64,
    pub actors: Vec<RefCell<Box<dyn Actor>>>,
}

pub trait Actor {
    fn position(&self) -> &Position;
    fn set_position(&mut self, new_pos: &Position);
    fn update(&mut self, info: &UpdateInfo);
    fn fixed_update(&mut self, info: &UpdateInfo);
    fn draw(&self, canvas: &mut Canvas<Window>);
    fn get_collider(&self) -> Option<Collider>;
}
