use bevy_ecs::{schedule::ScheduleLabel, prelude::*};
use bevy_app::prelude::*;
use raylib::prelude::*;
use glam::vec2;
use crate::{
    rl,
    transform,
    color,
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

    colors: Query<&color::Color>,
    positions: Query<&transform::Position>,
    shapes: Query<(Entity, &Shape)>,
) {
    let mut d = rl.begin_drawing(&thread);
    d.clear_background(Color::RAYWHITE);

    let default_position = transform::position(0, 0, 0);
    let default_color = color::DARKBROWN;

    // 2d sort by pos.z
    let mut shapes_sorted: Vec<(Entity, &Shape)> = shapes.iter().collect();
    shapes_sorted.sort_by_key(|(entity, _)| {
        positions.get(*entity).unwrap_or(&default_position).0.z as isize
    });
    
    // 2d shape draw
    for (entity, shape) in shapes_sorted {
        let position = positions.get(entity).unwrap_or(&default_position);
        let color = colors.get(entity).unwrap_or(&default_color);

        match shape {
            Shape::Circle(r) => {
                d.draw_circle_v(vec2(position.x, position.y), *r, color.0);
            }
        }
    }
}