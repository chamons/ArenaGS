use super::{Character, CharacterStyle};

pub struct BattleState {
    pub party: Vec<Character>,
    pub enemy: Character,
}

impl BattleState {
    #[allow(dead_code)]
    pub fn init() -> BattleState {
        BattleState {
            party: vec![],
            enemy: Character::init(0, 0, 0, CharacterStyle::MonsterBirdBrown),
        }
    }

    pub fn test_state() -> BattleState {
        BattleState {
            party: vec![
                Character::init(0, 0, 0, CharacterStyle::MaleBrownHairBlueBody),
                Character::init(1, 5, 5, CharacterStyle::MaleBlueHairRedBody),
            ],
            enemy: Character::init(2, 3, 3, CharacterStyle::MonsterBirdBrown),
        }
    }
}
