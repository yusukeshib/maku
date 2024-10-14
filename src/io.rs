use serde::{Deserialize, Serialize};

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
    item: &IoFilter,
    json_path: &std::path::Path,
) -> Option<(String, String, Vec<(String, f32)>)> {
    match item {
        IoFilter::Shader { frag, vert } => Some((
            std::fs::read_to_string(resolve_resource_path(vert, json_path)).unwrap(),
            std::fs::read_to_string(resolve_resource_path(frag, json_path)).unwrap(),
            vec![],
        )),
        IoFilter::BlackWhite => Some((
            include_str!("./presets/blackwhite.vert").to_string(),
            include_str!("./presets/blackwhite.frag").to_string(),
            vec![],
        )),
        IoFilter::GaussianBlur { radius } => Some((
            include_str!("./presets/gaussian_blur.vert").to_string(),
            include_str!("./presets/gaussian_blur.frag").to_string(),
            vec![("u_radius".to_string(), *radius)],
        )),
        IoFilter::Image { .. } => None,
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IoImageFit {
    ///  This is default. The image is resized to fill the given dimension. If necessary, the image will be stretched or squished to fit
    #[default]
    Fill,
    /// The image keeps its aspect ratio, but is resized to fit within the given dimension
    Contain,
    /// The image keeps its aspect ratio and fills the given dimension. The image will be clipped to fit
    Cover,
    /// The image is not resized
    None,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum IoFilter {
    Image {
        path: String,
        #[serde(default)]
        fit: IoImageFit,
    },
    Shader {
        frag: String,
        vert: String,
    },
    // List presets here
    BlackWhite,
    GaussianBlur {
        radius: f32,
    },
}

#[derive(Default, Serialize, Deserialize)]
pub struct IoProject {
    pub filters: Vec<IoFilter>,
    pub width: u32,
    pub height: u32,
}
