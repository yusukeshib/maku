uniform sampler2D u_texture;

out vec4 outColor;

void main() {
  vec4 t = texture(u_texture, gl_FragCoord.xy);
  float luminance = (t.r + t.g + t.b) / 3.0;
  outColor = vec4(luminance, luminance, luminance, 1.0);
}
