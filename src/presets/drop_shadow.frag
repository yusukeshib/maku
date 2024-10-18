// https://stackoverflow.com/questions/64837705/opengl-blurring
uniform sampler2D u_texture;
uniform vec2 u_resolution;
uniform float u_radius;
uniform vec2 u_offset;
uniform vec4 u_color;

out vec4 outColor;

void main() {
    float x, y, xx, yy, dx, dy, w;
    float rr = u_radius * u_radius;
    float w0 = 0.3780 / pow(u_radius, 1.975);
    vec2 p;
    vec2 pos0 = gl_FragCoord.xy / u_resolution;
    vec2 pos = (pos0 * 2.0) - 1.0;
    vec4 color = vec4(0.0, 0.0, 0.0, 0.0);
    vec2 offset = u_offset * vec2(1.0, -1.0) / u_resolution;

    if (u_radius == 0) {
        color += texture(u_texture, pos0 - offset);
    } else {
        for (
            dx = 1.0 / u_resolution.x, x = -u_radius, p.x = 0.5 + (pos.x * 0.5) + (x * dx);
            x <= u_radius;
            x++, p.x += dx
        ) {
            xx = x * x;
            for (
                dy = 1.0 / u_resolution.y, y = -u_radius, p.y = 0.5 + (pos.y * 0.5) + (y * dy);
                y <= u_radius;
                y++, p.y += dy
            ) {
                yy = y * y;
                if (xx + yy <= rr) {
                    w = w0 * exp((-xx - yy) / (2.0 * rr));
                    color += texture(u_texture, p - offset) * w;
                }
            }
        }
    }

    vec4 shadowColor = vec4(u_color.r, u_color.g, u_color.b, color.a);
    vec4 pixelColor = texture(u_texture, pos0);

    outColor = pixelColor * pixelColor.a + shadowColor * (1.0 - pixelColor.a);
}
