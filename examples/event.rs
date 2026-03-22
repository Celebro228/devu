use devu::prelude::*;


fn main() {
    let mut app = devu::init();
    app.update();
}


#[derive(Event)]
struct GameStarted;

#[startup]
fn start(
    mut commands: Commands,
) {
    commands.trigger(GameStarted);
}

#[event]
fn hello_color(
    _event: On<GameStarted>,
) {
    println!("Game is Start");
}