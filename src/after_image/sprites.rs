mod sprite;
pub use sprite::{Sprite, SpriteFolderDescription};

// One image
// Image Size (640 × 640)
// Blit Size (580, 595)
mod background;
pub use background::Background;

// Spritesheet (320 × x) (Always 5 across, variable height)
// Image Size (64x64)
// Blit Size (64x64)
mod bolt;
pub use bolt::Bolt;

// Many Images (for now https://github.com/chamons/ArenaGS/issues/206) (1 characters, all animation states)
// Image Size (48x48)
// Blit Size (96x96)
mod detailed_character;
pub use detailed_character::{CharacterAnimationState, DetailedCharacter};

// Spritesheet (3 by 4) (1 character, 4 directions)
// Image Size (122x122)
// Blit Size (122x96) (We cut off the shadow bottom for now https://github.com/chamons/ArenaGS/issues/207)
mod large_enemy;
pub use large_enemy::LargeEnemy;

// Spritesheet (12 by 8) (8 characters, 4 directions)
// Image Size (42x36)
// Blit Size (105x90) [2.5 zoom]
mod standard_character;
pub use standard_character::{StandardCharacter, StandardCharacterSize};
