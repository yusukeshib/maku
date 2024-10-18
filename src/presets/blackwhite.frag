uniform sampler2D u_texture;
uniform vec2 u_resolution;
out vec4 outColor;

void main() {
    vec4 t = texture(u_texture, gl_FragCoord.xy / u_resolution);
    float luminance = (0.2126 * t.r + 0.7152 * t.g + 0.0722 * t.b);
    outColor = vec4(luminance, luminance, luminance, t.a);
}
