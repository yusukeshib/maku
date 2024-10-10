use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum IoShader {
    Embed { embed: String },
    Path { path: String },
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

pub fn load_shader(item: &IoShader, json_path: &std::path::Path) -> String {
    match item {
        IoShader::Embed { embed } => embed.clone(),
        IoShader::Path { path } => {
            let resolved_path = resolve_resource_path(path, json_path);
            std::fs::read_to_string(resolved_path).unwrap()
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum IoFilter {
    Image {
        path: String,
    },
    Shader {
        fragment: IoShader,
        vertex: IoShader,
    },
}

#[derive(Default, Serialize, Deserialize)]
pub struct IoProject {
    pub filters: Vec<IoFilter>,
    pub width: u32,
    pub height: u32,
}
