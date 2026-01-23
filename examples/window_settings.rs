use devu::prelude::*;


fn main() {
    devu::run_ex(Conf {
        title: "Window Settings".to_string(),
        size: (1280, 720),
        resizable: true,
        msaa_4x: false,
        vsync: false,
        logging: false,
    })
}


#[startup]
fn setup(
    mut fullscreen: MessageWriter<Fullscreen>,
) {
    fullscreen.write(Fullscreen(true));
}

#[update]
fn print_update(
    window: Res<Window>,
) {
    println!("{} {:?}", window.fullscreen(), window.size());
}