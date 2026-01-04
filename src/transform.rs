use std::ops::{Deref, DerefMut};

use shipyard::*;
use glam::{Vec2, Vec3, vec2, vec3};


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

// ===================================== 2D =======================================
#[derive(Component, Clone, Copy, Debug)]
pub struct Position2D(
    pub Vec2,
);
impl Position2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self(vec2(x, y))
    }
}
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

// ===================================== 3D =======================================
#[derive(Component, Clone, Copy, Debug)]
pub struct Position3D(
    pub Vec3,
);
impl Position3D {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(vec3(x, y, z))
    }
}
impl Default for Position3D {
    fn default() -> Self {
        Self(Vec3::ZERO)
    }
}
impl Deref for Position3D {
    type Target = Vec3;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Position3D {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct Rotation3D(
    pub Vec3,
);
impl Rotation3D {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(vec3(x, y, z))
    }
}
impl Default for Rotation3D {
    fn default() -> Self {
        Self(Vec3::ZERO)
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

#[derive(Component, Clone, Copy, Debug)]
pub struct Scale3D(
    pub Vec3,
);
impl Scale3D {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(vec3(x, y, z))
    }
}
impl Default for Scale3D {
    fn default() -> Self {
        Self(Vec3::ZERO)
    }
}
impl Deref for Scale3D {
    type Target = Vec3;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Scale3D {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}