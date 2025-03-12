export function compileShader(gl: WebGLRenderingContext, shaderCode: string, shaderType: number): WebGLShader {
  const shader: WebGLShader = gl.createShader(shaderType) as WebGLShader;
  gl.shaderSource(shader, shaderCode);
  gl.compileShader(shader);
  return shader;
};

export function createProgram(gl: WebGLRenderingContext, vertexShader: WebGLShader, fragmentShader: WebGLShader): WebGLProgram {
  const program: WebGLProgram = gl.createProgram() as WebGLProgram;
  gl.attachShader(program, vertexShader);
  gl.attachShader(program, fragmentShader);
  gl.linkProgram(program);
  gl.useProgram(program);
  return program;
}
