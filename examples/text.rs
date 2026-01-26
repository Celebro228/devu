use devu::prelude::*;


fn main() {
    devu::run("Text");
}


#[startup]
fn spawn_texture(
    mut commands: Commands,
) {
    commands.spawn((
        text("examples/assets/head.png", 50.),
    ));
}