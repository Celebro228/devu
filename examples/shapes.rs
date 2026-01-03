use devu::prelude::*;


fn main() {
    devu::run(
        "Shapes",
        start,
        update,
    )
}


fn start() -> Workload {
    (
        spawn_shapes,
    ).into_workload()
}

fn spawn_shapes(
    mut vm_world: WorldViewMut,
) {
    // When showing the world, you can't show anything else
    let (width, height) = {
        let v_screen = vm_world.get_unique::<&Screen>().unwrap();
        (v_screen.width, v_screen.height)
    };

    vm_world.add_entity((
        Circle(100.),
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
    ));
}


fn update() -> Workload {
    ( || { } ).into_workload()
}