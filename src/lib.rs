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


#[derive(Unique)]
pub struct Rl(RaylibHandle);


pub fn run<SV, SR, SW, S, UV, UR, UW, U>(
    title: &str,
    start: impl AsLabel<S> + FnOnce() -> SW + 'static + Clone,
    update: impl AsLabel<U> + FnOnce() -> UW + 'static + Clone,
)
where
    SW: IntoWorkload<SV, SR>,
    UW: IntoWorkload<UV, UR>,
{
    let conf = conf::Conf {
        title: title.to_string(),
        ..Default::default()
    };
    run_ex(conf, start, update);
}


pub fn run_ex<SV, SR, SW, S, UV, UR, UW, U>(
    conf: conf::Conf,
    start: impl AsLabel<S> + FnOnce() -> SW + 'static + Clone,
    update: impl AsLabel<U> + FnOnce() -> UW + 'static + Clone,
)
where
    SW: IntoWorkload<SV, SR>,
    UW: IntoWorkload<UV, UR>,
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

    let world = World::new();
    world.add_unique(Rl(rl));
    world.add_workload(start_system);
    world.add_workload(pre_update_system);
    world.add_workload(post_update_system);
    world.add_workload(start.clone());
    world.add_workload(update.clone());

    world.run_workload(start_system).unwrap();
    world.run_workload(pre_update_system).unwrap();
    world.run_workload(start).unwrap();
    world.run_workload(post_update_system).unwrap();

    while !world.get_unique::<&Rl>().unwrap().0.window_should_close() {
        world.run_workload(pre_update_system).unwrap();
        world.run_workload(update.clone()).unwrap();
        world.run_workload(post_update_system).unwrap();

        world.get_unique::<&mut Rl>().unwrap().0.draw(&thread, |mut d| {
            d.clear_background(Color::RAYWHITE);
        });
    }
}

fn start_system() -> Workload {
    (
        || {}
    ).into_workload()
}

fn pre_update_system() -> Workload {
    (
        || {}
    ).into_workload()
}

fn post_update_system() -> Workload {
    (
        || {}
    ).into_workload()
}