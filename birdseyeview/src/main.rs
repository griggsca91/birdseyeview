use birdseyeview::{Diagram, Node};
fn main() {
    let s = Node {
        id: "Hello".to_string(),
        parent_id: None,
        nodes: None,
    };
    let diagram = Diagram { nodes: vec![s] };
    println!("{:?}", diagram)
}
