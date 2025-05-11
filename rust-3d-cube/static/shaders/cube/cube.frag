#version 300 es

precision highp float;
out vec4 outColor;

in vec2 TexCoord;

uniform float u_time;
uniform sampler2D image;

float multiplier = 1.0;

void main() {
  // outColor = vec4(sin(u_time * multiplier), .0, .0, 1.0);
  outColor = vec4(1.0, 0.0, 0.0, 1.0);
  outColor = vec4(vec3(texture(image, TexCoord)), 1.0);
}
