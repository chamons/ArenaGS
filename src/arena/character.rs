use crate::atlas::Point;

#[derive(Hash, PartialEq, Eq)]
#[allow(dead_code)]
pub enum CharacterStyle {
    MaleBrownHairBlueBody,
    MaleBlueHairRedBody,

    MonsterBirdBrown,
    MonsterBirdBlue,
    MonsterBirdRed,
}

#[derive(Hash, PartialEq, Eq)]
pub struct Character {
    pub id: u32,
    pub position: Point,
    pub style: CharacterStyle,
}

impl Character {
    pub fn init(id: u32, x: u32, y: u32, style: CharacterStyle) -> Character {
        Character {
            id,
            position: Point::init(x, y),
            style,
        }
    }
}
