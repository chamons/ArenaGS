use sdl2::render::Texture;
use std::collections::HashMap;

use super::IconLoader;
use crate::after_image::prelude::*;
use crate::atlas::prelude::*;

pub struct IconCache {
    cache: HashMap<String, Texture>,
}

impl IconCache {
    pub fn init<T: AsRef<str>>(render_context: &RenderContext, loader: IconLoader, names: &[T]) -> BoxResult<IconCache> {
        IconCache::init_with_alpha(render_context, loader, names, None)
    }

    pub fn init_with_alpha<T: AsRef<str>>(render_context: &RenderContext, loader: IconLoader, names: &[T], alpha: Option<u8>) -> BoxResult<IconCache> {
        let mut cache = HashMap::new();
        for n in names {
            let mut image = loader.get(render_context, n.as_ref())?;
            if let Some(alpha) = alpha {
                image.set_alpha_mod(alpha);
            }
            cache.insert(n.as_ref().to_string(), image);
        }
        Ok(IconCache { cache })
    }

    pub fn get(&self, name: &str) -> &Texture {
        &self.cache[name]
    }
}
