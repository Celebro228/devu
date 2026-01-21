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
    let (mut rl, thread) = conf.build();


    for system in ecs::START {
        system();
    }


    while !rl.window_should_close() {
        rl.draw(&thread, |mut d| {
            d.clear_background(Color::RAYWHITE);
        });
    }
}