use bevy_app::prelude::*;
use raylib::prelude::*;


pub mod prelude;
pub mod conf;
pub mod ecs;


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


    for system_start in ecs::START {
        app.add_systems(Startup, system_start());
    }

    for system_update in ecs::UPDATE {
        app.add_systems(Update, system_update());
    }
    

    app.run();
}


fn runner(mut app: App, conf: conf::Conf) -> AppExit {
    let (mut rl, thread) = conf.build();


    while !rl.window_should_close() {
        app.update();

        rl.draw(&thread, |mut d| {
            d.clear_background(Color::RAYWHITE);
        });
    }


    AppExit::Success
}