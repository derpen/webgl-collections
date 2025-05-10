#version 300 es

in vec4 position;
in vec3 normal;
in vec2 texcoord;

out vec3 Normal;
out vec2 TexCoord;

uniform mat4 model;

void main() {
  gl_Position = model * position;
  Normal = normal;
  TexCoord = texcoord;
}
