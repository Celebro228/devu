/*


DeVu - легкий и быстрый движок по типу ccode и fancade,
    и он не будет полным НИКОГДА. Этот движок разработан
    в первую очередь для легкости разработки 3д игр.


DeVu = three-d + kira + rapier + iroh


Архитектура:
lib - связь между устройством и движком (окно): threed {
    state
    node
    audio: kira
    physic: rapier
    multiplayer: iroh
}
singletoon {
    cross: rayon + parking_lot - кроссплатформенные функции
    render: threed
}


TODO:
[#] Создание окна
[?] Основная стуктура
[?] Создать обычный мир
[] Добавление 3д объектов
[] Добавить мобули для 3д объектов
[] Рисование 3д объектов
[] Создать state
[] Создать ивенты клавы
[] Создать ивенты мыши
[] Создать 3д камеру
[] Аудио
[] Добавить физику
[] Добавить многопоточность
[] Создание DEVU-CLI
[] Экспорт на андроид
[] Создать resource
[] Добавление изображения
[] Экспорт модели
[] Веб
[] Добавление текста
[] Сохранение данных
[] Мультиплеер


*/


pub mod prelude;
pub mod render;
pub mod module;
pub mod node;
//pub mod utils;

use node::NodeCreate;

use three_d::*;


pub fn start(title: &str, node: NodeCreate) {
    // Окно
    let window = window(title);
    let context = window.gl();

    // Свет
    let directional_light = DirectionalLight::new(
        &context, 
        1.0,
        Srgba::WHITE,
        vec3(0.5, 0.5, 0.5),
    );
    let ambient_light = AmbientLight::new(
        &context,
        0.1,
        Srgba::WHITE
    );

    // Камера (ВРЕМЕННО!!!)
    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(5.0, 2.0, 2.5),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    );
    let mut control = OrbitControl::new(camera.target(), 1.0, 100.0);

    let mut sphere = Gm::new(
        Mesh::new(&context, &CpuMesh::cube()),
        PhysicalMaterial::new_transparent(
            &context,
            &CpuMaterial {
                albedo: Srgba::RED,
                ..Default::default()
            }
        ),
    );

    // Вечно
    window.render_loop(move |mut frame| {
        camera.set_viewport(frame.viewport);
        control.handle_events(&mut camera, &mut frame.events);

        // Рендер
        frame.screen()
            .clear(ClearState::color_and_depth(
                0.1,
                0.1, 
                0.1, 
                1.0,
                1.,
            )).render(
                &camera,
                &sphere, 
                &[&directional_light, &ambient_light],
            );
        FrameOutput::default()
    });
}


fn window(title: &str) -> Window {
    Window::new(WindowSettings {
        title: title.to_string(),
        borderless: true,
        surface_settings: SurfaceSettings {
            #[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
            multisamples: 4,
            #[cfg(any(target_os = "android", target_os = "ios", target_arch = "wasm32"))]
            multisamples: 2,
            ..Default::default()
        },
        ..Default::default()
    }).expect("Window create error")
}