use three_d::*;


pub(crate) fn run<F: FnMut() + 'static>(title: &str, mut callpack: F) {
    let window = window(title);
    window.render_loop(move |_| {
        callpack();
        FrameOutput::default()
    });
}


fn window(title: &str) -> Window {
    Window::new(WindowSettings {
        title: title.to_string(),
        borderless: true,
        surface_settings: SurfaceSettings {
            #[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
            multisamples: 4,
            #[cfg(any(target_os = "android", target_os = "ios", target_arch = "wasm32"))]
            multisamples: 2,
            ..Default::default()
        },
        ..Default::default()
    }).expect("Window create error")
}