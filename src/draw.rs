use bevy_ecs::{schedule::ScheduleLabel, prelude::*};
use bevy_app::prelude::*;
use miniquad::*;
use crate::shader;


pub fn init_draw(app: &mut App) {
    app.init_schedule(Draw);

    let mut ctx = window::new_rendering_backend();

    let shader = ctx.new_shader(
        ShaderSource::Glsl {
            vertex: shader::VERTEX,
            fragment: shader::FRAGMENT,
        },
        shader::meta(),
    ).unwrap();

    let pipeline = ctx.new_pipeline(
        &[BufferLayout::default()],
        &shader::attributes(),
        shader,
        PipelineParams {
            cull_face: CullFace::Back,
            depth_write: true,
            depth_test: Comparison::LessOrEqual,
            color_blend: Some(BlendState::new(
                Equation::Add,
                BlendFactor::Value(BlendValue::SourceAlpha),
                BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
            )),
            alpha_blend: Some(BlendState::new(
                Equation::Add,
                BlendFactor::Zero,
                BlendFactor::One,
            )),
            ..Default::default()
        },
    );

    app.insert_non_send_resource(Ctx(ctx));
    app.insert_resource(Pipe(pipeline));

    app.add_systems(Draw, draw);
}


#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct Draw;

struct Ctx(Box<dyn RenderingBackend>);

#[derive(Resource)]
struct Pipe(Pipeline);


fn draw(
    mut ctx: NonSendMut<Ctx>,
    pipeline: Res<Pipe>,
) {
    ctx.0.begin_default_pass(PassAction::clear_color(5., 0., 0., 1.));
    ctx.0.apply_pipeline(&pipeline.0);

    ctx.0.end_render_pass();
    ctx.0.commit_frame();
}