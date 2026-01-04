use raylib::ffi::Rectangle;
use raylib::prelude::{RaylibDraw, RaylibMode2DExt};
use raylib::{camera, color, prelude::RaylibDrawHandle};
use shipyard::*;
use glam::{Vec2, vec2};

use crate::camera::Camera2D;
use crate::transform::*;
use crate::color::*;


#[derive(Component)]
pub struct Circle(
    pub f32,
);

#[derive(Component)]
pub struct Rect(
    pub f32,
    pub f32,
);



pub(super) fn draw_shapes(mut d: RaylibDrawHandle, world: &World) {
    let visibles = world.borrow::<View<Visible>>().unwrap();
    let visibles = VisibleBuffer { visibles };

    let positions = world.borrow::<View<Position2D>>().unwrap();
    let rotations = world.borrow::<View<Rotation2D>>().unwrap();
    let scales = world.borrow::<View<Scale2D>>().unwrap();

    let mut camera = camera::Camera2D {
        offset: Vec2::ZERO,
        target: Vec2::ZERO,
        rotation: 0.,
        zoom: 1.,
    };
    {
        let cameras = world.borrow::<View<Camera2D>>().unwrap();
        for (entity, _) in cameras.iter().with_id() {
            if visibles.is_visible(entity) {
                if let Ok(position) = positions.get(entity) {
                    camera.offset = -position.0;
                }
                if let Ok(rotation) = rotations.get(entity) {
                    camera.rotation = rotation.0;
                }
                if let Ok(scale) = scales.get(entity) {
                    camera.zoom = 1. / scale.0;
                }
            }
        }
    }
    
    let colors = world.borrow::<View<Color>>().unwrap();

    let tsb = TSB {
        positions,
        rotations,
        scales,
        colors,
    };

    d.draw_mode2D(camera, |mut d2| {
        {
            let circles = world.borrow::<View<Circle>>().unwrap();
            for (entity, circle) in circles.iter().with_id() {
                if visibles.is_visible(entity) {
                    let (position, _, scale, color) = tsb.get_or_default(entity);
                    d2.draw_circle_v(position, circle.0 * scale, color);
                }
            }
        }
        {
            let rects: View<'_, Rect, track::Untracked> = world.borrow::<View<Rect>>().unwrap();
            for (entity, rect) in rects.iter().with_id() {
                if visibles.is_visible(entity) {
                    let (position, rotation, scale, color) = tsb.get_or_default(entity);
                    let rect = vec2(rect.0 * scale, rect.1 * scale);
                    d2.draw_rectangle_pro(
                        Rectangle {
                            x: position.x,
                            y: position.y,
                            width: rect.x,
                            height: rect.y,
                        },
                        rect / 2.,
                        rotation,
                        color);
                }
            }
        }
    });
}


struct VisibleBuffer<'a> {
    visibles: View<'a, Visible>,
}
impl<'a> VisibleBuffer<'a> {
    fn is_visible(&self, entity_id: EntityId) -> bool {
        if let Ok(visible) = self.visibles.get(entity_id) {
            return visible.0;
        } else {
            return true;
        }
    }
}

struct TSB<'a> {
    positions: View<'a, Position2D>,
    rotations: View<'a, Rotation2D>,
    scales: View<'a, Scale2D>,
    colors: View<'a, Color>,
}
impl<'a> TSB<'a> {
    fn get_or_default(&self, entity_id: EntityId) -> (Vec2, f32, f32, color::Color) {
        let position = if let Ok(position) = self.positions.get(entity_id) {
            position.0
        } else {
            Vec2::ZERO
        };

        let rotation = if let Ok(rotation) = self.rotations.get(entity_id) {
            rotation.0
        } else {
            0.
        };

        let scale = if let Ok(scale) = self.scales.get(entity_id) {
            scale.0
        } else {
            1.
        };

        let color = if let Ok(color) = self.colors.get(entity_id) {
            color.to_raylib_color()
        } else {
            color::Color::WHITE
        };

        (position, rotation, scale, color)
    }
}