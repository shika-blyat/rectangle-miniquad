use miniquad::*;

pub const VERTEX: &str = include_str!("vertex.glsl");

pub const FRAGMENT: &str = include_str!("fragment.glsl");

pub fn meta() -> ShaderMeta {
    ShaderMeta {
        images: &[],
        uniforms: UniformBlockLayout {
            uniforms: &[("offset", UniformType::Float2)],
        },
    }
}

#[repr(C)]
pub struct Uniforms {
    pub offset: (f32, f32),
}
