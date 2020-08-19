use sdl2::render::Texture;
use std::collections::HashMap;

use super::IconLoader;
use crate::after_image::RenderContext;
use crate::atlas::BoxResult;

pub struct IconCache {
    cache: HashMap<String, Texture>,
}

impl IconCache {
    pub fn init(render_context: &RenderContext, loader: IconLoader, names: &[&str]) -> BoxResult<IconCache> {
        let mut cache = HashMap::new();
        for n in names {
            cache.insert(n.to_string(), loader.get(render_context, n)?);
        }
        Ok(IconCache { cache })
    }

    pub fn get(&self, name: &str) -> &Texture {
        &self.cache[name]
    }
}
