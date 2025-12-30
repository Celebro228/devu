use devu::prelude::*;


fn main() {
    devu::start("Bench", node(vec![]));
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