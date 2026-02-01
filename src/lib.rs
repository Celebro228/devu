pub use miniquad::conf;
use miniquad::EventHandler;
use bevy_app::prelude::*;


mod draw;
mod shader;

pub mod prelude;
pub mod ecs;
pub mod color;


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
    app.run();
}


fn runner(mut app: App, conf: conf::Conf) -> AppExit {
    miniquad::start(conf, || {

        draw::init_draw(&mut app); // set
        ecs::set_functions(&mut app); // set

        Box::new(Stage { app } )
    });

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
        self.app.world_mut().run_schedule(draw::Draw);
    }
}