/*

TODO:
[#] Создание окна
[] Основная логика
[] Модули
[] Логика модулей
[] Создать info
[] Создать основную сцену
[] Добавление 3д объектовя
[] Добавить мобули для 3д объектов
[] Рисование 3д объектов
[] Создать ивенты клавы
[] Создать ивенты мыши
[] Создать 3д камеру
[] Добавить физику
[] Создание DEVU-CLI
[] Создать resource
[] Добавление изображения
[] Экспорт модели
[] Аудио
[] Веб аудио
[] Добавление текста
[] Сохранение данных
[] Мультиплеер

*/


mod threed;


pub struct Engine {
    module_list: Vec<Box<dyn Module>>,
}
impl Engine {
    pub fn new() -> Self {
        Self {
            module_list: Vec::new(),
        }
    }
    pub fn add_module(&mut self, module: impl Module) -> &mut Self {
        self.module_list.push(Box::new(module));
        self
    }
    pub fn run(self, title: &str) {
        threed::run(title, || {

        });
    }
}


pub trait Module: std::any::Any {
    fn start(&mut self);
    fn update(&mut self);
    fn draw(&mut self);
}