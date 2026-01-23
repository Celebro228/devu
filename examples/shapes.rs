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
        GREEN,
    ));
    commands.spawn((
        rect(100, 100),
        position(0, 0, 1),
        rotation2d(45),
        BLUE,
    ));
    commands.spawn((
        line(vec2(0, -100), vec2(0, 100), 50),
        position(-100, 0, -1),
        RED,
    ));
}