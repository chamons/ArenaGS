use sdl2::rect::Rect as SDLRect;

use crate::after_image::*;
use crate::atlas::prelude::*;
use crate::props::HitTestResult;

pub struct RenderTextOptions<'a> {
    icons: Option<&'a IconCache>,
    color: FontColor,
    y_offset: i32,
    underline_links: bool,
    centered: Option<u32>,
}

impl<'a> RenderTextOptions<'a> {
    pub fn init(color: FontColor) -> RenderTextOptions<'a> {
        RenderTextOptions {
            color,
            icons: None,
            y_offset: 0,
            underline_links: false,
            centered: None,
        }
    }

    pub fn with_icons(mut self, icons: &'a IconCache) -> RenderTextOptions<'a> {
        self.icons = Some(icons);
        self
    }

    pub fn with_offset(mut self, y_offset: i32) -> RenderTextOptions<'a> {
        self.y_offset = y_offset;
        self
    }

    pub fn with_underline_links(mut self, underline_links: bool) -> RenderTextOptions<'a> {
        self.underline_links = underline_links;
        self
    }

    // In simple cases, without any links or inline images we can center text
    pub fn with_centered(mut self, centered: Option<u32>) -> RenderTextOptions<'a> {
        self.centered = centered;
        self
    }
}

pub fn render_text_layout<'a>(
    layout: &'a LayoutResult,
    canvas: &mut RenderCanvas,
    text: &TextRenderer,
    options: RenderTextOptions,
    mut on_hittest_text: impl FnMut(SDLRect, HitTestResult) + 'a,
) -> BoxResult<()> {
    for chunk in &layout.chunks {
        match &chunk.value {
            LayoutChunkValue::String(s) => {
                let (size, y_font_offset) = if chunk.attributes.contains(LayoutChunkAttributes::SMALLER_TEXT) {
                    (FontSize::VeryTiny, 2)
                } else {
                    (FontSize::Small, 0)
                };

                if let Some(width) = options.centered {
                    text.render_text_centered(
                        &s,
                        chunk.position.x as i32,
                        y_font_offset + options.y_offset + chunk.position.y as i32,
                        width,
                        canvas,
                        size,
                        options.color,
                    )?;
                } else {
                    text.render_text(
                        &s,
                        chunk.position.x as i32,
                        y_font_offset + options.y_offset + chunk.position.y as i32,
                        canvas,
                        size,
                        options.color,
                    )?;
                }
            }
            LayoutChunkValue::Link(s) => {
                let (width, height) = text.render_text(
                    &s,
                    chunk.position.x as i32,
                    options.y_offset + chunk.position.y as i32,
                    canvas,
                    if options.underline_links { FontSize::SmallUnderline } else { FontSize::Small },
                    options.color,
                )?;
                on_hittest_text(
                    SDLRect::new(chunk.position.x as i32, options.y_offset + chunk.position.y as i32, width, height),
                    HitTestResult::Text(s.to_string()),
                );
            }
            LayoutChunkValue::Icon(icon) => {
                let icon_loader = options.icons.expect("IconLoader not passed in context icons are found?");
                let icon_image = match icon {
                    LayoutChunkIcon::Sword => icon_loader.get("plain-dagger.png"),
                };
                canvas.copy(
                    icon_image,
                    None,
                    SDLRect::new(
                        chunk.position.x as i32,
                        options.y_offset + chunk.position.y as i32,
                        TEXT_ICON_SIZE,
                        TEXT_ICON_SIZE,
                    ),
                )?;
                on_hittest_text(
                    SDLRect::new(
                        chunk.position.x as i32,
                        options.y_offset + chunk.position.y as i32,
                        TEXT_ICON_SIZE,
                        TEXT_ICON_SIZE,
                    ),
                    HitTestResult::Icon(*icon),
                );

                #[cfg(feature = "debug_text_alignmnet")]
                {
                    canvas.set_draw_color(sdl2::pixels::Color::from((0, 128, 0, 128)));
                    canvas.fill_rect(SDLRect::new(
                        chunk.position.x as i32,
                        options.y_offset + chunk.position.y as i32 - 1,
                        TEXT_ICON_SIZE,
                        TEXT_ICON_SIZE,
                    ))?;
                }
            }
        }
    }
    Ok(())
}
