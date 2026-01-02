/*

Структура:
{
    Window,
    Audio,
    Models,
    Shapes,
    Math,
    Ecs,
    Input,
    Camera,
    Color,
    Transform,
    Prelude,
    Conf,
}

*/


use raylib::prelude::*;
use shipyard::{scheduler::AsLabel, *};


pub mod prelude;
pub mod conf;
pub mod ecs; // Re-export shipyard
pub mod math; // Re-export glam
pub mod window;
pub mod color;
pub mod shapes;


#[derive(Unique)]
struct Rl(RaylibHandle);


pub fn run<SV, SR, SW, S, UV, UR, UW, U>(
    title: &str,
    start: impl AsLabel<S> + FnOnce() -> SW + 'static + Clone + Sync + Send,
    update: impl AsLabel<U> + FnOnce() -> UW + 'static + Clone + Sync + Send,
)
where
    SW: IntoWorkload<SV, SR> + 'static,
    UW: IntoWorkload<UV, UR> + 'static,
{
    let conf = conf::Conf {
        title: title.to_string(),
        ..Default::default()
    };
    run_ex(conf, start, update);
}


pub fn run_ex<SV, SR, SW, S, UV, UR, UW, U>(
    conf: conf::Conf,
    start: impl AsLabel<S> + FnOnce() -> SW + 'static + Clone + Sync + Send,
    update: impl AsLabel<U> + FnOnce() -> UW + 'static + Clone + Sync + Send,
)
where
    SW: IntoWorkload<SV, SR> + 'static,
    UW: IntoWorkload<UV, UR> + 'static,
{
    let mut builder = init();
    builder.title(&conf.title);
    builder.size(conf.size.0, conf.size.1);
    if conf.resizable {
        builder.resizable();
    }
    if conf.msaa_4x {
        builder.msaa_4x();
    }
    if conf.vsync {
        builder.vsync();
    }
    if !conf.logging {
        builder.log_level(TraceLogLevel::LOG_NONE);
    }
    let (rl, thread) = builder.build();

    // Workloads
    let start = move || -> Workload {
        (
            start_system(),
            pre_update_system(),
            start.clone()(),
        ).into_workload()
    };
    let update = move || -> Workload {
        (
            pre_update_system(),
            update.clone()(),
            post_update_system(),
        ).into_workload()
    };

    // World
    let world = World::new();
    world.add_unique(Rl(rl));
    world.add_workload(start.clone());
    world.add_workload(update.clone());
    
    world.run_workload(start).unwrap();

    while !world.get_unique::<&Rl>().unwrap().0.window_should_close() {
        world.run_workload(update.clone()).unwrap();

        world.get_unique::<&mut Rl>().unwrap().0.draw(&thread, |mut d| {
            d.clear_background(Color::RAYWHITE);
        });
    }
}

// TODO: Заполнить сущности
fn start_system() -> Workload {
    (
        window::start_window_systems,
    ).into_workload()
}

fn pre_update_system() -> Workload {
    (
        window::pre_update_window,
    ).into_workload()
}

fn post_update_system() -> Workload {
    (
        || {}
    ).into_workload()
}