use raylib::camera;
use shipyard::*;
use glam::Vec2;


const DEFAULT_CAMERA2D: Camera2D = Camera2D::new();


#[derive(Component)]
pub struct Camera2D {
    raylib_camera: camera::Camera2D,
}

impl Camera2D {
    const fn new() -> Self {
        Self {
            raylib_camera: camera::Camera2D {
                offset: Vec2::ZERO,
                target: Vec2::ZERO,
                rotation: 0.,
                zoom: 0.,
            }
        }
    }
}