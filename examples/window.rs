use devu::prelude::*;


fn main() {
    devu::run("Window");
}


#[startup]
fn hello_world() {
    println!("Hello world!");
}