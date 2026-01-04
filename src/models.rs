use raylib::ffi::{CameraProjection};
use raylib::prelude::{RaylibDraw3D, RaylibMode3DExt};
use raylib::{camera, color, prelude::RaylibDrawHandle};
use shipyard::*;
use glam::{Vec3, vec3};

use crate::camera::{Camera3D, Target3d};
use crate::transform::*;
use crate::color::*;


#[derive(Component, Clone, Copy, Debug)]
pub struct Sphere(
    pub f32,
);

#[derive(Component, Clone, Copy, Debug)]
pub struct Cube(
    pub f32,
    pub f32,
    pub f32,
);



pub(super) fn draw_models(d: &mut RaylibDrawHandle, world: &World) {
    let visibles = world.borrow::<View<Visible>>().unwrap();
    let visibles = VisibleBuffer { visibles };

    let positions = world.borrow::<View<Position3D>>().unwrap();
    let rotations = world.borrow::<View<Rotation3D>>().unwrap();
    let scales = world.borrow::<View<Scale3D>>().unwrap();

    let mut camera = camera::Camera3D {
        position: Vec3::ZERO,
        target: Vec3::new(0., 0., 1.),
        up: Vec3::new(0., 1., 0.),
        fovy: Camera3D::default().fov,
        projection: CameraProjection::CAMERA_PERSPECTIVE,
    };
    {
        let cameras = world.borrow::<View<Camera3D>>().unwrap();
        let targets = world.borrow::<View<Target3d>>().unwrap();
        for (entity, e_camera) in cameras.iter().with_id() {
            if visibles.is_visible(entity) {
                if e_camera.orthographic {
                    camera.projection = CameraProjection::CAMERA_ORTHOGRAPHIC;
                } else {
                    camera.projection = CameraProjection::CAMERA_PERSPECTIVE;
                }
                camera.fovy = e_camera.fov;

                if let Ok(position) = positions.get(entity) {
                    camera.position = position.0;
                }
                if let Ok(rotation) = rotations.get(entity) {
                    let (pitch, yaw) = (rotation.x, rotation.y);

                    let target = Vec3::new(
                        yaw.cos() * pitch.cos(),
                        pitch.sin(),
                        yaw.sin() * pitch.cos(),
                    )
                    .normalize();

                    camera.target = camera.position + target;
                } else if let Ok(target) = targets.get(entity) {
                    camera.target = target.0;
                }
            }
        }
    }
    
    let colors = world.borrow::<View<Color>>().unwrap();

    let tsb = TMB {
        positions,
        rotations,
        scales,
        colors,
    };

    d.draw_mode3D(camera, |mut d2| {
        {
            let spheres = world.borrow::<View<Sphere>>().unwrap();
            for (entity, sphere) in spheres.iter().with_id() {
                if visibles.is_visible(entity) {
                    let (position, _, _, color) = tsb.get_or_default(entity);
                    d2.draw_sphere(position, sphere.0, color);
                }
            }
        }
        {
            let cubes = world.borrow::<View<Cube>>().unwrap();
            for (entity, cube) in cubes.iter().with_id() {
                if visibles.is_visible(entity) {
                    let (position, _, scale, color) = tsb.get_or_default(entity);
                    let cube = vec3(cube.0 * scale.x, cube.1 * scale.y, cube.2 * scale.z);
                    d2.draw_cube_v(position, cube, color);
                }
            }
        }
    });
}


struct TMB<'a> {
    positions: View<'a, Position3D>,
    rotations: View<'a, Rotation3D>,
    scales: View<'a, Scale3D>,
    colors: View<'a, Color>,
}
impl<'a> TMB<'a> {
    fn get_or_default(&self, entity_id: EntityId) -> (Vec3, Vec3, Vec3, color::Color) {
        let position = if let Ok(position) = self.positions.get(entity_id) {
            position.0
        } else {
            Vec3::ZERO
        };
        let rotation = if let Ok(rotation) = self.rotations.get(entity_id) {
            rotation.0
        } else {
            Vec3::ZERO
        };
        let scale = if let Ok(scale) = self.scales.get(entity_id) {
            scale.0
        } else {
            Vec3::ONE
        };
        let color = if let Ok(color) = self.colors.get(entity_id) {
            color.to_raylib_color()
        } else {
            color::Color::WHITE
        };
        (position, rotation, scale, color)
    }
}