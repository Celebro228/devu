use raylib::prelude::*;
use shipyard::*;

use crate::Rl;


pub(super) fn start_window_systems(
    v_world: AllStoragesView,
    mut vm_rl: UniqueViewMut<Rl>,
) {
    v_world.add_unique(Window::new(&mut vm_rl));
    v_world.add_unique(Time::new(&vm_rl));
}


struct WindowBuffer {
    fullscreen: bool,
}

#[derive(Unique)]
pub struct Window {
    window_buffer: WindowBuffer,
    pub fullscreen: bool,
}

impl Window {
    fn new(vm_rl: &mut Rl) -> Self {
        let fullscreen = vm_rl.0.is_window_fullscreen();
        if fullscreen {
            app_size_with_monitor_size(vm_rl);
        }

        Self {
            window_buffer: WindowBuffer { fullscreen, },
            fullscreen,
        }
    }
}

pub(super) fn pre_update_window(
    mut vm_rl: UniqueViewMut<Rl>,
    mut vm_window: UniqueViewMut<Window>,
) {
    let mut set_fullscreen = false;
    if vm_window.window_buffer.fullscreen != vm_window.fullscreen {
        if vm_window.fullscreen {
            app_size_with_monitor_size(&mut vm_rl);
        }
        set_fullscreen = true;
        vm_window.window_buffer.fullscreen = vm_window.fullscreen;
    }
    if vm_window.fullscreen != vm_rl.0.is_window_fullscreen() {
        if set_fullscreen {
            vm_rl.0.toggle_fullscreen();
        } else {
            vm_window.fullscreen = !vm_window.fullscreen;
            vm_window.window_buffer.fullscreen = vm_window.fullscreen;
        }
    }
}

fn app_size_with_monitor_size (vm_rl: &mut Rl) {
    let width = get_monitor_width(0);
    let height = get_monitor_height(0);
    vm_rl.0.set_window_size(width, height);
}


#[derive(Unique)]
pub struct Time {
    time: f64,
    delta: f32,
}

impl Time {
    fn new(v_rl: &Rl) -> Self {
        let time = v_rl.0.get_time();
        let delta = v_rl.0.get_frame_time();

        Self {
            time,
            delta,
        }
    }

    pub fn get_time(&self) -> f64 {
        self.time
    }

    pub fn get_delta(&self) -> f32 {
        self.delta
    }
}

pub(super) fn pre_update_time(
    v_rl: UniqueView<Rl>,
    mut vm_time: UniqueViewMut<Time>,
) {
    vm_time.time = v_rl.0.get_time();
    vm_time.delta = v_rl.0.get_frame_time();
}