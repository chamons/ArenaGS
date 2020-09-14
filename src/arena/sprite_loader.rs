use std::collections::HashMap;
use std::path::Path;

use enum_iterator::IntoEnumIterator;
use sdl2::rect::Point as SDLPoint;

use super::components::*;
use super::views::TILE_SIZE;
use crate::after_image::*;
use crate::atlas::BoxResult;

pub struct SpriteLoader {
    sprite_cache: HashMap<u32, Box<dyn Sprite>>,
}

impl SpriteLoader {
    pub fn init(render_context: &RenderContext) -> BoxResult<SpriteLoader> {
        Ok(SpriteLoader {
            sprite_cache: SpriteLoader::load_sprites(render_context)?,
        })
    }

    pub fn get(&self, id: u32) -> &dyn Sprite {
        // Done so we don't borrow a Box<dyn Sprite>
        &*self.sprite_cache[&id]
    }

    fn load_sprites(render_context: &RenderContext) -> BoxResult<HashMap<u32, Box<dyn Sprite>>> {
        let folder = Path::new("images");

        let mut sprites: HashMap<u32, Box<dyn Sprite>> = HashMap::new();
        for s in SpriteKinds::into_enum_iter() {
            let sprite: Box<dyn Sprite> = match s {
                SpriteKinds::BeachBackground => Box::new(Background::init(render_context, "beach")?),
                SpriteKinds::MaleBrownHairBlueBody => Box::new(DetailedCharacter::init(render_context, &SpriteFolderDescription::init(&folder, "1", "1"))?),
                SpriteKinds::MaleBlueHairRedBody => Box::new(DetailedCharacter::init(render_context, &SpriteFolderDescription::init(&folder, "1", "1"))?),
                SpriteKinds::SimpleGolem => Box::new(LargeEnemy::init(
                    render_context,
                    &SpriteFolderDescription::init_without_set(&folder, "$monster_golem1"),
                    LargeCharacterSize::Normal,
                )?),
                SpriteKinds::MonsterBirdBrown => Box::new(LargeEnemy::init(
                    render_context,
                    &SpriteFolderDescription::init_without_set(&folder, "$monster_bird1"),
                    LargeCharacterSize::LargeBird,
                )?),
                SpriteKinds::MonsterBirdBlue => Box::new(LargeEnemy::init(
                    render_context,
                    &SpriteFolderDescription::init_without_set(&folder, "$monster_bird2"),
                    LargeCharacterSize::LargeBird,
                )?),
                SpriteKinds::MonsterBirdRed => Box::new(LargeEnemy::init(
                    render_context,
                    &SpriteFolderDescription::init_without_set(&folder, "$monster_bird3"),
                    LargeCharacterSize::LargeBird,
                )?),
                SpriteKinds::SmallMonsterBirdBrown => Box::new(LargeEnemy::init(
                    render_context,
                    &SpriteFolderDescription::init_without_set(&folder, "$monster_bird1"),
                    LargeCharacterSize::Bird,
                )?),
                SpriteKinds::SmallMonsterBirdBlue => Box::new(LargeEnemy::init(
                    render_context,
                    &SpriteFolderDescription::init_without_set(&folder, "$monster_bird2"),
                    LargeCharacterSize::Bird,
                )?),
                SpriteKinds::SmallMonsterBirdRed => Box::new(LargeEnemy::init(
                    render_context,
                    &SpriteFolderDescription::init_without_set(&folder, "$monster_bird3"),
                    LargeCharacterSize::Bird,
                )?),
                SpriteKinds::NoBolt => Box::new(Bolt::init(
                    render_context,
                    &SpriteFolderDescription::init_without_set(&folder, "lightning"),
                    30, // Just an empty part of the sheet
                    1,
                )?),
                SpriteKinds::FireBolt => Box::new(Bolt::init(render_context, &SpriteFolderDescription::init_without_set(&folder, "fire"), 0, 4)?),
                SpriteKinds::WaterBolt => Box::new(Bolt::init(render_context, &SpriteFolderDescription::init_without_set(&folder, "water"), 44, 3)?),
                SpriteKinds::LightningOrb => Box::new(Bolt::init(
                    render_context,
                    &SpriteFolderDescription::init_without_set(&folder, "lightning"),
                    13,
                    4,
                )?),
                SpriteKinds::Bullet => Box::new(
                    Bolt::init(render_context, &SpriteFolderDescription::init_without_set(&folder, "weapons_2"), 6, 1)?.with_render_offset(bullet_offset()),
                ),
                SpriteKinds::FireBullet => {
                    Box::new(Bolt::init(render_context, &SpriteFolderDescription::init_without_set(&folder, "fire"), 0, 4)?.with_render_offset(bullet_offset()))
                }
                SpriteKinds::AirBullet => Box::new(
                    Bolt::init(render_context, &SpriteFolderDescription::init_without_set(&folder, "weapons_2"), 17, 2)?.with_render_offset(bullet_offset()),
                ),
                SpriteKinds::Bomb => Box::new(Bolt::init(render_context, &SpriteFolderDescription::init_without_set(&folder, "object"), 4, 2)?),
                SpriteKinds::Explosion => Box::new(
                    Bolt::init(render_context, &SpriteFolderDescription::init_without_set(&folder, "explosion"), 11, 8)?
                        .with_render_offset(SDLPoint::new(0, 25))
                        .with_scale(2.0),
                ),
                SpriteKinds::LightningStrike => {
                    Box::new(Bolt::init(render_context, &SpriteFolderDescription::init_without_set(&folder, "lightning"), 0, 6)?.with_scale(2.5))
                }
                SpriteKinds::Cloud => Box::new(Bolt::init(render_context, &SpriteFolderDescription::init_without_set(&folder, "smoke"), 0, 6)?.with_scale(2.0)),
                SpriteKinds::FireColumn => {
                    Box::new(Bolt::init(render_context, &SpriteFolderDescription::init_without_set(&folder, "fire"), 16, 5)?.with_scale(2.0))
                }
                SpriteKinds::WaterColumn => {
                    Box::new(Bolt::init(render_context, &SpriteFolderDescription::init_without_set(&folder, "water"), 30, 3)?.with_scale(2.0))
                }
                SpriteKinds::EarthColumn => {
                    Box::new(Bolt::init(render_context, &SpriteFolderDescription::init_without_set(&folder, "earth1"), 12, 6)?.with_scale(2.0))
                }
                SpriteKinds::Smoke => Box::new(
                    Bolt::init(render_context, &SpriteFolderDescription::init_without_set(&folder, "smoke"), 6, 4)?
                        .with_render_offset(bullet_offset())
                        .with_scale(2.0),
                ),
                SpriteKinds::Egg => Box::new(StandardCharacter::init(
                    render_context,
                    &SpriteFolderDescription::init_without_set(&folder, "birds1"),
                    9,
                    StandardCharacterSize::Micro,
                )?),
                SpriteKinds::Elementalist => Box::new(StandardCharacter::init(
                    render_context,
                    &SpriteFolderDescription::init_without_set(&folder, "npc6"),
                    9,
                    StandardCharacterSize::Normal,
                )?),
                SpriteKinds::WaterElemental => Box::new(StandardCharacter::init(
                    render_context,
                    &SpriteFolderDescription::init_without_set(&folder, "elemental"),
                    0,
                    StandardCharacterSize::Large,
                )?),
                SpriteKinds::FireElemental => Box::new(StandardCharacter::init(
                    render_context,
                    &SpriteFolderDescription::init_without_set(&folder, "elemental"),
                    3,
                    StandardCharacterSize::Large,
                )?),
                SpriteKinds::WindElemental => Box::new(StandardCharacter::init(
                    render_context,
                    &SpriteFolderDescription::init_without_set(&folder, "elemental"),
                    6,
                    StandardCharacterSize::Large,
                )?),
                SpriteKinds::EarthElemental => Box::new(StandardCharacter::init(
                    render_context,
                    &SpriteFolderDescription::init_without_set(&folder, "elemental"),
                    9,
                    StandardCharacterSize::Large,
                )?),
            };
            sprites.insert(s.into(), sprite);
        }
        Ok(sprites)
    }
}

fn bullet_offset() -> SDLPoint {
    SDLPoint::new(0, (TILE_SIZE / 2) as i32)
}
