pub struct Programs {
    copy: three_d::Program,
    blend: three_d::Program,
}

impl Programs {
    pub fn new(context: &three_d::Context) -> Self {
        // For copy textures
        let copy = three_d::Program::from_source(
            context,
            "
                in vec4 a_position;
                in vec4 a_uv;
                out vec2 v_uv;
                void main() {
                  gl_Position = a_position;
                  v_uv = a_uv.xy;
                }
            ",
            "
                uniform sampler2D u_texture;
                in vec2 v_uv;
                out vec4 outColor;
                void main() {
                  if(0.0 <= v_uv.x && v_uv.x <= 1.0 && 0.0 < v_uv.y && v_uv.y < 1.0) {
                    outColor = texture(u_texture, v_uv);
                  } else {
                    outColor = vec4(0.0);
                  }
                }
            ",
        )
        .unwrap();

        // For blend textures
        let blend = three_d::Program::from_source(
            context,
            "
                in vec4 a_position;
                in vec4 a_uv1;
                in vec4 a_uv2;
                out vec2 v_uv1;
                out vec2 v_uv2;
                void main() {
                  gl_Position = a_position;
                  v_uv1 = a_uv1.xy;
                  v_uv2 = a_uv2.xy;
                }
            ",
            "
                uniform sampler2D u_texture1;
                uniform sampler2D u_texture2;
                in vec2 v_uv1;
                in vec2 v_uv2;
                out vec4 outColor;
                void main() {
                  if(0.0 <= v_uv2.x && v_uv2.x < 1.0 && 0.0 <= v_uv2.y && v_uv2.y < 1.0) {
                    vec4 c1 = texture(u_texture1, v_uv1);
                    vec4 c2 = texture(u_texture2, v_uv2);
                    outColor = c2 * c2.a + c1 * (1.0 - c2.a);
                  } else {
                    outColor = texture(u_texture1, v_uv1);
                  }
                }
            ",
        )
        .unwrap();

        Self { copy, blend }
    }

    pub fn copy(&self, context: &three_d::Context, texture: &three_d::Texture2D) {
        let plane_positions = three_d::VertexBuffer::new_with_data(
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
        let plane_uv = three_d::VertexBuffer::new_with_data(
            context,
            &[
                three_d::vec3(0.0, 0.0, 0.0),
                three_d::vec3(0.0, 1.0, 0.0),
                three_d::vec3(1.0, 1.0, 0.0),
                three_d::vec3(0.0, 0.0, 0.0),
                three_d::vec3(1.0, 1.0, 0.0),
                three_d::vec3(1.0, 0.0, 0.0),
            ],
        );
        if self.copy.requires_attribute("a_uv") {
            self.copy.use_vertex_attribute("a_uv", &plane_uv);
        }
        if self.copy.requires_attribute("a_position") {
            self.copy
                .use_vertex_attribute("a_position", &plane_positions);
        }
        if self.copy.requires_uniform("u_texture") {
            self.copy.use_texture("u_texture", texture);
        }
        self.copy.draw_arrays(
            three_d::RenderStates::default(),
            three_d::Viewport::new_at_origo(texture.width(), texture.height()),
            plane_positions.vertex_count(),
        );
    }

    pub fn blend(
        &self,
        context: &three_d::Context,
        texture1: &three_d::Texture2D,
        texture2: &three_d::Texture2D,
        uv2: &three_d::VertexBuffer,
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
        let uv1 = three_d::VertexBuffer::new_with_data(
            context,
            &[
                three_d::vec3(0.0, 0.0, 0.0),
                three_d::vec3(0.0, 1.0, 0.0),
                three_d::vec3(1.0, 1.0, 0.0),
                three_d::vec3(0.0, 0.0, 0.0),
                three_d::vec3(1.0, 1.0, 0.0),
                three_d::vec3(1.0, 0.0, 0.0),
            ],
        );
        if self.blend.requires_attribute("a_uv1") {
            self.blend.use_vertex_attribute("a_uv1", &uv1);
        }
        if self.blend.requires_attribute("a_uv2") {
            self.blend.use_vertex_attribute("a_uv2", uv2);
        }
        if self.blend.requires_attribute("a_position") {
            self.blend.use_vertex_attribute("a_position", &geom);
        }
        if self.blend.requires_uniform("u_texture1") {
            self.blend.use_texture("u_texture1", texture1);
        }
        if self.blend.requires_uniform("u_texture2") {
            self.blend.use_texture("u_texture2", texture2);
        }
        self.blend.draw_arrays(
            three_d::RenderStates::default(),
            three_d::Viewport::new_at_origo(texture1.width(), texture1.height()),
            geom.vertex_count(),
        );
    }
}
