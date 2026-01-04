use devu::prelude::*;
use shipyard::IntoIter;


fn main() {
    devu::run(
        "Shapes",
        start,
        update,
    )
}


fn start() -> Workload {
    (
        spawn_camera,
    ).into_workload()
}

fn spawn_camera(
    mut vm_world: WorldViewMut,
) {
    let (width, height) = {
        let v_screen = vm_world.get_unique::<&Screen>().unwrap();
        (v_screen.width, v_screen.height)
    };

    // All transformations available to shapes are applied to the camera.
    vm_world.add_entity((
        Camera2D,
        Position2D(vec2(
            -width / 2.0, 
            -height / 2.0,
        )),
        Rotation2D(0.),
        Scale2D(2.),
    ));

    // Shapes
    vm_world.add_entity((
        Rect(10000., 300.),
        Color::RED,
    ));
    vm_world.add_entity((
        Rect(300., 10000.),
        Color::BLUE,
    ));
    vm_world.add_entity((
        Rect(300., 300.),
        Color::WHITE,
    ));
}


fn update() -> Workload {
    (
        update_camera,
    ).into_workload()
}

fn update_camera(
    v_cameras: View<Camera2D>,
    mut vm_rotations: ViewMut<Rotation2D>,
    v_delta: UniqueView<DeltaTime>,
) {
    for (_camera, rotation) in (&v_cameras, &mut vm_rotations).iter() {
        rotation.0 += v_delta.0 * 100.;
    }
}