use std::cmp;
use std::mem;

use super::Font;
use crate::atlas::{BoxResult, Point};

pub struct LayoutRequest {
    position: Point,
    width_to_render_in: u32,
    space_between_lines: u32,
}

impl LayoutRequest {
    pub fn init(x: u32, y: u32, width_to_render_in: u32, space_between_lines: u32) -> LayoutRequest {
        LayoutRequest {
            position: Point::init(x, y),
            width_to_render_in,
            space_between_lines,
        }
    }
}

#[derive(Copy, Clone, is_enum_variant)]
pub enum LayoutChunkIcon {
    Sword,
}

pub enum LayoutChunkValue {
    String(String),
    Icon(LayoutChunkIcon),
}

pub struct LayoutChunk {
    pub position: Point,
    pub value: LayoutChunkValue,
}

pub struct LayoutResult {
    pub chunks: Vec<LayoutChunk>,
    pub line_count: u32,
}

struct Layout {
    result: LayoutResult,
    current_line_width: u32,
    largest_line_height: u32,
    current_line: String,
    current_y_offset: u32,
    request: LayoutRequest,
}

impl Layout {
    fn init(request: LayoutRequest) -> Layout {
        Layout {
            result: LayoutResult { chunks: vec![], line_count: 0 },
            current_line_width: 0,
            largest_line_height: 0,
            current_line: String::new(),
            current_y_offset: request.position.y,
            request,
        }
    }

    fn create_next_chunk(&mut self) {
        self.result.chunks.push(LayoutChunk {
            value: LayoutChunkValue::String(mem::replace(&mut self.current_line, String::new())),
            position: Point::init(self.request.position.x, self.current_y_offset),
        });
        self.current_line = String::new();
        self.current_line_width = 0
    }

    fn run(&mut self, text: &str, font: &Font) -> BoxResult<()> {
        for word in text.split_ascii_whitespace() {
            let (mut width, mut height) = font.size_of_latin1(word.as_bytes())?;

            let is_symbol = word.starts_with("{{") && word.ends_with("}}");
            if is_symbol {
                width = 24;
                height = 24;
            }

            let is_line_wrapping = self.current_line_width + width > self.request.width_to_render_in && self.current_line_width > 0;
            if is_line_wrapping {
                self.create_next_chunk();
                self.current_y_offset += self.largest_line_height + self.request.space_between_lines;
                self.largest_line_height = 0;
                self.result.line_count += 1;
            }

            if is_symbol && !is_line_wrapping {
                self.create_next_chunk();
            }

            if is_symbol {
                let icon = match &word[2..word.len() - 2] {
                    "Sword" => LayoutChunkIcon::Sword,
                    _ => panic!("Unknown icon kind {}", &word[2..word.len() - 2]),
                };
                self.result.chunks.push(LayoutChunk {
                    value: LayoutChunkValue::Icon(icon),
                    position: Point::init(self.request.position.x, self.current_y_offset),
                });
            } else {
                self.largest_line_height = cmp::max(self.largest_line_height, height);
                self.current_line_width += width;
                if self.current_line.len() > 0 {
                    self.current_line.push_str(" ");
                }
                self.current_line.push_str(word);
            }
        }

        // Apply leftovers to the last line
        self.create_next_chunk();
        self.result.line_count += 1;

        Ok(())
    }

    fn results(self) -> LayoutResult {
        self.result
    }
}

