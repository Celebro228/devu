use devu::prelude::*;


fn main() {
    devu::run_ex(
        Conf {
            title: "Window Settings".to_string(),
            size: (1280, 720),
            resizable: true,
            msaa_4x: false,
            vsync: false,
            logging: true,
        },
        start,
        update,
    )
}


fn start() -> Workload {
    (
        fullscreen,
    ).into_workload()
}

fn fullscreen(mut vm_window: UniqueViewMut<Window>) {
    vm_window.fullscreen = true;
}


fn update() -> Workload {
    ( || { } ).into_workload()
}