#version 330 core

out vec4 FragColor;

in vec2 position;
in vec2 texture_position;

uniform vec2 resolution;
uniform float time;
uniform sampler2D tex;

void main()
{   
	FragColor = texture(tex, texture_position);
}

