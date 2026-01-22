pub use bevy_ecs::prelude::*;
pub use devu_macro::*;
pub use linkme::distributed_slice;


#[distributed_slice]
pub static START: [fn() -> Box<dyn System<In = (), Out = ()>>];
#[distributed_slice]
pub static UPDATE: [fn() -> Box<dyn System<In = (), Out = ()>>];