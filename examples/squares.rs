use devu::prelude::*;


fn main() {
    devu::run("Shapes");
}


#[startup]
fn spawn_shapes(
    mut commands: Commands,
) {
    commands.spawn((
        Circle(100.),
        position(0, 0, 0),
    ));
}