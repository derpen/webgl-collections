#version 300 es

precision highp float;
out vec4 outColor;

uniform float u_time;

float multiplier = 1.0;

void main() {
  outColor = vec4(sin(u_time * multiplier), .0, .0, 1.0);
}
