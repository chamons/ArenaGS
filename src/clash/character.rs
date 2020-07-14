pub struct Health {
    pub current: u32,
    pub max: u32,
}

impl Health {
    pub fn init(current: u32, max: u32) -> Health {
        Health { current, max }
    }
}

pub struct Character {
    pub health: Health,
}

impl Character {
    pub fn init() -> Character {
        Character { health: Health::init(8, 10) }
    }
}
