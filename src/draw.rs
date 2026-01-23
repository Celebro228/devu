use bevy_ecs::{schedule::ScheduleLabel, prelude::*};
use bevy_app::prelude::*;
use raylib::prelude::*;
use glam::{Vec2, vec2};
use crate::{
    rl,
    transform,
    color,
    window,
    camera,
    shapes::Shape,
};


pub fn init_draw(app: &mut App) {
    app.init_schedule(Draw);
    app.add_systems(Draw, draw);
}


#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct Draw;


fn draw(
    mut rl: ResMut<rl::Rl>,
    thread: NonSend<rl::Thread>,

    window: Res<window::Window>,

    colors: Query<&color::Color>,
    positions: Query<&transform::Position>,
    rotations2d: Query<&transform::Rotation2D>,
    cameras2d: Query<(Entity, &camera::Camera2D)>,
    shapes: Query<(Entity, &Shape)>,
) {
    let mut d = rl.begin_drawing(&thread);
    d.clear_background(Color::RAYWHITE);


    let (width, height) = window.size();


    let default_color = color::DARKBROWN;
    let default_position = transform::position(0, 0, 0);
    let default_rotation2d = transform::rotation2d(0);


    let mut camera2d = Camera2D {
        offset: vec2(width as f32 / 2.,  height as f32 / 2.),
        target: Vec2::ZERO,
        rotation: 0.,
        zoom: 1.,
    };
    for (entity, _) in cameras2d {
        let position = positions.get(entity).unwrap_or(&default_position).0;
        let rotation = rotations2d.get(entity).unwrap_or(&default_rotation2d).0;

        camera2d.target = vec2(position.x, position.y);
        camera2d.rotation = rotation;
    }

    d.draw_mode2D(camera2d, |mut d2| {
        // 2d sort by pos.z
        let mut shapes_sorted: Vec<(Entity, &Shape)> = shapes.iter().collect();
        shapes_sorted.sort_by_key(|(entity, _)| {
            positions.get(*entity).unwrap_or(&default_position).0.z as isize
        });
        
        // 2d shape draw
        for (entity, shape) in shapes_sorted {
            let color = colors.get(entity).unwrap_or(&default_color).0;
            let position = positions.get(entity).unwrap_or(&default_position).0;
            let position = vec2(position.x, position.y);
            let rotation = rotations2d.get(entity).unwrap_or(&default_rotation2d).0;

            match shape {
                Shape::Circle(r) => d2.draw_circle_v(
                    position,
                    *r,
                    color,
                ),
                Shape::Rect(w, h) => d2.draw_rectangle_pro(
                    Rectangle::new(position.x, position.y, *w, *h),
                    vec2(w / 2., h / 2.),
                    rotation,
                    color,
                ),
                Shape::Line(start_pos, end_pos, thick) => d2.draw_line_ex(
                    start_pos + position,
                    end_pos + position,
                    *thick,
                    color
                ),
            }
        }
    });
}