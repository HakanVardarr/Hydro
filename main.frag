#version 330 core
in vec4 vertex_color;
in vec2 texture_coords;

out vec4 FragColor;

uniform float time;
uniform sampler2D ourTexture;

void main()
{
    vec4 color = texture(ourTexture, texture_coords);
    FragColor = color;
} 