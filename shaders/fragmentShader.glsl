///totw fragment_shader
#version 140

#ifdef GL_ES
precision mediump float;
#endif

uniform vec2 u_resolution;
uniform sampler2D u_texture;

smooth in vec2 tex_coord;

void main() {
    //vec4(1.0, 0.0, 0.0, 1.0);
    gl_FragColor = texture(u_texture, tex_coord);
}
