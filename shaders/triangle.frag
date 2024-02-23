#version 330 core

in vec3 Color;
out vec4 FragColor;

uniform float time;
uniform float speed;

void main() {
    vec3 color = Color * abs(sin(time * speed));
    FragColor = vec4(color, 1.0) ;
}
