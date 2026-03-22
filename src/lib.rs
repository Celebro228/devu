use bevy_app::prelude::*;


pub mod prelude;
pub mod ecs;
pub mod math;
pub mod transform;


pub fn init() -> App {
    let mut app = App::new();
    ecs::set_functions(&mut app);
    app
}