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
        node.dependencies.iter().all(|d| state.skills.contains(d)) && node.cost <= state.experience
    }

    pub fn select(&self, state: &mut ProgressionState, name: &str) {
        if self.can_select(state, name) {
            let node = self.nodes.get(name).unwrap();

            state.skills.insert(name.to_string());
            state.experience -= node.cost;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_has_selected() {
        let state = ProgressionState::init(0, 0, &["Foo", "Bar"]);
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
    fn can_select_dependencies() {
        let mut state = ProgressionState::init(0, 0, &[]);
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

    #[test]
    fn can_select_cost() {
        let mut state = ProgressionState::init(0, 0, &[]);
        let tree = SkillTree::init(&[
            SkillTreeNode::init("Foo", Point::init(0, 0), 5, &[]),
            SkillTreeNode::init("Bazz", Point::init(0, 0), 10, &[]),
            SkillTreeNode::init("Bar", Point::init(0, 0), 5, &["Foo"]),
            SkillTreeNode::init("Buzz", Point::init(0, 0), 0, &["Bar"]),
        ]);
        assert_eq!(false, tree.can_select(&state, "Foo"));
        assert_eq!(false, tree.can_select(&state, "Bazz"));
        assert_eq!(false, tree.can_select(&state, "Bar"));
        assert_eq!(false, tree.can_select(&state, "Buzz"));

        state.experience = 5;
        assert!(tree.can_select(&state, "Foo"));
        assert_eq!(false, tree.can_select(&state, "Bazz"));
        assert_eq!(false, tree.can_select(&state, "Bar"));
        assert_eq!(false, tree.can_select(&state, "Buzz"));
    }

    #[test]
    fn select() {
        let mut state = ProgressionState::init(0, 7, &[]);
        let tree = SkillTree::init(&[
            SkillTreeNode::init("Foo", Point::init(0, 0), 5, &[]),
            SkillTreeNode::init("Bazz", Point::init(0, 0), 10, &[]),
            SkillTreeNode::init("Bar", Point::init(0, 0), 5, &["Foo"]),
            SkillTreeNode::init("Buzz", Point::init(0, 0), 0, &["Bar"]),
        ]);

        tree.select(&mut state, "Foo");
        assert_eq!(2, state.experience);
        assert!(state.skills.contains("Foo"));
    }
}
