use devu::prelude::*;


fn main() {
    devu::run(
        "Models",
        start,
        update,
    )
}


fn start() -> Workload {
    (
        spawn_models,
    ).into_workload()
}

fn spawn_models(
    mut vm_world: WorldViewMut,
) {
    vm_world.add_entity((
        Sphere(1.0),
        Position3D::new(-2., 0., 5.),
    ));

    vm_world.add_entity((
        Cube(0.4, 0.4, 0.4),
        Position3D::new(2., 0., 5.),
        Rotation3D::new(45., 45., 45.),
        Scale3D::new(2., 2., 2.),
        Color::BLUE,
        Visible(true),
    ));
}


fn update() -> Workload {
    ( || { } ).into_workload()
}