use crate::actor::{ColliderBounds, Vec2};

pub fn collides(c1: ColliderBounds, c2: ColliderBounds) -> bool {
    match c1 {
        ColliderBounds::Circle { radius, center } => match c2 {
            ColliderBounds::Rectangle {
                up,
                down,
                left,
                right,
            } => {
                let nearest_x = center.x.clamp(left, right);
                let nearest_y = center.y.clamp(up, down);

                let separation = center
                    - Vec2 {
                        x: nearest_x,
                        y: nearest_y,
                    };
                radius.powi(2) >= separation.length_squared()
            }
            ColliderBounds::Circle { radius, center } => unimplemented!(),
        },
        ColliderBounds::Rectangle {
            up,
            down,
            left,
            right,
        } => match c2 {
            ColliderBounds::Rectangle {
                up,
                down,
                left,
                right,
            } => unimplemented!(),
            ColliderBounds::Circle { radius, center } => collides(c2, c1),
        },
    }
}

pub fn separation(c1: ColliderBounds, c2: ColliderBounds) -> f32 {
    match c1 {
        ColliderBounds::Circle { radius, center } => match c2 {
            ColliderBounds::Rectangle {
                up,
                down,
                left,
                right,
            } => {
                let nearest_x = center.x.clamp(left, right);
                let nearest_y = center.y.clamp(up, down);

                let separation = center
                    - Vec2 {
                        x: nearest_x,
                        y: nearest_y,
                    };
                separation.length() - radius
            }
            ColliderBounds::Circle { radius, center } => unimplemented!(),
        },
        ColliderBounds::Rectangle {
            up,
            down,
            left,
            right,
        } => match c2 {
            ColliderBounds::Rectangle {
                up,
                down,
                left,
                right,
            } => unimplemented!(),
            ColliderBounds::Circle { radius, center } => separation(c2, c1),
        },
    }
}
