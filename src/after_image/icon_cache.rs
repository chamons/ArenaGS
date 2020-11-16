use std::rc::Rc;

use sdl2::render::Texture;
use std::collections::HashMap;

use super::IconLoader;
use crate::after_image::prelude::*;
use crate::atlas::prelude::*;

pub struct IconCache {
    cache: HashMap<String, Rc<Texture>>,
}

impl IconCache {
    pub fn init<T: AsRef<str>>(render_context: &RenderContext, loader: IconLoader, names: &[T]) -> BoxResult<IconCache> {
        IconCache::init_with_alpha(render_context, loader, names, None)
    }

    // We force lower case here for now - https://github.com/chamons/ArenaGS/issues/263
    pub fn init_with_alpha<T: AsRef<str>>(render_context: &RenderContext, loader: IconLoader, names: &[T], alpha: Option<u8>) -> BoxResult<IconCache> {
        let mut cache = HashMap::new();
        for n in names {
            let mut image = loader.get(render_context, n.as_ref())?;
            if let Some(alpha) = alpha {
                image.set_alpha_mod(alpha);
            }
            cache.insert(n.as_ref().to_ascii_lowercase().to_string(), Rc::new(image));
        }
        Ok(IconCache { cache })
    }

    pub fn get(&self, name: &str) -> &Texture {
        &self.cache[&name.to_ascii_lowercase()]
    }

    pub fn get_reference(&self, name: &str) -> &Rc<Texture> {
        &self.cache[&name.to_ascii_lowercase()]
    }
}
