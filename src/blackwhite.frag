uniform sampler2D u_texture;

out vec2 v_texCoord;
out vec4 fragColor;

void main() {
  vec4 t = texture2D(u_texture, v_texCoord);
  float luminance = (t.r+t.g+t.b)/3.0;
  gl_FragColor = vec4(luminance, luminance, luminance, 1.0);
}
