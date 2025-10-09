use devu::*;


fn main() {
    let mut engine = Engine::new();
    engine.add_module(Test);
    engine.run("Okay");
}


struct Test;
impl Module for Test {
    fn start(&mut self) {
        
    }
    fn update(&mut self) {
        
    }
    fn draw(&mut self) {
        
    }
}