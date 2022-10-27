use std::{
    cell::RefCell,
    collections::HashSet,
    ops::{AddAssign, Mul, MulAssign, Sub},
    time::Duration,
};

use sdl2::{keyboard::Keycode, render::Canvas, video::Window};

#[derive(Copy, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn length_squared(self) -> f32 {
        self.x.powi(2) + self.y.powi(2)
    }

    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Copy, Clone)]
pub struct RectangleDefinition {
    pub height: f32,
    pub width: f32,
}

#[derive(Copy, Clone)]
pub enum ColliderType {
    Rectangle(RectangleDefinition),
    Circle { radius: f32 },
}

#[derive(Copy, Clone)]
pub enum ColliderBounds {
    Rectangle {
        up: f32,
        down: f32,
        left: f32,
        right: f32,
    },
    Circle {
        radius: f32,
        center: Vec2,
    },
}

impl ColliderBounds {
    pub fn from(c: &ColliderType, pos: &Vec2) -> ColliderBounds {
        match c {
            ColliderType::Circle { radius } => ColliderBounds::Circle {
                radius: *radius,
                center: *pos,
            },
            ColliderType::Rectangle(r) => {
                let half_height = r.height / 2.0;
                let half_width = r.width / 2.0;

                ColliderBounds::Rectangle {
                    up: pos.y - half_height,
                    down: pos.y + half_height,
                    left: pos.x - half_width,
                    right: pos.x + half_width,
                }
            }
        }
    }
}

pub struct Collider {
    pub collider: ColliderType,
    pub is_static: bool,
}

impl Collider {
    pub fn to_bounds(&self, pos: Vec2) -> ColliderBounds {
        ColliderBounds::from(&self.collider, &pos)
    }
}

pub struct UpdateInfo {
    pub keys_pressed: HashSet<Keycode>,
    pub elapsed: Duration,
    pub elapsed_sec_f32: f32,
    pub actors: Vec<RefCell<Box<dyn Actor>>>,
}

pub trait Actor {
    fn position(&self) -> &Vec2;
    fn set_position(&mut self, new_pos: &Vec2);
    fn update(&mut self, info: &UpdateInfo);
    fn fixed_update(&mut self, info: &UpdateInfo);
    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String>;
    fn get_collider(&self) -> Option<Collider>;
}
