use shipyard::*;
use glam::Vec2;

// TODO: Доделать в будущем
// use crate::color::Color;


// const DEFAULT_TRANSFORM2D: Transform2D = Transform2D::ZERO;
// const DEFAULT_SHAPES_COLOR: Color = Color::WHITE;


#[derive(Component)]
pub struct Transform2D {
    pub position: Vec2,
    pub rotation: f32,
    pub scale: Vec2,
}

impl Transform2D {
    pub const ZERO: Self = Self::new(
        Vec2::ZERO,
        0.,
        Vec2::ONE,
    );

    pub const fn new(position: Vec2, rotation: f32, scale: Vec2) -> Self {
        Self {
            position,
            rotation,
            scale,
        }
    }

    pub const fn position(position: Vec2) -> Self {
        Self {
            position,
            rotation: 0.,
            scale: Vec2::ONE,
        }
    }

    pub const fn rotation(rotation: f32) -> Self {
        Self {
            position: Vec2::ZERO,
            rotation,
            scale: Vec2::ONE,
        }
    }

    pub const fn scale(scale: Vec2) -> Self {
        Self {
            position: Vec2::ZERO,
            rotation: 0.,
            scale,
        }
    }
}

impl Default for Transform2D {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            rotation: 0.,
            scale: Vec2::ONE,
        }
    }
}