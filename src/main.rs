type PropertyId = usize;
type NodeId = usize;

enum NodeInput {
    Add { a: f32, b: f32 },
    Multiply { a: f32, b: f32 },
}

enum Node {
    Add {
        a: PropertyId,
        b: PropertyId,
        c: PropertyId,
    },
    Multiply {
        a: PropertyId,
        b: PropertyId,
        c: PropertyId,
    },
}

enum PropertyValue {
    Float(f32),
    Int(i32),
}

enum Property {
    Value(PropertyValue),
    Link(PropertyId),
    Output(PropertyValue),
}

struct Maku {
    nodes: Vec<Option<Node>>,
    properties: Vec<Option<(NodeId, Property)>>,
}

impl Maku {
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            properties: vec![],
        }
    }

    pub fn add_node(&mut self, input: NodeInput) -> NodeId {
        let node_id: NodeId = self.nodes.len();
        match input {
            NodeInput::Add { a, b } => {
                let property_a = Property::Value(PropertyValue::Float(a));
                let property_b = Property::Value(PropertyValue::Float(b));
                let property_c = Property::Output(PropertyValue::Float(a + b));
                Node::Add {
                    a: self.add_property(node_id, property_a),
                    b: self.add_property(node_id, property_b),
                    c: self.add_property(node_id, property_c),
                }
            }
            NodeInput::Multiply { a, b } => {
                let property_a = Property::Value(PropertyValue::Float(a));
                let property_b = Property::Value(PropertyValue::Float(b));
                let property_c = Property::Output(PropertyValue::Float(a * b));
                Node::Multiply {
                    a: self.add_property(node_id, property_a),
                    b: self.add_property(node_id, property_b),
                    c: self.add_property(node_id, property_c),
                }
            }
        };
        node_id
    }

    fn add_property(&mut self, node_id: NodeId, property: Property) -> PropertyId {
        self.properties.push(Some((node_id, property)));
        let id: PropertyId = self.properties.len();
        id
    }
}

fn main() {
    let mut maku = Maku::new();
    maku.add_node(NodeInput::Add { a: 2.0, b: 4.0 });
    println!("Hello, world!");
}
