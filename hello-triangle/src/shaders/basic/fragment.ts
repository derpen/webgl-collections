export const _fragmentShader: string = `
  precision mediump float;

  void main(){
    vec3 color = vec3(1.0, 1.0, 0.0);
    gl_FragColor = vec4(color, 1.0);
  }
`;
