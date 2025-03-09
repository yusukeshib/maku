use maku::{Maku, MakuError, NodeInput};

fn main() -> Result<(), MakuError> {
    let mut maku = Maku::new();
    let node1 = maku.add_node(NodeInput::Add { a: 2.0, b: 4.0 });
    maku.set_property_value((node1, "b").into(), 2.0)?;
    let node2 = maku.add_node(NodeInput::Multiply { a: 3.0, b: 5.0 });
    maku.link_properties((node1, "c").into(), (node2, "a").into())?;
    maku.remove_node(node1);
    Ok(())
}
