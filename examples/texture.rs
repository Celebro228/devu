use devu::prelude::*;


fn main() {
    devu::run("Texture");
}


#[startup]
fn spawn_texture(
    mut commands: Commands,
) {
    commands.spawn((
        texture("examples/assets/head.png"),
    ));
}