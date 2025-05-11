#version 300 es

in vec4 position;
in vec3 normal;
in vec2 texcoord;

out vec3 Normal;
out vec2 TexCoord;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
  gl_Position = projection * view * model * position;
  Normal = normal;
  TexCoord = texcoord;
}
