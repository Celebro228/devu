pub use miniquad::conf;
use miniquad::EventHandler;
use bevy_app::prelude::*;


pub mod prelude;
pub mod ecs;


pub fn run(title: &str) {
    let conf = conf::Conf {
        window_title: title.to_string(),
        ..Default::default()
    };
    run_ex(conf);
}


pub fn run_ex(conf: conf::Conf) {
    let mut app = App::new();
    app.set_runner(|app| runner(app, conf));

    ecs::set_functions(&mut app);

    app.run();
}


fn runner(app: App, conf: conf::Conf) -> AppExit {
    miniquad::start(conf, || Box::new(Stage { app } ));

    AppExit::Success
}


pub struct Stage {
    app: App
}

impl EventHandler for Stage {
    fn update(&mut self) {
        self.app.update();
    }

    fn draw(&mut self) {

    }
}