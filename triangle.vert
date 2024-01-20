#version 140
#define E 2.7182818284

in vec3 color;
in vec2 position;
uniform float x;

out vec4 output_color;

void main() {
    gl_Position = vec4(position, 0.0, 1.0 / 1.0 + pow(E, -x));

    output_color = vec4(color, 1.0);
}