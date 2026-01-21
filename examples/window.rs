use devu::prelude::*;


fn main() {
    devu::run(
        "Window",
    )
}


#[system(START)]
fn hello_world() {
    println!("Hello world!");
}