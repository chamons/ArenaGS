use std::collections::HashMap;

use super::{EquipmentItem, ProgressionState};
use crate::atlas::prelude::*;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct SkillTreeNode {
    pub item: EquipmentItem,
    pub position: Point,
    pub cost: u32,
    pub dependencies: Vec<String>,
}

impl SkillTreeNode {
    pub fn init(item: EquipmentItem, position: Point, cost: u32, dependencies: &[&str]) -> SkillTreeNode {
        SkillTreeNode {
            item,
            position,
            cost,
            dependencies: dependencies.iter().map(|d| d.to_string()).collect(),
        }
    }

    pub fn name(&self) -> &String {
        &self.item.name
    }

    pub fn image(&self) -> &Option<String> {
        &self.item.image
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkillNodeStatus {
    Selected,
    Available,
    Unavailable,
}

pub struct SkillTree {
    nodes: HashMap<String, SkillTreeNode>,
}

impl SkillTree {
    pub fn init(nodes: &[SkillTreeNode]) -> SkillTree {
        SkillTree {
            nodes: nodes.iter().map(|n| (n.name().clone(), n.clone())).collect(),
        }
    }

    pub fn icons(&self) -> Vec<String> {
        self.nodes.values().filter_map(|n| n.image().to_owned()).collect()
    }

    pub fn all(&self, state: &ProgressionState) -> Vec<(SkillTreeNode, SkillNodeStatus)> {
        self.nodes
            .values()
            .map(|n| {
                let status = if state.skills.contains(n.name()) {
                    SkillNodeStatus::Selected
                } else if self.can_select(state, &n.name()) {
                    SkillNodeStatus::Available
                } else {
                    SkillNodeStatus::Unavailable
                };
                (n.clone(), status)
            })
            .collect()
    }

    pub fn can_select(&self, state: &ProgressionState, name: &str) -> bool {
        let node = self.nodes.get(name).unwrap();
        !state.skills.contains(node.name()) && node.dependencies.iter().all(|d| state.skills.contains(d)) && node.cost <= state.experience
    }

    pub fn select(&self, state: &mut ProgressionState, name: &str) {
        if self.can_select(state, name) {
            let node = self.nodes.get(name).unwrap();

            state.skills.insert(name.to_string());
            state.experience -= node.cost;
        }
    }

    pub fn get(&self, name: &str) -> &EquipmentItem {
        &self.nodes.get(name).unwrap().item
    }
}

#[cfg(test)]
mod tests {
    use super::super::{CharacterWeaponKind, Equipment};
    use super::*;

    fn state_with_deps(experience: u32, deps: &[&str]) -> ProgressionState {
        ProgressionState::init(0, experience, deps, CharacterWeaponKind::Gunslinger, Equipment::init_empty())
    }

    fn eq(name: &str) -> EquipmentItem {
        EquipmentItem::init(name, None, EquipmentKinds::Weapon)
    }

    #[test]
    fn all_has_selected() {
        let state = state_with_deps(0, &["Foo", "Bar"]);
        let tree = SkillTree::init(&[
            SkillTreeNode::init(eq("Foo"), Point::init(0, 0), 0, &[]),
            SkillTreeNode::init(eq("Bar"), Point::init(0, 0), 0, &[]),
            SkillTreeNode::init(eq("Buzz"), Point::init(0, 0), 0, &[]),
            SkillTreeNode::init(eq("Moo"), Point::init(0, 0), 100, &[]),
        ]);
        let all = tree.all(&state);
        assert_eq!(SkillNodeStatus::Selected, all.iter().find(|&a| a.0.name() == "Foo").unwrap().1);
        assert_eq!(SkillNodeStatus::Selected, all.iter().find(|&a| a.0.name() == "Bar").unwrap().1);
        assert_eq!(SkillNodeStatus::Available, all.iter().find(|&a| a.0.name() == "Buzz").unwrap().1);
        assert_eq!(SkillNodeStatus::Unavailable, all.iter().find(|&a| a.0.name() == "Moo").unwrap().1);
    }

    #[test]
    fn can_select_dependencies() {
        let mut state = state_with_deps(0, &[]);
        let tree = SkillTree::init(&[
            SkillTreeNode::init(eq("Foo"), Point::init(0, 0), 0, &[]),
            SkillTreeNode::init(eq("Bar"), Point::init(0, 0), 0, &["Foo"]),
            SkillTreeNode::init(eq("Buzz"), Point::init(0, 0), 0, &["Bar"]),
        ]);
        assert!(tree.can_select(&state, "Foo"));
        assert_eq!(false, tree.can_select(&state, "Bar"));
        assert_eq!(false, tree.can_select(&state, "Buzz"));

        state.skills.insert("Foo".to_string());
        assert!(!tree.can_select(&state, "Foo"));
        assert!(tree.can_select(&state, "Bar"));
        assert_eq!(false, tree.can_select(&state, "Buzz"));
    }

    #[test]
    fn can_select_cost() {
        let mut state = state_with_deps(0, &[]);
        let tree = SkillTree::init(&[
            SkillTreeNode::init(eq("Foo"), Point::init(0, 0), 5, &[]),
            SkillTreeNode::init(eq("Bazz"), Point::init(0, 0), 10, &[]),
            SkillTreeNode::init(eq("Bar"), Point::init(0, 0), 5, &["Foo"]),
            SkillTreeNode::init(eq("Buzz"), Point::init(0, 0), 0, &["Bar"]),
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
        let mut state = state_with_deps(7, &[]);
        let tree = SkillTree::init(&[
            SkillTreeNode::init(eq("Foo"), Point::init(0, 0), 5, &[]),
            SkillTreeNode::init(eq("Bazz"), Point::init(0, 0), 10, &[]),
            SkillTreeNode::init(eq("Bar"), Point::init(0, 0), 5, &["Foo"]),
            SkillTreeNode::init(eq("Buzz"), Point::init(0, 0), 0, &["Bar"]),
        ]);

        tree.select(&mut state, "Foo");
        assert_eq!(2, state.experience);
        assert!(state.skills.contains("Foo"));
    }

    #[test]
    fn select_already_selected() {
        let mut state = state_with_deps(10, &[]);
        let tree = SkillTree::init(&[
            SkillTreeNode::init(eq("Foo"), Point::init(0, 0), 5, &[]),
            SkillTreeNode::init(eq("Bazz"), Point::init(0, 0), 10, &[]),
            SkillTreeNode::init(eq("Bar"), Point::init(0, 0), 5, &["Foo"]),
            SkillTreeNode::init(eq("Buzz"), Point::init(0, 0), 0, &["Bar"]),
        ]);

        tree.select(&mut state, "Foo");
        assert_eq!(5, state.experience);
        tree.select(&mut state, "Foo");
        assert_eq!(5, state.experience);
    }
}
