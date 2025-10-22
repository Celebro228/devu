use devu::prelude::*;


fn main() {
    let mut engine = devu::Engine::new();
    engine.add_module(Test::default());
    engine.run("Bench");
}




#[derive(Default)]
struct Test {

}
impl Module for Test {
    fn start(&mut self) {
        println!("hello!");
    }
    fn update(&mut self) {

    }
    fn draw(&mut self) {

    }
}