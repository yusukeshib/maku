use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum IoShader {
    Embed { embed: String },
    Path { path: String },
}

impl From<&IoShader> for String {
    fn from(item: &IoShader) -> String {
        match item {
            IoShader::Embed { embed } => embed.clone(),
            IoShader::Path { path } => std::fs::read_to_string(path).unwrap(),
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
