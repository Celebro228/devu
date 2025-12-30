use devu::prelude::*;


fn main() {
    devu::run(
        "Window",
        start,
        update,
    )
}


fn start() -> Workload {
    ( || { } ).into_workload()
}


fn update() -> Workload {
    ( || { } ).into_workload()
}