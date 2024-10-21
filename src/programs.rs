pub struct Programs {
    draw_texture: three_d::Program,
    blend_textures: three_d::Program,
}

impl Programs {
    pub fn new(context: &three_d::Context) -> Self {
        // For draw_texture textures
        let draw_texture = three_d::Program::from_source(
            context,
            "
                uniform mat3 u_matrix;
                in vec3 a_position;
                in vec2 a_uv;
                out vec2 v_uv;

                void main() {
                    gl_Position = vec4(u_matrix * a_position, 1.0);
                    v_uv = a_uv;
                }
            ",
            "
                uniform sampler2D u_texture;
                in vec2 v_uv;
                out vec4 outColor;

                void main() {
                    outColor = texture(u_texture, v_uv);
                    
                }
            ",
        )
        .unwrap();

        // For blend_textures textures
        let blend_textures = three_d::Program::from_source(
            context,
            "
                in vec4 a_position;
                in vec2 a_uv;
                out vec2 v_uv;

                void main() {
                    gl_Position = a_position;
                    v_uv = a_uv;
                }
            ",
            "
                uniform sampler2D u_texture1;
                uniform sampler2D u_texture2;
                in vec2 v_uv;
                out vec4 outColor;

                void main() {
                    vec4 c1 = texture(u_texture1, v_uv);
                    vec4 c2 = texture(u_texture2, v_uv);
                    outColor = c2 * c2.a + c1 * (1.0 - c2.a);
                }
            ",
        )
        .unwrap();

        Self {
            draw_texture,
            blend_textures,
        }
    }

    pub fn draw_texture(
        &self,
        context: &three_d::Context,
        texture: &three_d::Texture2D,
        matrix: three_d::Mat3,
        viewport: three_d::Viewport,
    ) {
        let sx = texture.width() as f32 / viewport.width as f32;
        let sy = texture.height() as f32 / viewport.height as f32;
        let geom = three_d::VertexBuffer::new_with_data(
            context,
            &[
                three_d::vec3(-sx, -sy, 0.0),
                three_d::vec3(-sx, sy, 0.0),
                three_d::vec3(sx, sy, 0.0),
                three_d::vec3(-sx, -sy, 0.0),
                three_d::vec3(sx, sy, 0.0),
                three_d::vec3(sx, -sy, 0.0),
            ],
        );
        let a_uv = three_d::VertexBuffer::new_with_data(
            context,
            &[
                three_d::vec2(0.0, 0.0),
                three_d::vec2(0.0, 1.0),
                three_d::vec2(1.0, 1.0),
                three_d::vec2(0.0, 0.0),
                three_d::vec2(1.0, 1.0),
                three_d::vec2(1.0, 0.0),
            ],
        );

        self.draw_texture.use_vertex_attribute("a_uv", &a_uv);
        self.draw_texture.use_vertex_attribute("a_position", &geom);
        self.draw_texture.use_uniform("u_matrix", matrix);
        self.draw_texture.use_texture("u_texture", texture);
        self.draw_texture.draw_arrays(
            three_d::RenderStates::default(),
            viewport,
            geom.vertex_count(),
        );
    }

    pub fn blend_textures(
        &self,
        context: &three_d::Context,
        texture1: &three_d::Texture2D,
        texture2: &three_d::Texture2D,
        viewport: three_d::Viewport,
    ) {
        let geom = three_d::VertexBuffer::new_with_data(
            context,
            &[
                three_d::vec3(-1.0, -1.0, 0.0),
                three_d::vec3(-1.0, 1.0, 0.0),
                three_d::vec3(1.0, 1.0, 0.0),
                three_d::vec3(-1.0, -1.0, 0.0),
                three_d::vec3(1.0, 1.0, 0.0),
                three_d::vec3(1.0, -1.0, 0.0),
            ],
        );
        let a_uv = three_d::VertexBuffer::new_with_data(
            context,
            &[
                three_d::vec2(0.0, 0.0),
                three_d::vec2(0.0, 1.0),
                three_d::vec2(1.0, 1.0),
                three_d::vec2(0.0, 0.0),
                three_d::vec2(1.0, 1.0),
                three_d::vec2(1.0, 0.0),
            ],
        );
        self.blend_textures.use_vertex_attribute("a_uv", &a_uv);
        self.blend_textures
            .use_vertex_attribute("a_position", &geom);
        self.blend_textures.use_texture("u_texture1", texture1);
        self.blend_textures.use_texture("u_texture2", texture2);
        self.blend_textures.draw_arrays(
            three_d::RenderStates::default(),
            viewport,
            geom.vertex_count(),
        );
    }
}
