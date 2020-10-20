use std::collections::HashMap;

use super::ProgressionState;
use crate::atlas::prelude::*;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct SkillTreeNode {
    name: String,
    position: Point,
    cost: u32,
    dependencies: Vec<String>,
}

impl SkillTreeNode {
    pub fn init(name: &str, position: Point, cost: u32, dependencies: &[&str]) -> SkillTreeNode {
        SkillTreeNode {
            name: name.to_string(),
            position,
            cost,
            dependencies: dependencies.iter().map(|d| d.to_string()).collect(),
        }
    }
}

struct SkillTree {
    nodes: HashMap<String, SkillTreeNode>,
}

impl SkillTree {
    pub fn init(nodes: &[SkillTreeNode]) -> SkillTree {
        SkillTree {
            nodes: nodes.iter().map(|n| (n.name.clone(), n.clone())).collect(),
        }
    }

    pub fn all(&self, state: &ProgressionState) -> Vec<(SkillTreeNode, bool)> {
        self.nodes.values().map(|n| (n.clone(), state.skills.contains(&n.name))).collect()
    }

    pub fn can_select(&self, state: &ProgressionState, name: &str) -> bool {
        let node = self.nodes.get(name).unwrap();
        node.dependencies.iter().all(|d| state.skills.contains(d))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_has_selected() {
        let state = ProgressionState::init(0, &["Foo", "Bar"]);
        let tree = SkillTree::init(&[
            SkillTreeNode::init("Foo", Point::init(0, 0), 0, &[]),
            SkillTreeNode::init("Bar", Point::init(0, 0), 0, &[]),
            SkillTreeNode::init("Buzz", Point::init(0, 0), 0, &[]),
        ]);
        let all = tree.all(&state);
        assert!(all.iter().find(|&a| a.0.name == "Foo").unwrap().1);
        assert!(all.iter().find(|&a| a.0.name == "Bar").unwrap().1);
        assert_eq!(false, all.iter().find(|&a| a.0.name == "Buzz").unwrap().1);
    }

    #[test]
    fn can_select() {
        let mut state = ProgressionState::init(0, &[]);
        let tree = SkillTree::init(&[
            SkillTreeNode::init("Foo", Point::init(0, 0), 0, &[]),
            SkillTreeNode::init("Bar", Point::init(0, 0), 0, &["Foo"]),
            SkillTreeNode::init("Buzz", Point::init(0, 0), 0, &["Bar"]),
        ]);
        assert!(tree.can_select(&state, "Foo"));
        assert_eq!(false, tree.can_select(&state, "Bar"));
        assert_eq!(false, tree.can_select(&state, "Buzz"));

        state.skills.insert("Foo".to_string());
        assert!(tree.can_select(&state, "Foo"));
        assert!(tree.can_select(&state, "Bar"));
        assert_eq!(false, tree.can_select(&state, "Buzz"));
    }
}
