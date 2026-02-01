use miniquad::*;
// use glam::Mat4;
// use crate::color::*;


pub const VERTEX: &str = r#"#version 100
attribute vec3 in_pos;

uniform mat4 mvp;

void main() {
    gl_Position = mvp * vec4(in_pos, 1);
}"#;

pub const FRAGMENT: &str = r#"#version 100
uniform lowp vec4 color;

void main() {
    gl_FragColor = color;
}"#;


pub fn attributes() -> [VertexAttribute; 1] {
    [
        VertexAttribute::new("in_pos", VertexFormat::Float3),
    ]
}

pub fn meta() -> ShaderMeta {
    ShaderMeta {
        images: vec![],
        uniforms: UniformBlockLayout {
            uniforms: vec![
                UniformDesc::new("mvp", UniformType::Mat4),
                UniformDesc::new("color", UniformType::Float4),
            ],
        },
    }
}


// TODO: Удалить либо использовать
// #[repr(C)]
// pub struct Uniforms {
//     pub mvp: Mat4,
//     pub color: Color,
// }
