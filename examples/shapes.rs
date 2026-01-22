use devu::prelude::*;


fn main() {
    devu::run("Shapes");
}


#[startup]
fn spawn_shapes(
    mut commands: Commands,
) {
    commands.spawn((
        circle(100),
        position(100, 100, 0),
        GREEN,
    ));
    commands.spawn((
        rect(100, 100),
        position(100, 100, 1),
        rotation2d(45),
        BLUE,
    ));
}