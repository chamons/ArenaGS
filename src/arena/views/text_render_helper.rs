use std::rc::Rc;

use sdl2::rect::Rect as SDLRect;

use super::{HitTestResult, TextHitTester};
use crate::after_image::*;
use crate::atlas::BoxResult;

pub fn render_text_layout(
    layout: &LayoutResult,
    canvas: &mut RenderCanvas,
    hit_tester: &mut TextHitTester,
    text: &TextRenderer,
    icons: &IconCache,
    y_offset: i32,
) -> BoxResult<()> {
    for chunk in &layout.chunks {
        match &chunk.value {
            LayoutChunkValue::String(s) => {
                text.render_text(
                    &s,
                    chunk.position.x as i32,
                    y_offset + chunk.position.y as i32,
                    canvas,
                    FontSize::Small,
                    FontColor::Black,
                )?;
            }
            LayoutChunkValue::Link(s) => {
                let (width, height) = text.render_text(
                    &s,
                    chunk.position.x as i32,
                    y_offset + chunk.position.y as i32,
                    canvas,
                    FontSize::SmallUnderline,
                    FontColor::Black,
                )?;
                hit_tester.add(
                    SDLRect::new(chunk.position.x as i32, y_offset + chunk.position.y as i32, width, height),
                    HitTestResult::Text(s.to_string()),
                );
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
                hit_tester.add(
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
