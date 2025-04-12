#version 300 es

precision highp float;
out vec4 outColor;

uniform float u_time;

void main() {
  outColor = vec4(sin(u_time) * 0.2, .0, .0, 1.0);
}
