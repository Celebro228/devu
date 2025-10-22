/*

DeVu - легкий и быстрый движок по типу ccode и fancade,
    и он не будет полным НИКОГДА. Этот движок разработан
    в первую очередь для легкости разработки 3д игр.

TODO:
[#] Создание окна
[?] Основная стуктура
[] Создать обычный мир
[] Добавление 3д объектов
[] Добавить мобули для 3д объектов
[] Рисование 3д объектов
[] Создать state
[] Создать основную сцену
[] Создать ивенты клавы
[] Создать ивенты мыши
[] Создать 3д камеру
[] Аудио
[] Добавить физику
[] Создание DEVU-CLI
[] Экспорт на андроид
[] Создать resource
[] Добавление изображения
[] Экспорт модели
[] Веб
[] Добавление текста
[] Сохранение данных
[] Мультиплеер

*/


pub mod prelude;
pub mod render;
pub mod module;
//pub mod utils;


use module::ModulesEngine;
use module::Module;


pub struct Engine {
    modules: ModulesEngine,
}
impl Engine {
    pub fn new() -> Self {
        Self {
            modules: ModulesEngine::default(),
        }
    }
    pub fn add_module(&mut self, module: impl Module) -> &mut Self {
        self.modules.add(module);
        self
    }
    pub fn run(mut self, title: &str) {
        render::run(title, move || {
            self.modules.update();
        });
    }
}