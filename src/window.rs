use raylib::prelude::*;
use bevy_ecs::prelude::*;
use bevy_app::prelude::*;
use crate::rl;


pub(crate) fn init_window(app: &mut App) {
    app.add_message::<Fullscreen>();
    app.add_systems(Last, (
        fullscreen,
    ));
}


pub struct Window {
    size: (i32, i32)
}

#[derive(Message)]
pub struct Fullscreen(pub bool);


pub fn fullscreen(
    mut rl: ResMut<rl::Rl>,
    mut message: MessageReader<Fullscreen>,
) {
    for fullscreen in message.read() {
        if fullscreen.0 {
            let current_monitor = get_current_monitor();
            let width = get_monitor_width(current_monitor);
            let height = get_monitor_height(current_monitor);
            rl.set_window_size(width, height);
            if !rl.is_window_fullscreen() {
                rl.toggle_fullscreen();
            }
        } else {
            if rl.is_window_fullscreen() {
                rl.toggle_fullscreen();
            }
        }
    }
}