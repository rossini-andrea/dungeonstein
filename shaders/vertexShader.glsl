///totw vertex_shader
#version 140

#ifdef GL_ES
precision mediump float;
#endif

uniform mat4 u_model_matrix;
uniform mat4 u_view_matrix;
uniform mat4 u_projection_matrix;

in vec4 a_position;
in vec2 a_tex_coord;

smooth out vec2 tex_coord;

void main()
{
    gl_Position = u_projection_matrix * u_view_matrix * u_model_matrix * a_position;
    tex_coord = a_tex_coord;
}
