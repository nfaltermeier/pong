use sdl2::{pixels::Color, render::TextureQuery, ttf::Font};

use crate::actor::*;

#[derive(Copy, Clone)]
pub struct ScoreboardData {
    pub left_score: u32,
    pub right_score: u32,
}

pub struct Scoreboard<'a> {
    position: Vec2,
    data: ScoreboardData,
    font: &'a Font<'a, 'a>,
}

impl<'a> Scoreboard<'a> {
    pub fn new(position: &Vec2, font: &'a Font<'a, 'a>) -> Scoreboard<'a> {
        Scoreboard {
            position: *position,
            data: ScoreboardData { left_score: 0, right_score: 0 },
            font,
        }
    }
}

impl<'a> Actor for Scoreboard<'a> {
    fn position(&self) -> &Vec2 {
        &self.position
    }

    fn set_position(&mut self, new_pos: &Vec2) {
        self.position = *new_pos;
    }

    fn update(&mut self, _info: &UpdateInfo) {}

    fn fixed_update(&mut self, _info: &UpdateInfo) {}

    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String> {
        let texture_creator = canvas.texture_creator();
        let surface = self
            .font
            .render(&format!("{} : {}", self.data.left_score, self.data.right_score))
            .blended(Color::RGB(255, 255, 255))
            .map_err(|e| e.to_string())?;
        let tex = texture_creator
            .create_texture_from_surface(surface)
            .map_err(|e| e.to_string())?;

        let TextureQuery { width, height, .. } = tex.query();
        let bounds = sdl2::rect::Rect::new(
            (self.position.x - width as f32 / 2.0).round() as i32,
            (self.position.y - height as f32 / 2.0).round() as i32,
            width,
            height,
        );

        canvas.copy(&tex, None, bounds)?;
        Result::Ok(())
    }

    fn get_collider(&self) -> Option<Collider> {
        Option::None
    }

    fn get_data(&self) -> Option<ActorData> {
        Option::Some(ActorData::Scoreboard(self.data))
    }

    fn set_data(&mut self, data: ActorData) {
        match data {
            ActorData::Scoreboard(s) => {
                self.data = s;
            }
            _ => {}
        }
    }
}
