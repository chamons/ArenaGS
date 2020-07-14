use std::cmp;

use sdl2::pixels::Color;
use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use specs::prelude::*;
use specs_derive::Component;

use super::View;
use crate::after_image::{RenderCanvas, TextRenderer};
use crate::atlas::BoxResult;

pub struct LogView<'a> {
    position: SDLPoint,
    text: &'a TextRenderer<'a>,
}

impl<'a> LogView<'a> {
    pub fn init(position: SDLPoint, text: &'a TextRenderer<'a>) -> BoxResult<LogView> {
        Ok(LogView { position, text })
    }

    fn render_log(&self, ecs: &World, canvas: &mut RenderCanvas) -> BoxResult<()> {
        let log = ecs.read_resource::<LogComponent>();
        for (i, entry) in log.get(0, 5).iter().enumerate() {
            self.text.render_text(entry, self.position.x, self.position.y + (i as i32 * 30), canvas)?;
        }

        Ok(())
    }
}

impl<'a> View for LogView<'a> {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, _frame: u64) -> BoxResult<()> {
        canvas.set_draw_color(Color::from((0, 196, 196)));
        canvas.fill_rect(SDLRect::new(self.position.x, self.position.y, 230, 300))?;
        self.render_log(ecs, canvas)?;

        Ok(())
    }
}

#[derive(Component)]
pub struct LogComponent {
    pub logs: Vec<String>,
}

impl LogComponent {
    pub fn init() -> LogComponent {
        LogComponent { logs: vec![] }
    }

    pub fn get(&self, index: usize, length: usize) -> &[String] {
        if length == 0 || index >= self.logs.len() {
            return &[];
        }
        // Take as many items as you can before hitting the end
        let length = cmp::min(index + length, self.logs.len()) - index;
        if length == 1 {
            &self.logs[index..index + 1]
        } else {
            &self.logs[index..index + length]
        }
    }

    pub fn add(&mut self, entry: &str) {
        self.logs.push(entry.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_get() {
        let component = LogComponent {
            logs: vec!["1".to_string(), "2".to_string(), "3".to_string()],
        };
        let output = component.get(0, 5);
        assert_eq!(output.len(), 3);
        let output = component.get(2, 1);
        assert_eq!(output.len(), 1);
        assert_eq!(output[0], "3".to_string());
        let output = component.get(1, 2);
        assert_eq!(output.len(), 2);
        assert_eq!(output[1], "3".to_string());
    }

    #[test]
    fn get_zero() {
        let component = LogComponent {
            logs: vec!["1".to_string(), "2".to_string(), "3".to_string()],
        };
        let output = component.get(2, 0);
        assert_eq!(output.len(), 0);
    }

    #[test]
    fn out_of_range() {
        let component = LogComponent {
            logs: vec!["1".to_string(), "2".to_string(), "3".to_string()],
        };
        let output = component.get(4, 1);
        assert_eq!(output.len(), 0);
    }

    #[test]
    fn get_too_far() {
        let component = LogComponent {
            logs: vec!["1".to_string(), "2".to_string(), "3".to_string()],
        };
        let output = component.get(2, 3);
        assert_eq!(output.len(), 1);
    }
}
