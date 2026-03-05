pub use bevy_ecs::prelude::*;
pub use devu_macro::*;
pub use linkme::distributed_slice;

use bevy_app::prelude::*;
pub use bevy_app::App;


#[distributed_slice]
pub static PRE_START: [fn() -> Box<dyn System<In = (), Out = ()>>];
#[distributed_slice]
pub static START: [fn() -> Box<dyn System<In = (), Out = ()>>];
#[distributed_slice]
pub static POST_START: [fn() -> Box<dyn System<In = (), Out = ()>>];

#[distributed_slice]
pub static PRE_UPDATE: [fn() -> Box<dyn System<In = (), Out = ()>>];
#[distributed_slice]
pub static UPDATE: [fn() -> Box<dyn System<In = (), Out = ()>>];
#[distributed_slice]
pub static POST_UPDATE: [fn() -> Box<dyn System<In = (), Out = ()>>];

#[distributed_slice]
pub static EVENT: [fn(&mut App)];


pub(crate) fn set_functions(app: &mut App) {
    for system_pre_start in PRE_START {
        app.add_systems(PreStartup, system_pre_start());
    }
    for system_start in START {
        app.add_systems(Startup, system_start());
    }
    for post_system_start in POST_START {
        app.add_systems(PostStartup, post_system_start());
    }

    for system_pre_update in PRE_UPDATE {
        app.add_systems(PreUpdate, system_pre_update());
    }
    for system_update in UPDATE {
        app.add_systems(Update, system_update());
    }
    for post_system_update in POST_UPDATE {
        app.add_systems(PostUpdate, post_system_update());
    }
    
    for event_systems in EVENT {
        event_systems(app);
    }
}