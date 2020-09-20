use std::cmp;

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

pub struct LayoutChunk {
    pub position: Point,
    pub text: String,
}

pub struct LayoutResult {
    pub lines: Vec<LayoutChunk>,
}

pub fn layout_text(text: &str, font: Font, request: LayoutRequest) -> BoxResult<LayoutResult> {
    let mut current_line_width = 0;
    let mut largest_line_height = 0;
    let mut current_line = String::new();
    let mut current_y_offset = request.position.y;
    let mut result = LayoutResult { lines: vec![] };

    for word in text.split_ascii_whitespace() {
        let (width, height) = font.size_of_latin1(word.as_bytes())?;
        if current_line_width + width <= request.width_to_render_in {
            largest_line_height = cmp::max(largest_line_height, height);
            current_line_width += width;
            if current_line.len() > 0 {
                current_line.push_str(" ");
            }
            current_line.push_str(word);
        } else {
            result.lines.push(LayoutChunk {
                text: current_line,
                position: Point::init(request.position.x, current_y_offset),
            });
            current_y_offset += largest_line_height + request.space_between_lines;
            current_line = String::new();
            current_line_width = 0;
            largest_line_height = 0;
        }
    }

    // Apply leftovers to the last line
    result.lines.push(LayoutChunk {
        text: current_line,
        position: Point::init(request.position.x, current_y_offset),
    });
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::atlas::{assert_points_equal, get_exe_folder};
    use leak::Leak;
    use std::path::Path;

    #[test]
    fn layout_text_one_line() {
        let ttf_context = Box::from(sdl2::ttf::init().unwrap()).leak();
        let font_path = Path::new(&get_exe_folder()).join("fonts").join("LibreFranklin-Regular.ttf");
        let mut font = ttf_context.load_font(font_path, 14).unwrap();
        font.set_style(sdl2::ttf::FontStyle::NORMAL);

        let result = layout_text("Hello World", font, LayoutRequest::init(10, 10, 32 + 39 /*sizeof Hello World*/, 10)).unwrap();
        assert_eq!(1, result.lines.len());
        assert_eq!("Hello World", result.lines[0].text);
        assert_points_equal(Point::init(10, 10), result.lines[0].position);
    }
}
