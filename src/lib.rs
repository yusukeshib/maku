use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MakuError {
    #[error("Image error")]
    Image(#[from] image::ImageError),
    #[error("Project loading error")]
    ProjectLoad(#[from] serde_json::Error),
    #[error("File loading error")]
    FileLoad(#[from] std::io::Error),
    #[error("Headless error")]
    Headless(#[from] three_d::HeadlessError),
}
// {
//   "nodes": [
//     {
//       "id": 1,
//       "type": "Image",
//       "path": { "type": "value", "key": "input", "value": "./assets/input.png" }
//     },
//     {
//       "id": 2,
//       "type": "GaussianBlur",
//       "radius": { "type": "value", "value": 16 },
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
//       "path": { "type": "value", "key": output", "value": "./assets/output.png" }
//     }
//   ],
//   // "values": {
//   //   "input": "assets/input.png",
//   //   "output": "assets/output.png"
//   // }
// }

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum IoNode {
    Math { a: f32, b: f32, op: u32 },
}

#[derive(Serialize, Deserialize)]
struct IoNodeWithId {
    id: usize,
    #[serde(flatten)]
    node: IoNode,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum IoValue {
    Variable { key: String, value: f32 },
    Link { node_id: usize, key: String },
}

pub type PropertyId = usize;
pub type NodeId = usize;

pub enum PropertyValue {
    Float(f32),
    UInt(u32),
}

impl From<u32> for PropertyValue {
    fn from(value: u32) -> Self {
        PropertyValue::UInt(value)
    }
}

impl From<f32> for PropertyValue {
    fn from(value: f32) -> Self {
        PropertyValue::Float(value)
    }
}

pub enum Property {
    Value(PropertyValue),
    Link(PropertyId),
}

pub enum Node {
    Math {
        a: PropertyId,
        b: PropertyId,
        op: PropertyId,
        output: PropertyId,
    },
}

#[derive(Default)]
pub struct Maku {
    nodes: HashMap<NodeId, Node>,
    properties: HashMap<PropertyId, Property>,
}

impl Maku {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_property<T>(&mut self, value: T) -> PropertyId
    where
        T: Into<PropertyValue>,
    {
        let id: PropertyId = self.properties.len();
        self.properties.insert(id, Property::Value(value.into()));
        id
    }

    pub fn remove_property(&mut self, _id: PropertyId) {
        todo!()
    }

    pub fn add_node(&mut self, io_node: &IoNode) {
        match io_node {
            IoNode::Math { a, b, op } => {
                let id: NodeId = self.nodes.len();
                let node = Node::Math {
                    a: self.add_property(*a),
                    b: self.add_property(*b),
                    op: self.add_property(*op),
                    output: self.add_property(0),
                };
                self.nodes.insert(id, node);
            }
        }
    }

    pub fn remove_node(&mut self, _node_id: NodeId) {
        todo!()
    }

    pub fn link_properties(&mut self, _p1: PropertyId, _p2: PropertyId) {
        todo!()
    }

    pub fn property(&self, id: &PropertyId) -> &Property {
        &self.properties[id]
    }

    pub fn update(&mut self) -> Result<(), MakuError> {
        // TODO: visited flag
        // for node in self.nodes.values() {
        //     // Apply each node
        //     match node {
        //         Node::Math { a, b, op, output } => {
        //             let tex_input = self.tex_value(input);
        //             let width = tex_input.width();
        //             let height = tex_input.height();
        //         }
        //     }
        // }

        Ok(())
    }

    pub fn render(&mut self) -> Result<(), MakuError> {
        todo!()
    }
}
