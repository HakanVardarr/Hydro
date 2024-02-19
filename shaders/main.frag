#version 330 core

in vec2 texCoord;

out vec4 FragColor;

uniform float time;

uniform sampler2D crocodileTex;


void main()
{
    FragColor = texture(crocodileTex, texCoord);
} 