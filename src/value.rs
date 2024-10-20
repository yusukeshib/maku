pub enum UniformValue {
    Float(f32),
    Vec2(three_d::Vec2),
    Vec3(three_d::Vec3),
    Vec4(three_d::Vec4),
    Mat3(three_d::Mat3),
}

impl UniformValue {
    pub fn apply(&self, program: &three_d::Program, name: &str) {
        match self {
            UniformValue::Float(v) => program.use_uniform(name, v),
            UniformValue::Vec2(v) => program.use_uniform(name, v),
            UniformValue::Vec3(v) => program.use_uniform(name, v),
            UniformValue::Vec4(v) => program.use_uniform(name, v),
            UniformValue::Mat3(v) => program.use_uniform(name, v),
        };
    }
}

impl From<f32> for UniformValue {
    fn from(value: f32) -> Self {
        Self::Float(value)
    }
}

impl From<(f32, f32)> for UniformValue {
    fn from(value: (f32, f32)) -> Self {
        Self::Vec2(three_d::Vector2::new(value.0, value.1))
    }
}

impl From<[f32; 3]> for UniformValue {
    fn from(value: [f32; 3]) -> Self {
        Self::Vec3(three_d::Vector3::new(value[0], value[1], value[2]))
    }
}

impl From<[f32; 4]> for UniformValue {
    fn from(value: [f32; 4]) -> Self {
        Self::Vec4(three_d::Vector4::new(
            value[0], value[1], value[2], value[3],
        ))
    }
}
