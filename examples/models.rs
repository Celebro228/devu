use devu::prelude::*;


fn main() {
    devu::run(
        "Models",
        start,
        update,
    )
}


fn start() -> Workload {
    (
        spawn_models,
    ).into_workload()
}

fn spawn_models(
    mut vm_world: WorldViewMut,
) {
    vm_world.add_entity((
        Cube(100.),
        Position2D(vec2(
            width * 0.33, 
            height / 2.0,
        )),
        // Rotation(0.) - You don't have to put it
        // Components without position and rotation are equivalent to their default versions.
    ));

    vm_world.add_entity((
        Rect(75., 75.),
        Position2D(vec2(
            width * 0.66, 
            height / 2.0,
        )),
        Rotation2D(45.),
        Scale2D(2.),
        Color::BLUE,
        Visible(true),
    ));
}


fn update() -> Workload {
    ( || { } ).into_workload()
}