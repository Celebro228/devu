use raylib::prelude::*;
use bevy_ecs::prelude::*;
use bevy_app::prelude::*;
use crate::rl;


pub(crate) fn init_window(app: &mut App) {
    app.add_message::<Fullscreen>();
    app.insert_resource(Window {
        size: (800, 400),
        fullscreen: false,
    });
    app.insert_resource(Time {
        time: 0.1,
        delta: 0.1,
    });
    app.add_systems(First, (
        window_update,
        time_update,
    ));
    app.add_systems(Last, (
        fullscreen,
    ));
}


#[derive(Message)]
pub struct Fullscreen(pub bool);

#[derive(Resource)]
pub struct Time {
    time: f64,
    delta: f32,
}
impl Time {
    pub fn time(&self) -> f64 {
        self.time
    }
    pub fn delta(&self) -> f32 {
        self.delta
    }
}

#[derive(Resource)]
pub struct Window {
    size: (i32, i32),
    fullscreen: bool,
}
impl Window {
    pub fn fullscreen(&self) -> bool {
        self.fullscreen
    }
    pub fn size(&self) -> (i32, i32) {
        self.size
    }
    pub fn width(&self) -> i32 {
        self.size.0
    }
    pub fn height(&self) -> i32 {
        self.size.1
    }
}


fn window_update(
    rl: Res<rl::Rl>,
    mut window: ResMut<Window>,
) {
    window.size = (rl.get_screen_width(), rl.get_screen_height());
    window.fullscreen = rl.is_window_fullscreen();
}

fn time_update(
    rl: Res<rl::Rl>,
    mut time: ResMut<Time>,
) {
    time.time = rl.get_time();
    time.delta = rl.get_frame_time();
}

fn fullscreen(
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