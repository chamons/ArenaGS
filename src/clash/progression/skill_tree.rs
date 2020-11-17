use std::collections::HashMap;

use super::{EquipmentItem, EquipmentKinds, ProgressionState};
use crate::atlas::prelude::*;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum SkillTreeContents {
    Equipment(EquipmentItem),
    EquipmentExpansion(EquipmentKinds, u32),
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct SkillTreeNode {
    pub contents: SkillTreeContents,
    pub position: Point,
    pub cost: u32,
    pub dependencies: Vec<String>,
}

impl SkillTreeNode {
    pub fn with_equipment(equipment: EquipmentItem, position: Point, cost: u32, dependencies: &[&str]) -> SkillTreeNode {
        SkillTreeNode {
            contents: SkillTreeContents::Equipment(equipment),
            position,
            cost,
            dependencies: dependencies.iter().map(|d| d.to_string()).collect(),
        }
    }

    pub fn with_expansion(kind: EquipmentKinds, generation: u32, position: Point, cost: u32, dependencies: &[&str]) -> SkillTreeNode {
        SkillTreeNode {
            contents: SkillTreeContents::EquipmentExpansion(kind, generation),
            position,
            cost,
            dependencies: dependencies.iter().map(|d| d.to_string()).collect(),
        }
    }

    pub fn name(&self) -> String {
        match &self.contents {
            SkillTreeContents::Equipment(item) => item.name.to_string(),
            SkillTreeContents::EquipmentExpansion(kind, generation) => {
                let suffix = if *generation > 1 {
                    format!(" {}", roman::to(*generation as i32).unwrap())
                } else {
                    "".to_string()
                };
                format!("{:#?} Expansion{}", kind, suffix)
            }
        }
    }

    pub fn image(&self) -> Option<String> {
        match &self.contents {
            SkillTreeContents::Equipment(item) => item.image.clone(),
            SkillTreeContents::EquipmentExpansion(kind, _) => match kind {
                EquipmentKinds::Weapon => Some("SpellBook05_09.png".to_string()),
                EquipmentKinds::Armor => Some("SpellBook05_10.png".to_string()),
                EquipmentKinds::Accessory => Some("SpellBook05_59.png".to_string()),
                EquipmentKinds::Mastery => Some("SpellBook05_38.png".to_string()),
            },
        }
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
            nodes: nodes.iter().map(|n| (n.name(), n.clone())).collect(),
        }
    }

    pub fn icons(&self) -> Vec<String> {
        self.nodes.values().filter_map(|n| n.image()).collect()
    }

    pub fn all(&self, state: &ProgressionState) -> Vec<(SkillTreeNode, SkillNodeStatus)> {
        self.nodes
            .values()
            .map(|n| {
                let status = if state.has_unlock(&n.name()) {
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

    pub fn cost(&self, name: &str) -> u32 {
        self.nodes.get(name).unwrap().cost
    }

    pub fn can_select(&self, state: &ProgressionState, name: &str) -> bool {
        let node = self.nodes.get(name).unwrap();
        let not_already_selected = !state.has_unlock(&node.name());
        let dependencies_fulfilled = node.dependencies.iter().all(|d| state.has_unlock(d));
        let can_afford = node.cost <= state.influence;
        not_already_selected && dependencies_fulfilled && can_afford
    }

    pub fn select(&self, state: &mut ProgressionState, name: &str) {
        if self.can_select(state, name) {
            let node = self.nodes.get(name).unwrap();

            // We don't use sales APIs here since skill node cost != normal card cost
            state.influence -= node.cost;

            match node.contents {
                SkillTreeContents::Equipment(_) => {
                    state.items.insert(name.to_string());
                }
                SkillTreeContents::EquipmentExpansion(kind, _) => {
                    state.equipment_expansions.insert(name.to_string());
                    state.equipment.extend(kind);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::{CharacterWeaponKind, Equipment, EquipmentEffect, EquipmentKinds, EquipmentRarity};
    use super::*;

    fn state_with_deps(influence: u32, deps: &[&str]) -> ProgressionState {
        ProgressionState::init(0, influence, deps, CharacterWeaponKind::Gunslinger, Equipment::init_empty())
    }

    fn eq(name: &str) -> EquipmentItem {
        EquipmentItem::init(name, None, EquipmentKinds::Weapon, EquipmentRarity::Common, &[EquipmentEffect::None])
    }

    #[test]
    fn all_has_selected() {
        let state = state_with_deps(0, &["Foo", "Bar"]);
        let tree = SkillTree::init(&[
            SkillTreeNode::with_equipment(eq("Foo"), Point::init(0, 0), 0, &[]),
            SkillTreeNode::with_equipment(eq("Bar"), Point::init(0, 0), 0, &[]),
            SkillTreeNode::with_equipment(eq("Buzz"), Point::init(0, 0), 0, &[]),
            SkillTreeNode::with_equipment(eq("Moo"), Point::init(0, 0), 100, &[]),
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
            SkillTreeNode::with_equipment(eq("Foo"), Point::init(0, 0), 0, &[]),
            SkillTreeNode::with_equipment(eq("Bar"), Point::init(0, 0), 0, &["Foo"]),
            SkillTreeNode::with_equipment(eq("Buzz"), Point::init(0, 0), 0, &["Bar"]),
        ]);
        assert!(tree.can_select(&state, "Foo"));
        assert_eq!(false, tree.can_select(&state, "Bar"));
        assert_eq!(false, tree.can_select(&state, "Buzz"));

        state.items.insert("Foo".to_string());
        assert!(!tree.can_select(&state, "Foo"));
        assert!(tree.can_select(&state, "Bar"));
        assert_eq!(false, tree.can_select(&state, "Buzz"));
    }

    #[test]
    fn can_select_cost() {
        let mut state = state_with_deps(0, &[]);
        let tree = SkillTree::init(&[
            SkillTreeNode::with_equipment(eq("Foo"), Point::init(0, 0), 5, &[]),
            SkillTreeNode::with_equipment(eq("Bazz"), Point::init(0, 0), 10, &[]),
            SkillTreeNode::with_equipment(eq("Bar"), Point::init(0, 0), 5, &["Foo"]),
            SkillTreeNode::with_equipment(eq("Buzz"), Point::init(0, 0), 0, &["Bar"]),
        ]);
        assert_eq!(false, tree.can_select(&state, "Foo"));
        assert_eq!(false, tree.can_select(&state, "Bazz"));
        assert_eq!(false, tree.can_select(&state, "Bar"));
        assert_eq!(false, tree.can_select(&state, "Buzz"));

        state.influence = 5;
        assert!(tree.can_select(&state, "Foo"));
        assert_eq!(false, tree.can_select(&state, "Bazz"));
        assert_eq!(false, tree.can_select(&state, "Bar"));
        assert_eq!(false, tree.can_select(&state, "Buzz"));
    }

    #[test]
    fn select() {
        let mut state = state_with_deps(7, &[]);
        let tree = SkillTree::init(&[
            SkillTreeNode::with_equipment(eq("Foo"), Point::init(0, 0), 5, &[]),
            SkillTreeNode::with_equipment(eq("Bazz"), Point::init(0, 0), 10, &[]),
            SkillTreeNode::with_equipment(eq("Bar"), Point::init(0, 0), 5, &["Foo"]),
            SkillTreeNode::with_equipment(eq("Buzz"), Point::init(0, 0), 0, &["Bar"]),
        ]);

        tree.select(&mut state, "Foo");
        assert_eq!(2, state.influence);
        assert!(state.items.contains("Foo"));
    }

    #[test]
    fn select_already_selected() {
        let mut state = state_with_deps(10, &[]);
        let tree = SkillTree::init(&[
            SkillTreeNode::with_equipment(eq("Foo"), Point::init(0, 0), 5, &[]),
            SkillTreeNode::with_equipment(eq("Bazz"), Point::init(0, 0), 10, &[]),
            SkillTreeNode::with_equipment(eq("Bar"), Point::init(0, 0), 5, &["Foo"]),
            SkillTreeNode::with_equipment(eq("Buzz"), Point::init(0, 0), 0, &["Bar"]),
        ]);

        tree.select(&mut state, "Foo");
        assert_eq!(5, state.influence);
        tree.select(&mut state, "Foo");
        assert_eq!(5, state.influence);
    }

    #[test]
    fn select_expansion() {
        let mut state = state_with_deps(10, &[]);
        let tree = SkillTree::init(&[
            SkillTreeNode::with_expansion(EquipmentKinds::Armor, 1, Point::init(0, 0), 5, &[]),
            SkillTreeNode::with_equipment(eq("Bazz"), Point::init(0, 0), 5, &["Armor Expansion"]),
        ]);

        tree.select(&mut state, "Armor Expansion");
        assert_eq!(1, state.equipment.count(EquipmentKinds::Armor));
        assert!(tree.can_select(&state, "Bazz"));
    }

    #[test]
    fn select_multiple_expansions() {
        let mut state = state_with_deps(15, &[]);
        let tree = SkillTree::init(&[
            SkillTreeNode::with_expansion(EquipmentKinds::Armor, 1, Point::init(0, 0), 5, &[]),
            SkillTreeNode::with_expansion(EquipmentKinds::Armor, 2, Point::init(0, 0), 5, &["Armor Expansion"]),
            SkillTreeNode::with_expansion(EquipmentKinds::Armor, 3, Point::init(0, 0), 5, &["Armor Expansion II"]),
        ]);

        tree.select(&mut state, "Armor Expansion");
        assert_eq!(1, state.equipment.count(EquipmentKinds::Armor));
        assert!(tree.can_select(&state, "Armor Expansion II"));

        tree.select(&mut state, "Armor Expansion II");
        assert_eq!(2, state.equipment.count(EquipmentKinds::Armor));
        assert!(tree.can_select(&state, "Armor Expansion III"));

        tree.select(&mut state, "Armor Expansion III");
        assert_eq!(3, state.equipment.count(EquipmentKinds::Armor));
    }
}
