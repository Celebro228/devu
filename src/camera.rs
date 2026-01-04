use std::ops::{Deref, DerefMut};

use shipyard::*;
use glam::{Vec3, vec3};


#[derive(Component, Clone, Copy, Debug)]
pub struct Camera2D;


#[derive(Component, Clone, Copy, Debug)]
pub struct Camera3D {
    pub fov: f32,
    pub orthographic: bool,
}
impl Camera3D {
    pub fn new(fov: f32) -> Self {
        Self {
            fov,
            orthographic: false,
        }
    }
}
impl Default for Camera3D {
    fn default() -> Self {
        Self {
            fov: 60.,
            orthographic: false,
        }
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct Target3d(
    pub Vec3,
);
impl Target3d {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(vec3(x, y, z))
    }
}
impl Default for Target3d {
    fn default() -> Self {
        Self(Vec3::ZERO)
    }
}
impl Deref for Target3d {
    type Target = Vec3;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Target3d {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}