// TODO: Добавить текст


use devu::prelude::*;
use shipyard::IntoIter;


#[derive(Component)]
struct DeltaShape(bool);


fn main() {
    devu::run(
        "DeltaTime",
        start,
        update,
    )
}


fn start() -> Workload {
    (
        spawn_shapes,
    ).into_workload()
}

fn spawn_shapes(
    mut vm_world: WorldViewMut,
) {
    // When showing the world, you can't show anything else
    let height = {
        let v_screen = vm_world.get_unique::<&Screen>().unwrap();
        v_screen.height
    };

    // Delta Circle
    vm_world.add_entity((
        Circle(100.),
        Position2D(vec2(
            0.0,
            height * 0.33,
        )),
        Color::RED,
        DeltaShape(true),
    ));

    // No-delta Circle
    vm_world.add_entity((
        Circle(100.),
        Position2D(vec2(
            0.0, 
            height * 0.66,
        )),
        Color::BLUE,
        DeltaShape(false),
    ));
}


fn update() -> Workload {
    (
        update_shapes,
    ).into_workload()
}

fn update_shapes(
    mut vm_positions: ViewMut<Position2D>,
    v_circles: View<Circle>,
    v_delta_shapes: View<DeltaShape>,
    v_delta: UniqueView<DeltaTime>,
    v_screen: UniqueView<Screen>,
) {
    let speed = 100.;

    for (_, delta_shape, position) in (&v_circles, &v_delta_shapes, &mut vm_positions).iter() {
        if delta_shape.0 {
            position.0.x += speed * v_delta.0;
        } else {
            position.0.x += speed / 60.;
        }

        if position.0.x > v_screen.width {
            position.0.x = 0.;
        }
    }
}