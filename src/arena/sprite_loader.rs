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
                SpriteKinds::MonsterBirdBrown => Box::new(LargeEnemy::init(
                    render_context,
                    &SpriteFolderDescription::init_without_set(&folder, "$monster_bird1"),
                    1.5,
                )?),
                SpriteKinds::MonsterBirdBlue => Box::new(LargeEnemy::init(
                    render_context,
                    &SpriteFolderDescription::init_without_set(&folder, "$monster_bird2"),
                    1.5,
                )?),
                SpriteKinds::MonsterBirdRed => Box::new(LargeEnemy::init(
                    render_context,
                    &SpriteFolderDescription::init_without_set(&folder, "$monster_bird3"),
                    1.5,
                )?),
                SpriteKinds::SmallMonsterBirdBrown => Box::new(LargeEnemy::init(
                    render_context,
                    &SpriteFolderDescription::init_without_set(&folder, "$monster_bird1"),
                    1.0,
                )?),
                SpriteKinds::SmallMonsterBirdBlue => Box::new(LargeEnemy::init(
                    render_context,
                    &SpriteFolderDescription::init_without_set(&folder, "$monster_bird2"),
                    1.0,
                )?),
                SpriteKinds::SmallMonsterBirdRed => Box::new(LargeEnemy::init(
                    render_context,
                    &SpriteFolderDescription::init_without_set(&folder, "$monster_bird3"),
                    1.0,
                )?),
                SpriteKinds::FireBolt => Box::new(Bolt::init(render_context, &SpriteFolderDescription::init_without_set(&folder, "fire"), 0, 4)?),
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
                SpriteKinds::Smoke => Box::new(
                    Bolt::init(render_context, &SpriteFolderDescription::init_without_set(&folder, "smoke"), 6, 4)?.with_render_offset(bullet_offset()),
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
            };
            sprites.insert(s.into(), sprite);
        }
        Ok(sprites)
    }
}

fn bullet_offset() -> SDLPoint {
    SDLPoint::new(0, (TILE_SIZE / 2) as i32)
}
