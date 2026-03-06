use devu::prelude::*;


fn main() {
    devu::run("Event");
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