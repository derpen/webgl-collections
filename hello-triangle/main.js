// Get the canvas element
const canvas = document.getElementById('canvas');

// Get a WebGL context
const gl = canvas.getContext('webgl');

// Define the vertices of the triangle
const vertices = new Float32Array([
  -0.5, -0.5, 0.0,
   0.5, -0.5, 0.0,
   0.0,  0.5, 0.0
]);

// Create a vertex buffer object (VBO)
const vbo = gl.createBuffer();
gl.bindBuffer(gl.ARRAY_BUFFER, vbo);
gl.bufferData(gl.ARRAY_BUFFER, vertices, gl.STATIC_DRAW);

// Create a vertex shader
const vertexShaderCode = `
  attribute vec3 position;
  void main() {
    gl_Position = vec4(position, 1.0);
  }
`;
const vertexShader = gl.createShader(gl.VERTEX_SHADER);
gl.shaderSource(vertexShader, vertexShaderCode);
gl.compileShader(vertexShader);

// Create a fragment shader
const fragmentShaderCode = `
  void main() {
    gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);
  }
`;
const fragmentShader = gl.createShader(gl.FRAGMENT_SHADER);
gl.shaderSource(fragmentShader, fragmentShaderCode);
gl.compileShader(fragmentShader);

// Create a program and link the shaders
const program = gl.createProgram();
gl.attachShader(program, vertexShader);
gl.attachShader(program, fragmentShader);
gl.linkProgram(program);
gl.useProgram(program);

// Get the location of the position attribute
const positionLocation = gl.getAttribLocation(program, 'position');
gl.vertexAttribPointer(positionLocation, 3, gl.FLOAT, false, 0, 0);
gl.enableVertexAttribArray(positionLocation);

// Clear the canvas and draw the triangle
gl.clearColor(0.3, 0.36, 0.411, 1.0);
gl.clear(gl.COLOR_BUFFER_BIT);
gl.drawArrays(gl.TRIANGLES, 0, 3);
