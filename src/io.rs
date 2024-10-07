use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum IoShader {
    Embed { embed: String },
    Path { path: String },
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

#[derive(Serialize, Deserialize)]
pub struct IoProject {
    pub filters: Vec<IoFilter>,
}
