use raylib::prelude::*;
use shipyard::*;

use crate::Rl;


pub(super) fn start_window_systems(
    v_world: AllStoragesView,
    mut vm_rl: UniqueViewMut<Rl>,
) {
    v_world.add_unique(Fullscreen::new(&mut vm_rl));
    v_world.add_unique(Time(0.));
    v_world.add_unique(DeltaTime(0.));
    v_world.add_unique(BackgroundColor(
        crate::color::Color::BLACK,
    ));
    v_world.add_unique(Screen{
        width: 0.,
        height: 0.,
    });
}


#[derive(Unique)]
pub struct Fullscreen(
    pub bool,
    bool,
);

impl Fullscreen {
    fn new(vm_rl: &mut Rl) -> Self {
        let fullscreen = vm_rl.0.is_window_fullscreen();
        if fullscreen {
            app_size_with_monitor_size(vm_rl);
        }

        Self(
            fullscreen,
            fullscreen,
        )
    }
}

pub(super) fn pre_update_fullscreen(
    mut vm_rl: UniqueViewMut<Rl>,
    mut vm_fullscreen: UniqueViewMut<Fullscreen>,
) {
    let mut set_fullscreen = false;
    if vm_fullscreen.1 != vm_fullscreen.0 {
        if vm_fullscreen.0 {
            app_size_with_monitor_size(&mut vm_rl);
        }
        set_fullscreen = true;
        vm_fullscreen.1 = vm_fullscreen.0;
    }
    if vm_fullscreen.0 != vm_rl.0.is_window_fullscreen() {
        if set_fullscreen {
            vm_rl.0.toggle_fullscreen();
        } else {
            vm_fullscreen.0 = !vm_fullscreen.0;
            vm_fullscreen.1 = vm_fullscreen.0;
        }
    }
}

fn app_size_with_monitor_size(vm_rl: &mut Rl) {
    let width = get_monitor_width(0);
    let height = get_monitor_height(0);
    vm_rl.0.set_window_size(width, height);
}


#[derive(Unique)]
pub struct Screen {
    pub width: f32,
    pub height: f32,
}


#[derive(Unique)]
pub struct Time(
    pub f64,
);

#[derive(Unique)]
pub struct DeltaTime(
    pub f32,
);

pub(super) fn pre_update_date_and_time_and_screen(
    v_rl: UniqueView<Rl>,
    mut vm_time: UniqueViewMut<Time>,
    mut vm_delta_time: UniqueViewMut<DeltaTime>,
    mut vm_window_size: UniqueViewMut<Screen>,
) {
    vm_time.0 = v_rl.0.get_time();
    vm_delta_time.0 = v_rl.0.get_frame_time();

    vm_window_size.width = v_rl.0.get_screen_width() as f32;
    vm_window_size.height = v_rl.0.get_screen_height() as f32;
}


#[derive(Unique)]
pub struct BackgroundColor(
    pub crate::color::Color,
);