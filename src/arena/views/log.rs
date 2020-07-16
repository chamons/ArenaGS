use std::cmp;

use sdl2::pixels::Color;
use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use specs::prelude::*;
use specs_derive::Component;

use super::View;
use crate::after_image::{FontColor, FontSize, RenderCanvas, TextRenderer};
use crate::atlas::BoxResult;

const LOG_COUNT: usize = 10;

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
        for (i, entry) in log.get(log.index, LOG_COUNT).iter().enumerate() {
            self.text.render_text(
                entry,
                self.position.x,
                self.position.y + (i as i32 * 30),
                canvas,
                FontSize::Small,
                FontColor::Black,
            )?;
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
    logs: Vec<String>,
    pub index: usize,
}

impl LogComponent {
    pub fn init() -> LogComponent {
        LogComponent { logs: vec![], index: 0 }
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
        self.index = self.clamp_index(self.logs.len() as i64 - LOG_COUNT as i64);
    }

    pub fn scroll_back(&mut self) {
        self.index = self.clamp_index(self.index as i64 - LOG_COUNT as i64);
    }

    pub fn scroll_forward(&mut self) {
        self.index = self.clamp_index(self.index as i64 + LOG_COUNT as i64);
    }

    fn clamp_index(&self, index: i64) -> usize {
        cmp::min(cmp::max(index, 0) as usize, self.logs.len() - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_get() {
        let log = LogComponent {
            logs: vec!["1".to_string(), "2".to_string(), "3".to_string()],
            index: 0,
        };
        let output = log.get(0, 5);
        assert_eq!(output.len(), 3);
        let output = log.get(2, 1);
        assert_eq!(output.len(), 1);
        assert_eq!(output[0], "3".to_string());
        let output = log.get(1, 2);
        assert_eq!(output.len(), 2);
        assert_eq!(output[1], "3".to_string());
    }

    #[test]
    fn get_zero() {
        let log = LogComponent {
            logs: vec!["1".to_string(), "2".to_string(), "3".to_string()],
            index: 0,
        };
        let output = log.get(2, 0);
        assert_eq!(output.len(), 0);
    }

    #[test]
    fn out_of_range() {
        let log = LogComponent {
            logs: vec!["1".to_string(), "2".to_string(), "3".to_string()],
            index: 0,
        };
        let output = log.get(4, 1);
        assert_eq!(output.len(), 0);
    }

    #[test]
    fn get_too_far() {
        let log = LogComponent {
            logs: vec!["1".to_string(), "2".to_string(), "3".to_string()],
            index: 0,
        };
        let output = log.get(2, 3);
        assert_eq!(output.len(), 1);
    }

    #[test]
    fn add_can_bump_index() {
        let mut log = LogComponent::init();
        for i in 0..5 {
            log.add("Test");
            assert_eq!(log.index, 0);
        }
        log.add("Test");
        assert_eq!(log.index, 1);
        log.index = 0;
        log.add("Test");
        assert_eq!(log.index, 2);
    }

    #[test]
    fn scroll_forward() {
        let mut log = LogComponent::init();
        for i in 0..15 {
            log.add("Test");
        }
        log.index = 0;
        log.scroll_forward();
        assert_eq!(log.index, LOG_COUNT);

        for i in 0..5 {
            log.scroll_forward();
        }
        assert_eq!(log.index, log.logs.len() - 1);
    }

    #[test]
    fn scroll_back() {
        let mut log = LogComponent::init();
        for i in 0..15 {
            log.add("Test");
        }
        log.index = 14;
        log.scroll_back();
        assert_eq!(log.index, 14 - LOG_COUNT);

        for i in 0..5 {
            log.scroll_back();
        }
        assert_eq!(log.index, 0);
    }
}

use crate::atlas::Logger;

impl Logger for World {
    fn log(&mut self, message: &str) {
        let mut log = self.write_resource::<LogComponent>();
        log.add(message);
    }
    fn log_scroll_forward(&mut self) {
        let mut log = self.write_resource::<LogComponent>();
        log.scroll_forward();
    }
    fn log_scroll_back(&mut self) {
        let mut log = self.write_resource::<LogComponent>();
        log.scroll_back();
    }
}
