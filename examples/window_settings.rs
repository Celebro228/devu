use devu::prelude::*;


fn main() {
    devu::run_ex(Conf {
        window_title: "Window Settings".to_string(),
        window_width: 1280,
        window_height: 720,
        high_dpi: true,
        fullscreen: true,
        sample_count: 4, 
        window_resizable: false,
        ..Default::default()
    })
}


#[update]
fn print_update() {
    println!("update");
}