use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "shader")]
pub enum IoShader {
    Embed { frag: String, vert: String },
    File { frag: String, vert: String },
    // List presets here
    BlackWhite,
    GaussianBlur { radius: f32 },
}

pub fn resolve_resource_path(
    resource_path: &str,
    json_path: &std::path::Path,
) -> std::path::PathBuf {
    let parent_dir = json_path
        .parent()
        .unwrap_or_else(|| std::path::Path::new("."))
        .to_path_buf();
    let resolved = parent_dir.join(resource_path);
    println!("resolve {} = {}", resource_path, resolved.to_str().unwrap());
    resolved
}

// Return (vert, frag)
pub fn load_shader(
    item: &IoShader,
    json_path: &std::path::Path,
) -> (String, String, Vec<(String, f32)>) {
    match item {
        IoShader::Embed { frag, vert } => (frag.clone(), vert.clone(), vec![]),
        IoShader::File { frag, vert } => (
            std::fs::read_to_string(resolve_resource_path(vert, json_path)).unwrap(),
            std::fs::read_to_string(resolve_resource_path(frag, json_path)).unwrap(),
            vec![],
        ),
        IoShader::BlackWhite => (
            include_str!("./presets/blackwhite.vert").to_string(),
            include_str!("./presets/blackwhite.frag").to_string(),
            vec![],
        ),
        IoShader::GaussianBlur { radius } => (
            include_str!("./presets/gaussian_blur.vert").to_string(),
            include_str!("./presets/gaussian_blur.frag").to_string(),
            vec![("u_radius".to_string(), *radius)],
        ),
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum IoFilter {
    Image { path: String },
    Shader(IoShader),
}

#[derive(Default, Serialize, Deserialize)]
pub struct IoProject {
    pub filters: Vec<IoFilter>,
    pub width: u32,
    pub height: u32,
}
