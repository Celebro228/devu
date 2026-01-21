use devu::prelude::*;


fn main() {
    devu::run(
        "Window",
        start,
        update,
    )
}


#[system(START)]
fn hello_world() {
    println!("Hello world!");
}