use crate::actor::{ColliderBounds, Vec2};

pub fn collides(c1: ColliderBounds, c2: ColliderBounds) -> bool {
    match c1 {
        ColliderBounds::Circle { radius, center } => match c2 {
            ColliderBounds::Rectangle {
                up,
                down,
                left,
                right,
                center: _,
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
            center: _,
        } => match c2 {
            ColliderBounds::Rectangle {
                up: up2,
                down: down2,
                left: left2,
                right: right2,
                center: _,
            } => !(right < left2 || left > right2 || up > down2 || down < up2),
            ColliderBounds::Circle {
                radius: _,
                center: _,
            } => collides(c2, c1),
        },
    }
}

pub fn separation_vec(c1: ColliderBounds, c2: ColliderBounds) -> Vec2 {
    match c1 {
        ColliderBounds::Circle { radius, center } => match c2 {
            ColliderBounds::Rectangle {
                up,
                down,
                left,
                right,
                center: _,
            } => {
                let nearest = Vec2 {
                    x: center.x.clamp(left, right),
                    y: center.y.clamp(up, down),
                };

                center - nearest
            }
            ColliderBounds::Circle { radius, center } => unimplemented!(),
        },
        ColliderBounds::Rectangle {
            up,
            down,
            left,
            right,
            center,
        } => match c2 {
            ColliderBounds::Rectangle {
                up: up2,
                down: down2,
                left: left2,
                right: right2,
                center: center2,
            } => {
                let nearest = Vec2 {
                    x: center.x.clamp(left2, right2),
                    y: center.y.clamp(up2, down2),
                };
                let nearest2 = Vec2 {
                    x: center2.x.clamp(left, right),
                    y: center2.y.clamp(up, down),
                };

                return nearest - nearest2;
            }
            ColliderBounds::Circle { radius, center } => separation_vec(c2, c1),
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
                center: _,
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
            center,
        } => match c2 {
            ColliderBounds::Rectangle {
                up: up2,
                down: down2,
                left: left2,
                right: right2,
                center: center2,
            } => unimplemented!(),
            ColliderBounds::Circle {
                radius: _,
                center: _,
            } => separation(c2, c1),
        },
    }
}
