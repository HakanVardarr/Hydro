#version 330 core
in vec4 vertex_color;

out vec4 FragColor;

uniform float time;

void main()
{
    vec4 color = vertex_color * min((0.2 + abs(sin(time * 2.0))), 1.0);
    FragColor = color;
} 