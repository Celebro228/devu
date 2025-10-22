use devu::prelude::*;


fn main() {
    let node = node([
        node_cube()
            .module(Test)
            .position(),
        node_camera().rotation().rotation_target(),
    ]);

    start("Okay", node);
}


struct Test;
impl Node for Test {
    fn start() -> Self {
        
    }
    fn update(&mut self, node: &mut Node, state: &State) {
        node.node_type;
        node.name;
        state.delta;
        state.time;
        state.window_size;
        state.device; // Mobile or Desktop
    }
    fn touch_press() {

    }
    fn touch_release() {

    }
    fn touch_move() {

    }
    fn mouse_move() {

    }
    fn key_press() {

    }
    fn key_release() {

    }
}