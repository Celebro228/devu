use devu::prelude::*;


fn main() {
    devu::run("Window");
}


#[derive(Component)]
struct Ok;


#[startup]
fn hello_world_1(
    d: Query<&Ok>,
) {
    println!("Hello world! len: {}", d.iter().len());
}

#[startup]
fn hello_world_2() {
    println!("Hello world! 2");
}

#[startup]
fn hello_world_3() {
    println!("Hello world! 3");
}