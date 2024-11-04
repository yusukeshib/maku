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
pub struct IoTransform {
    #[serde(default)]
    pub translate: [f32; 2],
    #[serde(default)]
    pub rotate: f32,
    #[serde(default)]
    pub scale: Scale,
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

// {
//   "nodes": [
//     {
//       "id": 1,
//       "type": "Image",
//       "path": { "type": "variable", "key": "input", "value": "./assets/input.png" }
//     },
//     {
//       "id": 2,
//       "type": "GaussianBlur",
//       "radius": { "type": "variable", "value": 16 },
//       "input": { "type": "link", "node": 1, "key": "output" }
//     },
//     {
//       "id": 3,
//       "type": "BlackWhite",
//       "input": { "type": "link", "node": 2, "key": "output" }
//     },
//     {
//       "id": 4,
//       "type": "Save",
//       "input": { "type": "link", "node": 3, "key": "output" },
//       "path": { "type": "variable", "key": "output", "value": "./assets/output.png" }
//     }
//   ],
//   "variables": {
//     "input": "assets/input.png",
//     "output": "assets/output.png"
//   }
// }

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum IoNode {
    Composition(IoComposition),
    Image(IoImage),
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
pub struct IoImage {
    pub path: String,
    #[serde(default)]
    pub transform: IoTransform,
}

#[derive(Default, Serialize, Deserialize)]
pub struct IoComposition {
    pub nodes: Vec<IoNode>,
    pub width: u32,
    pub height: u32,
    #[serde(default)]
    pub transform: IoTransform,
}
