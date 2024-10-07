uniform sampler2D u_texture;
uniform vec2 u_resolution;

out vec4 outColor;

void main() {
  vec4 t = texture(u_texture, gl_FragCoord.xy / u_resolution);
  float luminance = (t.r + t.g + t.b) / 3.0;
  outColor = vec4(luminance, luminance, luminance, 1.0);
}
