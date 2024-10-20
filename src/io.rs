use serde::{Deserialize, Serialize};

pub fn resolve_resource_path(
    parent_dir: &std::path::Path,
    resource_path: &str,
) -> std::path::PathBuf {
    let parent_dir = parent_dir.to_path_buf();
    let resolved = parent_dir.join(resource_path);
    println!("resolve {} = {}", resource_path, resolved.to_str().unwrap());
    resolved
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum IoImageFit {
    ///  This is default. The image is resized to fill the given dimension. If necessary, the image will be stretched or squished to fit
    #[default]
    Fill,
    /// The image keeps its aspect ratio, but is resized to fit within the given dimension
    Contain,
    /// The image keeps its aspect ratio and fills the given dimension. The image will be clipped to fit
    Cover,
    /// The image is not resized
    None {
        #[serde(default)]
        translate: [f32; 2],
        #[serde(default)]
        rotate: f32,
        #[serde(default)]
        scale: Scale,
    },
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Scale([f32; 2]);

impl Default for Scale {
    fn default() -> Self {
        Self([1.0, 1.0])
    }
}

impl Scale {
    pub fn x(&self) -> f32 {
        self.0[0]
    }
    pub fn y(&self) -> f32 {
        self.0[1]
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum IoFilter {
    Composition(IoComposition),
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
    DropShadow {
        radius: f32,
        offset: [f32; 2],
        color: [f32; 4],
    },
}

#[derive(Default, Serialize, Deserialize)]
pub struct IoComposition {
    pub filters: Vec<IoFilter>,
    pub width: u32,
    pub height: u32,
    #[serde(default)]
    pub fit: IoImageFit,
}
