use devu::prelude::*;


fn main() {
    devu::run("Event");
}


#[startup]
fn start(
    mut commands: Commands,
) {
    commands.spawn(RED);
}


#[event]
fn hello_color(
    event: On<Add, Color>,
    colors: Query<&Color>,
) {
    println!("Hello {:?}!", colors.get(event.entity).unwrap());
}