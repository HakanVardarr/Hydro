#version 330 core

in vec3 Color;
out vec4 FragColor;

uniform float time;


void main() {
    vec3 color = Color;
    FragColor = vec4(color, 1.0) ;
}
