#version 330 core

layout(location = 0) in vec3 aPos;
uniform float t;

out vec4 pos;

void main() {
    pos = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    gl_Position = pos;
    pos += 0.5;
}