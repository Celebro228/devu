use devu::prelude::*;


fn main() {
    devu::run_ex(
        Conf {
            title: "Window Settings".to_string(),
            size: (1280, 720),
            resizable: true,
            msaa_4x: false,
            vsync: false,
            logging: false,
        },
        start,
        update,
    )
}


fn start() -> Workload {
    (
        fullscreen,
        background_color,
    ).into_workload()
}

fn fullscreen(mut vm_fullscreen: UniqueViewMut<Fullscreen>) {
    **vm_fullscreen = true;
}

fn background_color(mut vm_bg_color: UniqueViewMut<BackgroundColor>) {
    **vm_bg_color = Color::YELLOW;
}


fn update() -> Workload {
    ( || { } ).into_workload()
}