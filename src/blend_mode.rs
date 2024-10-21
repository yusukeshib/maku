use serde::{Deserialize, Serialize};

// https://godotshaders.com/snippet/blending-modes/

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BlendMode {
    #[default]
    Normal,
    Multiply,    // a * b
    Screen,      // 1 - (1 - a) * (1 - b)
    Darken,      // min(a, b)
    Lighten,     // max(a, b)
    Difference,  // abs(a - b)
    Exclusion,   // a + b - 2 * a * b
    Overlay,     // a < 0.5 ? (2.0 * a * b) : (1.0 - 2.0 * (1.0 - a) * (1.0 - b))
    HardLight,   // b < 0.5 ? (2.0 * a * b) : (1.0 - 2.0 * (1.0 - a) * (1.0 - b))
    SoftLight, // b < 0.5 ? (2.0 * a * b + a * a * (1.0 - 2.0 * b)) : (sqrt(a) * (2.0 * b - 1.0) + (2.0 * a) * (1.0 - b))
    ColorDodge, // a / (1.0 - b)
    LinearDodge, // a + b
    Burn,      // 1.0 - (1 - a) / b
    LinearBurn, // a + b - 1.0
}
