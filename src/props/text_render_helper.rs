use sdl2::rect::Rect as SDLRect;

use crate::after_image::*;
use crate::atlas::prelude::*;
use crate::props::HitTestResult;

#[allow(clippy::too_many_arguments)]
pub fn render_text_layout<'a>(
    layout: &'a LayoutResult,
    canvas: &mut RenderCanvas,
    text: &TextRenderer,
    icons: Option<&IconCache>,
    color: FontColor,
    y_offset: i32,
    underline_links: bool,
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

                text.render_text(
                    &s,
                    chunk.position.x as i32,
                    y_font_offset + y_offset + chunk.position.y as i32,
                    canvas,
                    size,
                    color,
                )?;
            }
            LayoutChunkValue::Link(s) => {
                let (width, height) = text.render_text(
                    &s,
                    chunk.position.x as i32,
                    y_offset + chunk.position.y as i32,
                    canvas,
                    if underline_links { FontSize::SmallUnderline } else { FontSize::Small },
                    color,
                )?;
                on_hittest_text(
                    SDLRect::new(chunk.position.x as i32, y_offset + chunk.position.y as i32, width, height),
                    HitTestResult::Text(s.to_string()),
                );
            }
            LayoutChunkValue::Icon(icon) => {
                let icon_loader = icons.expect("IconLoader not passed in context icons are found?");
                let icon_image = match icon {
                    LayoutChunkIcon::Sword => icon_loader.get("plain-dagger.png"),
                };
                canvas.copy(
                    icon_image,
                    None,
                    SDLRect::new(chunk.position.x as i32, y_offset + chunk.position.y as i32, TEXT_ICON_SIZE, TEXT_ICON_SIZE),
                )?;
                on_hittest_text(
                    SDLRect::new(chunk.position.x as i32, y_offset + chunk.position.y as i32, TEXT_ICON_SIZE, TEXT_ICON_SIZE),
                    HitTestResult::Icon(*icon),
                );

                #[cfg(feature = "debug_text_alignmnet")]
                {
                    canvas.set_draw_color(sdl2::pixels::Color::from((0, 128, 0, 128)));
                    canvas.fill_rect(SDLRect::new(
                        chunk.position.x as i32,
                        y_offset + chunk.position.y as i32 - 1,
                        TEXT_ICON_SIZE,
                        TEXT_ICON_SIZE,
                    ))?;
                }
            }
        }
    }
    Ok(())
}