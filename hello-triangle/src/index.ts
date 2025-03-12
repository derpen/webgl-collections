import { compileShader, createProgram } from './utils/utils';
import { _vertexShader } from './shaders/basic/vertex';
import { _fragmentShader } from './shaders/basic/fragment';

const canvas: HTMLCanvasElement = document.getElementById('canvas') as HTMLCanvasElement;
const gl: WebGLRenderingContext = canvas.getContext('webgl') as WebGLRenderingContext;

const vertices: Float32Array = new Float32Array([
  -0.5, -0.5, 0.0,
   0.5, -0.5, 0.0,
   0.0,  0.5, 0.0
]);
const vbo: WebGLBuffer = gl.createBuffer() as WebGLBuffer;
gl.bindBuffer(gl.ARRAY_BUFFER, vbo);
gl.bufferData(gl.ARRAY_BUFFER, vertices, gl.STATIC_DRAW);

const vertexShaderCode: string = _vertexShader;
const fragmentShaderCode: string = _fragmentShader;

const vertexShader: WebGLShader = compileShader(gl, vertexShaderCode, gl.VERTEX_SHADER);
const fragmentShader: WebGLShader = compileShader(gl, fragmentShaderCode, gl.FRAGMENT_SHADER);
const program: WebGLProgram = createProgram(gl, vertexShader, fragmentShader);

const positionLocation: number = gl.getAttribLocation(program, 'position');
gl.vertexAttribPointer(positionLocation, 3, gl.FLOAT, false, 0, 0);
gl.enableVertexAttribArray(positionLocation);

gl.clearColor(0.0, 0.0, 0.0, 1.0);
gl.clear(gl.COLOR_BUFFER_BIT);
gl.drawArrays(gl.TRIANGLES, 0, 3);
