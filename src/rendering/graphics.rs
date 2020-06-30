use std::f32::consts::PI;
use glm::{Vec3};
use na::base::Matrix4;
use gl::types::GLuint;
use crate::rendering::{
    engine_types::Uniforms,
    glwrap,
    glwrap::{
        GlShaderProgram, GlShader, GlVertexArray, Bind
    }
};
use crate::game::model::{
    DungeonFloor,
    DungeonCell::{Empty, Floor, Wall}
};
use sdl2::{
    render::{ Texture, TextureCreator },
    image::LoadTexture,
    video::WindowContext
};
use slotmap::{ SlotMap, new_key_type };

pub struct DungeonGraphics<'r> {
    wall_texture: Texture<'r>,
    floor_texture: Texture<'r>,
    shader_program: GlShaderProgram,
    uniforms: Uniforms,
    a_position: GLuint,
    a_tex_coord: GLuint,
    wall_vertex_array: GlVertexArray,
    floor_vertex_array: GlVertexArray
}

impl<'r> DungeonGraphics<'r> {
    pub fn new(texture_creator: & 'r TextureCreator<WindowContext>) -> Self {
        let a_position: GLuint = 0;
        let a_tex_coord: GLuint = 1;
        let shaders = vec![
            GlShader::from_file("shaders/vertexShader.glsl")
                .unwrap(),
            GlShader::from_file("shaders/fragmentShader.glsl")
                .unwrap()
        ];
        let shader_program = GlShaderProgram::new(&shaders, &attrib_bindings![a_position, a_tex_coord])
            .unwrap();
        let wall_vertex_array = GlVertexArray::from_vertex_buffer(
            &[
                0.0, 0.0, 0.0,    0.0, 0.0,
                2.0, 0.0, 0.0,    1.0, 0.0,
                2.0, 0.0, 2.0,    1.0, 1.0,
                0.0, 0.0, 2.0,    0.0, 1.0,
                2.0,  2.0, 0.0,    2.0, 0.0,
                2.0,  2.0, 2.0,    2.0, 1.0,
                0.0,  2.0, 0.0,    3.0, 0.0,
                0.0,  2.0, 2.0,    3.0, 1.0,
                0.0, 0.0, 0.0,    4.0, 0.0,
                0.0, 0.0, 2.0,    4.0, 1.0,
            ],
            &[
                0, 1, 2,
                0, 2, 3,
                1, 4, 5,
                1, 5, 2,
                4, 6, 7,
                4, 7, 5,
                6, 8, 9,
                6, 9, 7,
            ],
            &[(a_position, 3), (a_tex_coord, 2)]
        );
        let floor_vertex_array = GlVertexArray::from_vertex_buffer(
            &[
                0.0, 0.0, 0.0,    0.0, 0.0,
                2.0, 0.0, 0.0,    1.0, 0.0,
                2.0, 2.0, 0.0,    1.0, 1.0,
                0.0, 2.0, 0.0,    0.0, 1.0,
            ], &[0, 1, 2, 0, 2, 3], &[(a_position, 3), (a_tex_coord, 2)]
        );
        let uniforms = Uniforms::from_program(&shader_program);
        Self {
            wall_texture: glwrap::texture_from_file(texture_creator, "textures/wall00.png"),
            floor_texture: glwrap::texture_from_file(texture_creator, "textures/floor00.png"),
            shader_program,
            uniforms,
            a_position, a_tex_coord,
            wall_vertex_array, floor_vertex_array
        }
    }
}

new_key_type! {
    struct ModelKey;
    struct ShaderKey;
    struct RenderElementKey;
    struct TextureElementKey;
    struct VertexArrayKey;
}

#[derive(Copy, Clone)]
struct Sprite {
    texture: TextureElementKey,
    shape: VertexArrayKey
}

#[derive(Copy, Clone)]
struct Tile {

}

#[derive(Copy, Clone)]
pub enum Model {
    Sprite(Sprite),
    Tile(Tile)
}

#[derive(Copy, Clone)]
struct RenderElement {
    model: ModelKey,
    pos: [i32; 2],
    facing: [i32; 2]
}

pub struct ViewSettings {
    pub pos: Vec3,
    pub facing: Vec3,
    pub height: f32
}

pub struct GlEngine<'r> {
    world_graphics: DungeonGraphics<'r>,
    models: SlotMap<ModelKey, Model>,
    render_elements: SlotMap<RenderElementKey, RenderElement>
}

impl<'r> GlEngine<'r> {
    pub fn new(texture_creator: & 'r TextureCreator<WindowContext>) -> Self {
        Self {
            world_graphics: DungeonGraphics::new(&texture_creator),
            models: SlotMap::with_capacity_and_key(16),
            render_elements: SlotMap::with_capacity_and_key(16)
        }
    }

    pub fn render(&mut self, world: &DungeonFloor, view_settings: ViewSettings) {
        self.render_world(world, &view_settings);
/*
        for (_, element) in self.render_elements.iter() {

        }*/
    }

    fn render_world(&mut self, world: &DungeonFloor, view_settings: &ViewSettings) {
        let _prg_bind = Bind::new(&self.world_graphics.shader_program);
        let projection_matrix: [[f32; 4]; 4] = glm::perspective(
            8.0 / 6.0, PI * 0.4, 0.1, 100.0
        ).into();
        let view_matrix: [[f32; 4]; 4] = glm::look_at(
            &Vec3::new(
                view_settings.pos[0],
                view_settings.pos[1],
                view_settings.height
            ),
            &Vec3::new(
                (view_settings.pos[0] + view_settings.facing[0]) as f32,
                (view_settings.pos[1] + view_settings.facing[1]) as f32,
                view_settings.height
            ),
            &Vec3::new(0.0, 0.0, 1.0)).into();

        unsafe {
            gl::Uniform2d(self.world_graphics.uniforms.u_resolution, 800.0, 600.0);
            gl::UniformMatrix4fv(self.world_graphics.uniforms.u_projection_matrix, 1, gl::FALSE,
                &projection_matrix[0][0] as *const f32);
            gl::UniformMatrix4fv(self.world_graphics.uniforms.u_view_matrix, 1, gl::FALSE,
                &view_matrix[0][0] as *const f32);
            gl::Uniform1ui(self.world_graphics.uniforms.u_texture0, gl::TEXTURE0);

            gl::ActiveTexture(gl::TEXTURE0);
        }

        let w = world.width;
        for x in 0..w {
            for y in 0..world.height {
                let model_matrix: [[f32; 4]; 4] = glm::translation(
                    &Vec3::new(x as f32 * 2.0, y as f32 * 2.0, 0.0)
                ).into();
                unsafe { gl::UniformMatrix4fv(self.world_graphics.uniforms.u_model_matrix, 1,
                    gl::FALSE, &model_matrix[0][0] as *const f32); }

                match world[(x, y)] {
                    Floor => {
                        let _vertex_bind = Bind::new(&self.world_graphics.floor_vertex_array);
                        unsafe {
                            self.world_graphics.floor_texture.gl_bind_texture();
                            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null::<std::ffi::c_void>());
                            self.world_graphics.floor_texture.gl_unbind_texture();
                        }
                    }
                    Wall => {
                        let _vertex_bind = Bind::new(&self.world_graphics.wall_vertex_array);
                        unsafe {
                            self.world_graphics.wall_texture.gl_bind_texture();
                            gl::DrawElements(gl::TRIANGLES, 24, gl::UNSIGNED_INT, std::ptr::null::<std::ffi::c_void>());
                            self.world_graphics.wall_texture.gl_unbind_texture();
                        }
                    }
                    _ => { }
                }
            }
        }
    }
}
