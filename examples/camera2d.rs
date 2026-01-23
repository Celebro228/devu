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
        position(0, 0, 0),
        rotation2d(0),
    ));
    commands.spawn((
        rect(10000, 300),
        RED,
    ));
    commands.spawn((
        rect(300, 10000),
        BLUE,
    ));
    commands.spawn((
        rect(300, 300),
        WHITE,
    ));
}


#[update]
fn update_camera(
    mut camera: Single<&mut Rotation2D, With<Camera2D>>,
    time: Res<Time>,
) {
    camera.0 += time.delta() * 90.;
}