use devu::prelude::*;


fn main() {
    devu::run("Camera2D");
}


#[startup]
fn spawn(
    mut commands: Commands,
) {
    commands.spawn((
        Camera2D,
        position(0., 0., 0.),
        rotation2d(),
    ));

    // Shapes
    commands.add_entity((
        Rect(10000., 300.),
        Color::RED,
    ));
    commands.add_entity((
        Rect(300., 10000.),
        Color::BLUE,
    ));
    commands.add_entity((
        Rect(300., 300.),
        Color::WHITE,
    ));
}


#[update]
fn update_camera(
    v_cameras: View<Camera2D>,
    mut vm_rotations: ViewMut<Rotation2D>,
    v_delta: UniqueView<DeltaTime>,
) {
    for (_camera, rotation) in (&v_cameras, &mut vm_rotations).iter() {
        **rotation += **v_delta * 100.;
    }
}