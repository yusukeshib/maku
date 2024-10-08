uniform sampler2D u_texture;
uniform vec2 u_resolution;

out vec4 outColor;

void main() {
  outColor = texture(u_texture, gl_FragCoord.xy / u_resolution);
}

