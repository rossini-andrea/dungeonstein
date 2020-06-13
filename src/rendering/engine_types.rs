use crate::rendering::glwrap::GlShaderProgram;
use gl::types::{
    GLuint, GLint
};

/// Program attributes I suppose my basic engine will need
/// (refactor when it ages!).
pub enum ProgramAttribs {
    Position = 0,
    Normal = 1,
    Color = 2,
    TexCoord0 = 3,
    TexCoord1 = 4
}

impl From<ProgramAttribs> for GLuint {
    fn from(value: ProgramAttribs) -> Self {
        value as Self
    }
}

/// Program uniforms
pub struct Uniforms {
    pub u_model_matrix: GLint,
    pub u_view_matrix: GLint,
    pub u_projection_matrix: GLint,
    pub u_resolution: GLint,
    pub u_texture0: GLint,
    pub u_texture1: GLint
}

impl Uniforms {
    pub fn from_program(program: &GlShaderProgram) -> Self {
        Self {
            u_model_matrix: program.uniform_location("u_model_matrix\0"),
            u_view_matrix: program.uniform_location("u_view_matrix\0"),
            u_projection_matrix: program.uniform_location("u_projection_matrix\0"),
            u_resolution: program.uniform_location("u_resolution\0"),
            u_texture0: program.uniform_location("u_texture0\0"),
            u_texture1: program.uniform_location("u_texture1\0")
        }
    }
}
