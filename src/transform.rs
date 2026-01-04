use std::ops::{Deref, DerefMut};

use shipyard::*;
use glam::Vec2;


// TODO: Использовать либо удалить
// #[derive(Component)]
// pub struct Depth(
//     pub i32,
// );

#[derive(Component, Clone, Copy, Debug)]
pub struct Visible(
    pub bool,
);
impl Default for Visible {
    fn default() -> Self {
        Self(true)
    }
}
impl Deref for Visible {
    type Target = bool;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Visible {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct Position2D(
    pub Vec2,
);
impl Default for Position2D {
    fn default() -> Self {
        Self(Vec2::ZERO)
    }
}
impl Deref for Position2D {
    type Target = Vec2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Position2D {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct Rotation2D(
    pub f32,
);
impl Default for Rotation2D {
    fn default() -> Self {
        Self(0.)
    }
}
impl Deref for Rotation2D {
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Rotation2D {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct Scale2D(
    pub f32,
);
impl Default for Scale2D {
    fn default() -> Self {
        Self(1.)
    }
}
impl Deref for Scale2D {
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Scale2D {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}