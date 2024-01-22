#version 330 core
layout(location = 0) in vec3 aPos;
layout(location = 1) in vec2 aTex;

uniform mat4 matrix;

out vec2 position;
out vec2 texture_position;

void main() {
    gl_Position = vec4(aPos, 1.0) * matrix;
    position = aPos.xy;
    texture_position = aTex;
}