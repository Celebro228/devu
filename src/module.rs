#[derive(Default)]
pub(crate) struct ModulesEngine {
    module_list: Vec<Box<dyn Module>>,
    module_list_len: usize,
}
impl ModulesEngine {
    pub(crate) fn update(&mut self) {
        for module in &mut self.module_list[self.module_list_len..] {
            module.start();
        }
        self.module_list_len = self.module_list.len();

        for module in &mut self.module_list {
            module.update();
        }
    }
    pub(crate) fn add(&mut self, module: impl Module) {
        self.module_list.push(Box::new(module));
    }
}


pub trait Module: std::any::Any {
    fn start(&mut self) {}
    fn update(&mut self);
    fn draw(&mut self);
}


pub struct State {

}