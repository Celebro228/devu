use bevy_ecs::component::Component;
use num_traits::AsPrimitive;
use glam::{Vec3, vec3};
use std::ops::{Deref, DerefMut};


#[derive(Component, Clone, Copy, Debug)]
pub struct Position(
    pub Vec3,
);
impl Position {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(vec3(x, y, z))
    }
}
impl Default for Position {
    fn default() -> Self {
        Self(Vec3::ZERO)
    }
}
impl Deref for Position {
    type Target = Vec3;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Position {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[inline]
pub fn position<X, Y, Z>(x: X, y: Y, z: Z) -> Position
where
    X: AsPrimitive<f32>,
    Y: AsPrimitive<f32>,
    Z: AsPrimitive<f32>,
{
    Position(vec3(x.as_(), y.as_(), z.as_()))
}