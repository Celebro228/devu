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