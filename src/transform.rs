use shipyard::*;
use glam::Vec2;


// TODO: Использовать в shapes и models
#[derive(Component)]
pub struct Depth(
    pub i32,
);

#[derive(Component)]
pub struct Visible(
    pub bool,
);

#[derive(Component)]
pub struct Position2D(
    pub Vec2,
);

#[derive(Component)]
pub struct Rotation2D(
    pub f32,
);

#[derive(Component)]
pub struct Scale2D(
    pub f32,
);