pub fn layout_text(text: &str, font: &Font, request: LayoutRequest) -> BoxResult<LayoutResult> {
    let mut layout = Layout::init(request);
    layout.run(text, font)?;
    Ok(layout.results())
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use lazy_static::lazy_static;
    use leak::Leak;

    use super::*;
    use crate::atlas::{assert_points_equal, get_exe_folder};

    lazy_static! {
        static ref TTF_CONTEXT: &'static sdl2::ttf::Sdl2TtfContext = Box::from(sdl2::ttf::init().unwrap()).leak();
    }

    fn get_test_font() -> Font {
        let font_path = Path::new(&get_exe_folder()).join("fonts").join("LibreFranklin-Regular.ttf");
        let mut font = TTF_CONTEXT.load_font(font_path, 14).unwrap();
        font.set_style(sdl2::ttf::FontStyle::NORMAL);
        font
    }

    fn get_text(chunk: &LayoutChunkValue) -> &String {
        match chunk {
            LayoutChunkValue::String(s) => s,
            _ => panic!("Wrong type?"),
        }
    }

    fn get_icon(chunk: &LayoutChunkValue) -> LayoutChunkIcon {
        match chunk {
            LayoutChunkValue::Icon(i) => *i,
            _ => panic!("Wrong type?"),
        }
    }

    #[test]
    fn layout_one_line() {
        let result = layout_text("Hello World", &get_test_font(), LayoutRequest::init(10, 10, 32 + 39 /*sizeof Hello World*/, 10)).unwrap();
        assert_eq!(1, result.chunks.len());
        assert_eq!("Hello World", get_text(&result.chunks[0].value));
        assert_eq!(1, result.line_count);
        assert_points_equal(Point::init(10, 10), result.chunks[0].position);
    }

    #[test]
    fn layout_multiple_line() {
        let result = layout_text(
            "Hello World Hello World Hello",
            &get_test_font(),
            LayoutRequest::init(10, 10, 32 + 39 /*sizeof Hello World*/, 10),
        )
        .unwrap();
        assert_eq!(3, result.chunks.len());
        assert_eq!("Hello World", get_text(&result.chunks[0].value));
        assert_eq!("Hello World", get_text(&result.chunks[1].value));
        assert_eq!("Hello", get_text(&result.chunks[2].value));
        assert_eq!(3, result.line_count);
        assert_points_equal(Point::init(10, 10), result.chunks[0].position);
    }

    #[test]
    fn layout_one_super_long_word() {
        let result = layout_text(
            "HelloWorldHelloWorldHello",
            &get_test_font(),
            LayoutRequest::init(10, 10, 32 + 39 /*sizeof Hello World*/, 10),
        )
        .unwrap();
        assert_eq!(1, result.chunks.len());
        assert_eq!("HelloWorldHelloWorldHello", get_text(&result.chunks[0].value));
        assert_eq!(1, result.line_count);
        assert_points_equal(Point::init(10, 10), result.chunks[0].position);
    }

    #[test]
    fn layout_line_with_icon() {
        let result = layout_text(
            "A {{Sword}} B",
            &get_test_font(),
            LayoutRequest::init(10, 10, 32 + 39 /*sizeof Hello World*/, 10),
        )
        .unwrap();
        assert_eq!(3, result.chunks.len());
        assert_eq!("A", get_text(&result.chunks[0].value));
        assert!(get_icon(&result.chunks[1].value).is_sword());
        assert_eq!("B", get_text(&result.chunks[2].value));
        assert_eq!(1, result.line_count);
        assert_points_equal(Point::init(10, 10), result.chunks[0].position);
        assert_points_equal(Point::init(10, 37), result.chunks[1].position);
        assert_points_equal(Point::init(10, 10), result.chunks[2].position);
    }

    //#[test]
    fn layout_line_with_link() {
        let result = layout_text(
            "Hello [[World]] Bye",
            &get_test_font(),
            LayoutRequest::init(10, 10, 32 + 39 /*sizeof Hello World*/, 10),
        )
        .unwrap();
        assert_eq!(3, result.chunks.len());
        assert_eq!("Hello", get_text(&result.chunks[0].value));
        assert_eq!("World", get_text(&result.chunks[1].value));
        assert_eq!("Bye", get_text(&result.chunks[2].value));
        assert_eq!(2, result.line_count);
        assert_points_equal(Point::init(10, 10), result.chunks[0].position);
        assert_points_equal(Point::init(10, 37), result.chunks[1].position);
        assert_points_equal(Point::init(10, 10), result.chunks[2].position);
    }

    //#[test]
    fn layout_line_with_link_sandwhich() {
        let result = layout_text(
            "A [[World]] B",
            &get_test_font(),
            LayoutRequest::init(10, 10, 32 + 39 /*sizeof Hello World*/, 10),
        )
        .unwrap();
        assert_eq!(3, result.chunks.len());
        assert_eq!("A", get_text(&result.chunks[0].value));
        assert_eq!("World", get_text(&result.chunks[1].value));
        assert_eq!("B", get_text(&result.chunks[2].value));
        assert_eq!(1, result.line_count);
        assert_points_equal(Point::init(10, 10), result.chunks[0].position);
        assert_points_equal(Point::init(10, 10), result.chunks[1].position);
        assert_points_equal(Point::init(10, 10), result.chunks[2].position);
    }
}
