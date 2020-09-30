use sdl2::rect::Rect as SDLRect;

use super::{HitTestResult, TextHitTester};
use crate::after_image::*;
use crate::atlas::BoxResult;

pub fn render_text_layout(
    layout: &LayoutResult,
    canvas: &mut RenderCanvas,
    hit_tester: &mut Option<&mut TextHitTester>,
    text: &TextRenderer,
    icons: &IconCache,
    color: FontColor,
    y_offset: i32,
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
                    FontSize::SmallUnderline,
                    color,
                )?;
                if let Some(ref mut hit_tester) = hit_tester.as_mut() {
                    hit_tester.add(
                        SDLRect::new(chunk.position.x as i32, y_offset + chunk.position.y as i32, width, height),
                        HitTestResult::Text(s.to_string()),
                    );
                }
            }
            LayoutChunkValue::Icon(icon) => {
                let icon_image = match icon {
                    LayoutChunkIcon::Sword => icons.get("plain-dagger.png"),
                };
                canvas.copy(
                    icon_image,
                    None,
                    SDLRect::new(chunk.position.x as i32, y_offset + chunk.position.y as i32, TEXT_ICON_SIZE, TEXT_ICON_SIZE),
                )?;
                if let Some(ref mut hit_tester) = hit_tester.as_mut() {
                    hit_tester.add(
                        SDLRect::new(chunk.position.x as i32, y_offset + chunk.position.y as i32, TEXT_ICON_SIZE, TEXT_ICON_SIZE),
                        HitTestResult::Icon(*icon),
                    );
                }

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
