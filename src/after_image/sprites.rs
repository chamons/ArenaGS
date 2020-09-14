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
// Normal
//    Image Size (94x100)
//    Blit Size (94x100) (We cut off the shadow bottom for now https://github.com/chamons/ArenaGS/issues/207)
// Bird
//    Image Size (122x122)
//    Blit Size (122x96) (We cut off the shadow bottom for now https://github.com/chamons/ArenaGS/issues/207)
// LargeBird
//    Image Size (122x122)
//    Blit Size (122x96) (zoom 1.5) (We cut off the shadow bottom for now https://github.com/chamons/ArenaGS/issues/207)
mod large_enemy;
pub use large_enemy::{LargeCharacterSize, LargeEnemy};

// Spritesheet (12 by 8) (8 characters, 4 directions)
// Micro Size:
//    Image Size (42x36)
//    Blit Size (105x90) [2.5 zoom]
// Standard Size:
//    Image Size (52x72)
//    Blit Size (52x72)
mod standard_character;
pub use standard_character::{StandardCharacter, StandardCharacterSize};
