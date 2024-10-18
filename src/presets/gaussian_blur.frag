// https://stackoverflow.com/questions/64837705/opengl-blurring
uniform sampler2D u_texture;
uniform vec2 u_resolution;
uniform float u_radius;

out vec4 outColor;

void main() {
    float x, y, xx, yy, dx, dy, w;
    float rr = u_radius * u_radius;
    float w0 = 0.3780 / pow(u_radius, 1.975);
    vec2 p;
    vec2 pos0 = gl_FragCoord.xy / u_resolution;
    vec2 pos = (pos0 * 2.0) - 1.0;
    vec4 col = vec4(0.0, 0.0, 0.0, 0.0);

    if (u_radius == 0) {
        col += texture(u_texture, pos0);
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
                    col += texture(u_texture, p) * w;
                }
            }
        }
    }

    outColor = col;
}
