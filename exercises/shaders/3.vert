#version 330 core

layout(location = 0) in vec3 aPos;
uniform float t;
uniform float h_offset;

out vec4 vertexColor;

void main() {
    gl_Position = vec4(aPos.x + h_offset, aPos.y, aPos.z, 1.0);
    vertexColor = vec4(0.5, (sin(t) / 2.0) + 0.5, 0.0, 1.0);
}