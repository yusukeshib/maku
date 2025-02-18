use std::collections::HashMap;
use std::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MakuError {
    #[error("Invalid property_id={0}")]
    InvalidPropertyId(PropertyId),
}

type NodeId = usize;

// Ex. (NodeID=12, Key="a")
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct PropertyId {
    pub node_id: NodeId,
    pub key: String,
}

impl From<(NodeId, &str)> for PropertyId {
    fn from(value: (NodeId, &str)) -> Self {
        Self {
            node_id: value.0,
            key: value.1.to_string(),
        }
    }
}

impl fmt::Display for PropertyId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PropertyId(node_id={}, key={})", self.node_id, self.key)
    }
}

pub enum NodeInput {
    Add { a: f32, b: f32 },
    Multiply { a: f32, b: f32 },
}

enum NodeType {
    Add,
    Multiply,
}

struct Node {
    pub ty: NodeType,
    pub properties: Vec<PropertyId>,
}

#[allow(dead_code)]
enum PropertyValue {
    Float(f32),
    Int(i32),
}

impl From<f32> for PropertyValue {
    fn from(value: f32) -> Self {
        Self::Float(value)
    }
}

impl From<i32> for PropertyValue {
    fn from(value: i32) -> Self {
        Self::Int(value)
    }
}

#[allow(dead_code)]
enum Property {
    Value(PropertyValue),
    Link(PropertyId),
    Output,
}

pub struct Maku {
    nodes: Vec<Option<Node>>,
    properties: HashMap<PropertyId, Property>,
}

impl Maku {
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            properties: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, input: NodeInput) -> NodeId {
        let node_id: NodeId = self.nodes.len();
        let node = match input {
            NodeInput::Add { a, b } => Node {
                ty: NodeType::Add,
                properties: vec![
                    self.add_property(node_id, "a", Property::Value(a.into())),
                    self.add_property(node_id, "b", Property::Value(b.into())),
                    self.add_property(node_id, "c", Property::Output),
                ],
            },
            NodeInput::Multiply { a, b } => Node {
                ty: NodeType::Multiply,
                properties: vec![
                    self.add_property(node_id, "a", Property::Value(a.into())),
                    self.add_property(node_id, "b", Property::Value(b.into())),
                    self.add_property(node_id, "c", Property::Output),
                ],
            },
        };
        self.nodes.push(Some(node));
        node_id
    }

    pub fn remove_node(&mut self, node_id: NodeId) {
        let node = &self.nodes[node_id];
        if let Some(node) = node {
            for property_id in &node.properties {
                self.properties.remove(property_id);
            }
            // TODO: Remove links too
            self.nodes[node_id] = None;
        }
    }

    // Link id1 to id2
    pub fn link_properties(&mut self, id1: PropertyId, id2: PropertyId) -> Result<(), MakuError> {
        let p2 = self
            .properties
            .get_mut(&id2)
            .ok_or(MakuError::InvalidPropertyId(id2))?;
        // TODO: Output isn't allowed to be
        // TODO: Check property types
        *p2 = Property::Link(id1);
        Ok(())
    }

    pub fn set_property_value<T>(&mut self, id: PropertyId, value: T) -> Result<(), MakuError>
    where
        T: Into<PropertyValue>,
    {
        let p = self
            .properties
            .get_mut(&id)
            .ok_or(MakuError::InvalidPropertyId(id))?;
        *p = Property::Value(value.into());
        Ok(())
    }

    fn add_property(&mut self, node_id: NodeId, key: &str, property: Property) -> PropertyId {
        let property_id: PropertyId = (node_id, key).into();
        self.properties.insert(property_id.clone(), property);
        property_id
    }
}
