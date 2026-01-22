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
        position(100, 0, 0),
        GREEN,
    ));
}