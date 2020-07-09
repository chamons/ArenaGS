use std::collections::HashMap;
use std::path::Path;

use enum_iterator::IntoEnumIterator;

use super::SpriteKinds;
use crate::after_image::{Background, DetailedCharacter, LargeEnemy, RenderContext, Sprite, SpriteFolderDescription};
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

    pub fn get(&self, id: u32) -> &Box<dyn Sprite> {
        &self.sprite_cache[&id]
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
                )?),
                SpriteKinds::MonsterBirdBlue => Box::new(LargeEnemy::init(
                    render_context,
                    &SpriteFolderDescription::init_without_set(&folder, "$monster_bird2"),
                )?),
                SpriteKinds::MonsterBirdRed => Box::new(LargeEnemy::init(
                    render_context,
                    &SpriteFolderDescription::init_without_set(&folder, "$monster_bird3"),
                )?),
            };
            sprites.insert(s.into(), sprite);
        }
        Ok(sprites)
    }
}