use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::PathBuf;

use anyhow::Result;
use ggez::graphics::Image;
use ggez::Context;

pub struct ImageCache {
    images: HashMap<String, ggez::graphics::Image>,
}

impl ImageCache {
    pub fn load(ctx: &mut Context, dir: PathBuf) -> Result<Self> {
        let mut images = HashMap::new();
        load_images(ctx, dir, &mut images)?;
        Ok(ImageCache { images })
    }

    pub fn get(&self, name: &str) -> &Image {
        match self.images.get(name) {
            Some(image) => image,
            None => panic!("Unable to find image {} in cache", name),
        }
    }
}

fn load_images(ctx: &mut Context, dir: PathBuf, images: &mut HashMap<String, ggez::graphics::Image>) -> Result<()> {
    for item in ctx.fs.read_dir(dir)? {
        if ctx.fs.is_file(&item) {
            if let Some(extension) = item.extension().and_then(OsStr::to_str).map(|s| s.to_lowercase()) {
                if extension.as_str() == "png" {
                    let image = ggez::graphics::Image::from_path(ctx, &item)?;
                    let key = item.to_str().unwrap().replace('\\', "/");
                    images.insert(key, image);
                }
            }
        } else {
            load_images(ctx, item, images)?;
        }
    }
    Ok(())
}
