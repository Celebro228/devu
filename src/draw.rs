use bevy_ecs::{schedule::ScheduleLabel, system::{NonSend, ResMut}};
use bevy_app::prelude::*;
use raylib::prelude::*;
use crate::rl;


pub fn init_draw(app: &mut App) {
    app.init_schedule(Draw);
    app.add_systems(Draw, draw);
}


#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct Draw;


fn draw(
    mut rl: ResMut<rl::Rl>,
    thread: NonSend<rl::Thread>,
) {
    let mut d = rl.begin_drawing(&thread);

    d.clear_background(Color::RAYWHITE);
}