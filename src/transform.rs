use shipyard::*;
use glam::Vec2;


// TODO: Использовать либо удалить
// #[derive(Component)]
// pub struct Depth(
//     pub i32,
// );

#[derive(Component)]
pub struct Visible(
    pub bool,
);

impl Default for Visible {
    fn default() -> Self {
        Self(true)
    }
}

#[derive(Component)]
pub struct Position2D(
    pub Vec2,
);

impl Default for Position2D {
    fn default() -> Self {
        Self(Vec2::ZERO)
    }
}

#[derive(Component)]
pub struct Rotation2D(
    pub f32,
);

impl Default for Rotation2D {
    fn default() -> Self {
        Self(0.)
    }
}

#[derive(Component)]
pub struct Scale2D(
    pub f32,
);

impl Default for Scale2D {
    fn default() -> Self {
        Self(1.)
    }
}