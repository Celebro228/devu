use bevy_app::prelude::*;


mod rl;
mod draw;

pub mod prelude;
pub mod conf;
pub mod ecs;
pub mod math;
pub mod window;
pub mod color;
pub mod transform;
pub mod camera;
pub mod shapes;


pub fn run(title: &str) {
    let conf = conf::Conf {
        title: title.to_string(),
        ..Default::default()
    };
    run_ex(conf);
}


pub fn run_ex(conf: conf::Conf) {
    let mut app = App::new();
    app.set_runner(|app| runner(app, conf));

    draw::init_draw(&mut app);
    window::init_window(&mut app);
    ecs::set_functions(&mut app);

    app.run();
}


fn runner(mut app: App, conf: conf::Conf) -> AppExit {
    let (raly, thread) = conf.build();
    app.insert_resource(rl::Rl(raly));
    app.insert_non_send_resource(rl::Thread(thread));


    while !app.world().resource::<rl::Rl>().window_should_close() {
        app.update();
        app.world_mut().run_schedule(draw::Draw);
    }


    AppExit::Success
}


