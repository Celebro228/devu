use bevy_ecs::component::Component;
use num_traits::AsPrimitive;
use glam::{Vec3, vec3};
use std::ops::{Deref, DerefMut};


#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Position(pub Vec3);
impl Position {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(vec3(x, y, z))
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

#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Rotation2D(pub f32);
impl Rotation2D {
    pub fn new(r: f32) -> Self {
        Self(r)
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

#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Rotation3D(
    pub Vec3,
);
impl Rotation3D {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(vec3(x, y, z))
    }
}
impl Deref for Rotation3D {
    type Target = Vec3;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Rotation3D {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


#[inline(always)]
pub fn position<X, Y, Z>(x: X, y: Y, z: Z) -> Position
where
    X: AsPrimitive<f32>,
    Y: AsPrimitive<f32>,
    Z: AsPrimitive<f32>,
{
    Position(vec3(x.as_(), y.as_(), z.as_()))
}

#[inline(always)]
pub fn rotation2d<R: AsPrimitive<f32>>(r: R) -> Rotation2D {
    Rotation2D(r.as_())
}

#[inline(always)]
pub fn rotation3d<X, Y, Z>(x: X, y: Y, z: Z) -> Rotation3D
where
    X: AsPrimitive<f32>,
    Y: AsPrimitive<f32>,
    Z: AsPrimitive<f32>,
{
    Rotation3D(vec3(x.as_(), y.as_(), z.as_()))
}