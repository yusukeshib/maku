pub enum UniformValue {
    Float(f32),
    Vector2(three_d::Vector2<f32>),
    Vector3(three_d::Vector3<f32>),
    Vector4(three_d::Vector4<f32>),
}

impl UniformValue {
    pub fn apply(&self, program: &three_d::Program, name: &str) {
        match self {
            UniformValue::Float(v) => program.use_uniform(name, v),
            UniformValue::Vector2(v) => program.use_uniform(name, v),
            UniformValue::Vector3(v) => program.use_uniform(name, v),
            UniformValue::Vector4(v) => program.use_uniform(name, v),
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
        Self::Vector2(three_d::Vector2::new(value.0, value.1))
    }
}

impl From<(f32, f32, f32)> for UniformValue {
    fn from(value: (f32, f32, f32)) -> Self {
        Self::Vector3(three_d::Vector3::new(value.0, value.1, value.2))
    }
}

impl From<(f32, f32, f32, f32)> for UniformValue {
    fn from(value: (f32, f32, f32, f32)) -> Self {
        Self::Vector4(three_d::Vector4::new(value.0, value.1, value.2, value.3))
    }
}
