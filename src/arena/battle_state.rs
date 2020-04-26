use super::{Character, CharacterStyle};

pub struct BattleState {
    pub party: Vec<Character>,
}

impl BattleState {
    #[allow(dead_code)]
    pub fn init() -> BattleState {
        BattleState { party: vec![] }
    }

    pub fn test_state() -> BattleState {
        BattleState {
            party: vec![
                Character::init(0, 0, 0, CharacterStyle::MaleBrownHairBlueBody),
                Character::init(1, 5, 5, CharacterStyle::MaleBlueHairRedBody),
            ],
        }
    }
}
