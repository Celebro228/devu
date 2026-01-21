use raylib::prelude::*;


pub mod prelude;
pub mod conf;


pub fn run(title: &str) {
    let conf = conf::Conf {
        title: title.to_string(),
        ..Default::default()
    };
    run_ex(conf);
}


pub fn run_ex(conf: conf::Conf) {
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
    let (mut rl, thread) = builder.build();

    while !rl.window_should_close() {
        rl.draw(&thread, |mut d| {
            d.clear_background(Color::RAYWHITE);
        });
    }
}