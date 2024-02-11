#version 330 core

in vec2 texCoord;

out vec4 FragColor;

uniform float time;

uniform sampler2D containerTex;
uniform sampler2D faceTex;


void main()
{
    FragColor = mix(texture(containerTex, texCoord), texture(faceTex, texCoord), 0.2);
} 