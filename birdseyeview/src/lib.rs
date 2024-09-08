/**
 * Subgraphs
 *  contains Subgraphs
 *      Connections to other subgraphs
 * Subgraphs can either be just a string
 * or a block
 */

#[derive(Debug)]
pub struct Node {
    pub id: String,
    pub parent_id: Option<String>,
    pub nodes: Option<Vec<Node>>,
}

#[derive(Debug)]
pub struct Diagram {
    pub nodes: Vec<Node>,
}

impl Diagram {
    pub fn render(self) -> String {
        let mut res = "flowchart TB".to_string();

        for ele in self.nodes {
            res.push_str("\n");
            res.push_str(&ele.render(1));
        }

        res
    }
}

impl Node {
    pub fn render(self, level: usize) -> String {
        let indentation = "\t".repeat(level);
        if self.nodes.is_none() {
            return indentation + self.id.as_str();
        }

        let mut res = indentation.clone() + "subgraph ";
        res.push_str(self.id.as_str());
        for ele in self.nodes.unwrap() {
            res.push_str("\n");
            res.push_str(&ele.render(level + 1));
        }
        res.push_str("\n");
        res.push_str(indentation.as_str());
        res.push_str("end");

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_diagram_renders() {
        let expected_result = "flowchart TB".to_string();
        let diagram = Diagram { nodes: vec![] };
        let result = diagram.render();
        assert_eq!(result, expected_result)
    }

    #[test]
    fn diagram_with_one_node_renders() {
        let expected_result = "flowchart TB\n\tnode1".to_string();
        let node_one = Node {
            id: "node1".to_string(),
            parent_id: None,
            nodes: None,
        };
        let diagram = Diagram {
            nodes: vec![node_one],
        };
        let result = diagram.render();
        assert_eq!(result, expected_result)
    }

    #[test]
    fn diagram_with_subgraph_renders() {
        let expected_result = "\
flowchart TB
\tsubgraph node1
\t\tnode2
\tend"
            .to_string();
        let inner_node = Node {
            id: "node2".to_string(),
            parent_id: None,
            nodes: None,
        };

        let node_one = Node {
            id: "node1".to_string(),
            parent_id: None,
            nodes: Some(vec![inner_node]),
        };
        let diagram = Diagram {
            nodes: vec![node_one],
        };
        let result = diagram.render();
        println!("{}", result);
        assert_eq!(result, expected_result)
    }

    #[test]
    fn diagram_with_multiple_subgraph_renders() {
        let expected_result = "\
flowchart TB
\tsubgraph node1
\t\tsubgraph node2
\t\t\tnode3
\t\tend
\tend"
            .to_string();
        let node_three = Node {
            id: "node3".to_string(),
            parent_id: None,
            nodes: None,
        };
        let node_two = Node {
            id: "node2".to_string(),
            parent_id: None,
            nodes: Some(vec![node_three]),
        };

        let node_one = Node {
            id: "node1".to_string(),
            parent_id: None,
            nodes: Some(vec![node_two]),
        };
        let diagram = Diagram {
            nodes: vec![node_one],
        };
        let result = diagram.render();
        println!("{}", result);
        assert_eq!(result, expected_result)
    }
}
