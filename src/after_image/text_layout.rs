use std::cmp;
use std::mem;

use lazy_static::lazy_static;
use regex::{Captures, Regex};

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

#[derive(Copy, Clone, is_enum_variant, Debug)]
pub enum LayoutChunkIcon {
    Sword,
}

pub enum LayoutChunkValue {
    String(String),
    Link(String),
    Icon(LayoutChunkIcon),
}

bitflags! {
    pub struct LayoutChunkAttributes : u32 {
        const SMALLER_TEXT =         0b00000001;
    }
}

pub struct LayoutChunk {
    pub position: Point,
    pub value: LayoutChunkValue,
    pub attributes: LayoutChunkAttributes,
}

pub struct LayoutResult {
    pub chunks: Vec<LayoutChunk>,
    pub line_count: u32,
}

// We collect words until we line wrap or non a non-word
// then flush it as a single Layout chunk
struct WordBuffer {
    current_line: String,
    width: u32,
}

impl WordBuffer {
    pub fn init() -> WordBuffer {
        WordBuffer {
            current_line: String::new(),
            width: 0,
        }
    }

    pub fn has_content(&self) -> bool {
        self.width > 0
    }

    pub fn add(&mut self, text: &str, mut width: u32, space_size: u32) {
        if self.current_line.len() > 0 {
            self.current_line.push_str(" ");
            width += space_size;
        }
        self.current_line.push_str(text);
        self.width += width;
    }

    pub fn flush(&mut self) -> (String, u32) {
        let value = mem::replace(&mut self.current_line, String::new());
        let width = self.width;
        self.width = 0;
        (value, width)
    }
}

struct LayoutRect {
    corner: Point,
    largest_line_height: u32,
    current_line_width: u32,
    // Current x position based upon flushed content
    current_x_offset: u32,
    // Current y position based upon flushed content
    current_y_offset: u32,
}

impl LayoutRect {
    pub fn init(corner: &Point) -> LayoutRect {
        LayoutRect {
            corner: *corner,
            current_line_width: 0,
            largest_line_height: 0,
            current_x_offset: corner.x,
            current_y_offset: corner.y,
        }
    }

    // We 'spend' line width but do not update x,y cursor
    pub fn add_text(&mut self, height: u32, width: u32) {
        self.current_line_width += width;
        self.largest_line_height = cmp::max(self.largest_line_height, height);
    }

    // Updates for flushed content and returns cursor before move
    pub fn flush(&mut self, width: u32) -> Point {
        let point = Point::init(self.current_x_offset, self.current_y_offset);
        self.current_x_offset += width;
        point
    }

    // We've completed a line, reset for next
    pub fn new_line(&mut self, space_between_lines: u32) {
        self.current_x_offset = self.corner.x;
        self.current_y_offset += self.largest_line_height + space_between_lines;
        self.largest_line_height = 0;
        self.current_line_width = 0;
    }

    pub fn at_start_of_line(&self) -> bool {
        self.current_line_width == 0
    }
}

struct Layout {
    request: LayoutRequest,

    word_buffer: WordBuffer, // Buffer for words until wrap/non-word content
    rect: LayoutRect,        // Tracks current cursor location, spent width, etc
    space_size: u32,
    links_in_flight: String, // Links can have spaces in the middle, this buffers them until closing ]]

    result: LayoutResult,
}

pub const TEXT_ICON_SIZE: u32 = 17;

impl Layout {
    fn init(request: LayoutRequest) -> Layout {
        Layout {
            result: LayoutResult { chunks: vec![], line_count: 0 },
            word_buffer: WordBuffer::init(),
            rect: LayoutRect::init(&request.position),
            space_size: 0,
            request,
            links_in_flight: String::new(),
        }
    }

    fn should_wrap(&self, width: u32) -> bool {
        // If it's longer than the remaining space AND we've moved a bit over
        // A word longer than an entire line should not wrap an empty space
        self.rect.current_line_width + width > self.request.width_to_render_in && self.rect.current_line_width > 0
    }

    fn longer_single_line(&self, width: u32) -> bool {
        width > self.request.width_to_render_in
    }

    fn flush_any_text(&mut self) {
        if self.word_buffer.has_content() {
            let (text, text_width) = self.word_buffer.flush();
            let position = self.rect.flush(text_width);

            self.result.chunks.push(LayoutChunk {
                value: LayoutChunkValue::String(text),
                position,
                attributes: LayoutChunkAttributes::empty(),
            });
        }
    }

    fn flush_small_text(&mut self, text: &str, text_width: u32) {
        let position = self.rect.flush(text_width + 4);

        self.result.chunks.push(LayoutChunk {
            value: LayoutChunkValue::String(text.to_string()),
            position,
            attributes: LayoutChunkAttributes::SMALLER_TEXT,
        });
    }

    fn flush_icon(&mut self, name: &str) {
        let icon = match name {
            "Sword" => LayoutChunkIcon::Sword,
            _ => panic!("Unknown icon kind {}", name),
        };

        let position = self.rect.flush(TEXT_ICON_SIZE);
        self.result.chunks.push(LayoutChunk {
            value: LayoutChunkValue::Icon(icon),
            position,
            attributes: LayoutChunkAttributes::empty(),
        });
    }

    fn flush_link(&mut self, text: &str, text_width: u32) {
        // Add space sized space after link
        let mid_line_link = !self.rect.at_start_of_line();
        let number_spaces_added = if mid_line_link { 2 } else { 1 };
        let mut position = self.rect.flush(text_width + self.space_size * number_spaces_added);

        if mid_line_link {
            // Add a space by adjust start over one space
            position.x += self.space_size;
        }

        let mut attributes = LayoutChunkAttributes::empty();
        if self.longer_single_line(text_width) {
            attributes.insert(LayoutChunkAttributes::SMALLER_TEXT);
        }

        self.result.chunks.push(LayoutChunk {
            value: LayoutChunkValue::Link(text.to_string()),
            position,
            attributes,
        });
    }

    pub const SYMBOL_REGEX: &'static str = "^(.*)(\\{\\{\\w*\\}\\})(.*)$";
    pub const LINK_REGEX: &'static str = "^(.*)(\\[\\[\\w*\\]\\])(.*)$";
    pub const LINK_REGEX_FRONT: &'static str = "^(.*)(\\[\\[\\w*)$";
    pub const LINK_REGEX_END: &'static str = "^(\\w*\\]\\])(.*)$";
    pub const TAB_REGEX: &'static str = "^\\|tab\\|(.*)$";

    fn run(&mut self, text: &str, font: &Font) -> BoxResult<()> {
        let (space_width, _) = font.size_of_latin1(b" ")?;
        self.space_size = space_width;

        for mut word in text.split_ascii_whitespace() {
            lazy_static! {
                static ref SYMBOL_RE: Regex = Regex::new(Layout::SYMBOL_REGEX).unwrap();
                static ref LINK_RE: Regex = Regex::new(Layout::LINK_REGEX).unwrap();
                static ref FRONT_LINK_RE: Regex = Regex::new(Layout::LINK_REGEX_FRONT).unwrap();
                static ref END_LINK_RE: Regex = Regex::new(Layout::LINK_REGEX_END).unwrap();
                static ref TAB_RE: Regex = Regex::new(Layout::TAB_REGEX).unwrap();
            }

            if let Some(_) = TAB_RE.captures(word) {
                // Four spaces, the 5th will be added between this chunk and the first word
                self.process_word("    ", font)?;
                word = word.trim_start_matches("|tab|")
            }

            if let Some(m) = SYMBOL_RE.captures(word) {
                self.process_complex_chunk(m, font)?;
            } else if let Some(m) = LINK_RE.captures(word) {
                self.process_complex_chunk(m, font)?;
            } else if let Some(m) = FRONT_LINK_RE.captures(word) {
                self.process_complex_chunk(m, font)?;
            } else if let Some(m) = END_LINK_RE.captures(word) {
                self.process_complex_chunk(m, font)?;
            } else {
                self.process_word(word, font)?;
            }
        }

        // Apply leftovers to the last line
        self.flush_any_text();
        self.result.line_count += 1;

        Ok(())
    }

    fn process_complex_chunk(&mut self, m: Captures, font: &Font) -> BoxResult<()> {
        for i in 1..4 {
            if let Some(chunk) = m.get(i) {
                let chunk = chunk.as_str();
                if chunk.len() > 0 {
                    self.process_word(chunk, font)?;
                }
            }
        }

        Ok(())
    }

    fn process_link_word(&mut self, word: &str) -> (bool, Option<String>) {
        let is_link_start = word.starts_with("[[");
        let is_link_end = word.ends_with("]]");
        let is_full_link = is_link_start && is_link_end;

        if is_full_link {
            (false, Some(word[2..word.len() - 2].to_string()))
        } else if is_link_start {
            if self.links_in_flight.len() > 0 {
                self.links_in_flight.push_str(" ");
            }
            self.links_in_flight.push_str(word);
            (true, None)
        } else if is_link_end {
            self.links_in_flight.push_str(" ");
            self.links_in_flight.push_str(word);
            let link = self.links_in_flight[2..self.links_in_flight.len() - 2].to_string();
            self.links_in_flight.clear();
            (false, Some(link))
        } else if self.links_in_flight.len() > 0 {
            self.links_in_flight.push_str(" ");
            self.links_in_flight.push_str(word);
            (true, None)
        } else {
            (false, None)
        }
    }

    fn process_word(&mut self, word: &str, font: &Font) -> BoxResult<()> {
        let (mut width, mut height) = font.size_of_latin1(word.as_bytes())?;

        let is_symbol = word.starts_with("{{") && word.ends_with("}}");
        if is_symbol {
            width = TEXT_ICON_SIZE;
            height = TEXT_ICON_SIZE;
        }

        let (skip, link_text) = self.process_link_word(word);
        if skip {
            return Ok(());
        }

        if let Some(link_text) = &link_text {
            let (w, h) = font.size_of_latin1(link_text.as_bytes())?;
            width = w;
            height = h;
        }

        let is_line_wrapping = self.should_wrap(width);
        let longer_single_line = self.longer_single_line(width);

        if is_line_wrapping | longer_single_line | is_symbol | link_text.is_some() {
            self.flush_any_text();
        }
        if is_line_wrapping {
            self.rect.new_line(self.request.space_between_lines);
            self.result.line_count += 1;
        }

        if is_symbol {
            self.flush_icon(&word[2..word.len() - 2]);
        } else if let Some(link_text) = &link_text {
            self.flush_link(&link_text, width)
        } else if longer_single_line {
            // Spend the correct width
            self.rect.add_text(height, width);

            // Then flush right away (so nothing else is marked small)
            self.flush_small_text(word, width);
        } else {
            self.rect.add_text(height, width);
            self.word_buffer.add(word, width, self.space_size);
        }

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

    fn has_test_font() -> bool {
        let font_path = Path::new(&get_exe_folder()).join("fonts").join("LibreFranklin-Regular.ttf");
        font_path.exists()
    }

    fn get_test_font() -> Font {
        let font_path = Path::new(&get_exe_folder()).join("fonts").join("LibreFranklin-Regular.ttf");
        let mut font = TTF_CONTEXT.load_font(font_path, 14).unwrap();
        font.set_style(sdl2::ttf::FontStyle::NORMAL);
        font
    }

    fn get_tiny_test_font() -> Font {
        let font_path = Path::new(&get_exe_folder()).join("fonts").join("LibreFranklin-Regular.ttf");
        let mut font = TTF_CONTEXT.load_font(font_path, 9).unwrap();
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

    fn get_link(chunk: &LayoutChunkValue) -> &String {
        match chunk {
            LayoutChunkValue::Link(s) => s,
            _ => panic!("Wrong type?"),
        }
    }

    #[test]
    fn layout_one_line() {
        if !has_test_font() {
            return;
        }

        let result = layout_text("Hello World", &get_test_font(), LayoutRequest::init(10, 10, 32 + 39 /*sizeof Hello World*/, 10)).unwrap();
        assert_eq!(1, result.chunks.len());
        assert_eq!("Hello World", get_text(&result.chunks[0].value));
        assert_eq!(1, result.line_count);
        assert_points_equal(Point::init(10, 10), result.chunks[0].position);
    }

    #[test]
    fn layout_multiple_line() {
        if !has_test_font() {
            return;
        }

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
        assert_points_equal(Point::init(10, 37), result.chunks[1].position);
        assert_points_equal(Point::init(10, 64), result.chunks[2].position);
    }

    #[test]
    fn layout_one_super_long_word() {
        if !has_test_font() {
            return;
        }

        let result = layout_text(
            "HelloWorldHelloWorldHello",
            &get_test_font(),
            LayoutRequest::init(10, 10, 32 + 39 /*sizeof Hello World*/, 10),
        )
        .unwrap();
        assert_eq!(1, result.chunks.len());
        assert_eq!("HelloWorldHelloWorldHello", get_text(&result.chunks[0].value));
        assert!(result.chunks[0].attributes.contains(LayoutChunkAttributes::SMALLER_TEXT));
        assert_eq!(1, result.line_count);
        assert_points_equal(Point::init(10, 10), result.chunks[0].position);
    }

    #[test]
    fn recognize_icon_with_parens() {
        if !has_test_font() {
            return;
        }

        let result = layout_text("({{Sword}})", &get_test_font(), LayoutRequest::init(10, 10, 32 + 39 /*sizeof Hello World*/, 10)).unwrap();
        assert_eq!(3, result.chunks.len());
        assert_eq!("(", get_text(&result.chunks[0].value));
        assert!(get_icon(&result.chunks[1].value).is_sword());
        assert_eq!(")", get_text(&result.chunks[2].value));
    }

    #[test]
    fn layout_line_with_icon() {
        if !has_test_font() {
            return;
        }

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
        assert_points_equal(Point::init(20, 10), result.chunks[1].position);
        assert_points_equal(Point::init(37, 10), result.chunks[2].position);
    }

    #[test]
    fn layout_line_with_duel_icons() {
        if !has_test_font() {
            return;
        }

        let result = layout_text(
            "{{Sword}} {{Sword}}",
            &get_test_font(),
            LayoutRequest::init(10, 10, 32 + 39 /*sizeof Hello World*/, 10),
        )
        .unwrap();
        assert_eq!(2, result.chunks.len());
        assert!(get_icon(&result.chunks[0].value).is_sword());
        assert!(get_icon(&result.chunks[1].value).is_sword());
        assert_eq!(1, result.line_count);
        assert_points_equal(Point::init(10, 10), result.chunks[0].position);
        assert_points_equal(Point::init(27, 10), result.chunks[1].position);
    }

    #[test]
    fn layout_icon_multiline_text() {
        if !has_test_font() {
            return;
        }

        let result = layout_text(
            "Hello World Hello {{Sword}} LongerWorld {{Sword}} Board",
            &get_test_font(),
            LayoutRequest::init(10, 10, 32 + 39 /*sizeof Hello World*/, 10),
        )
        .unwrap();
        assert_eq!(6, result.chunks.len());
        assert_eq!("Hello World", get_text(&result.chunks[0].value));
        assert_eq!("Hello", get_text(&result.chunks[1].value));
        assert!(get_icon(&result.chunks[2].value).is_sword());
        assert_eq!("LongerWorld", get_text(&result.chunks[3].value));
        assert!(get_icon(&result.chunks[4].value).is_sword());
        assert_eq!("Board", get_text(&result.chunks[5].value));
        assert_eq!(4, result.line_count);
        assert_points_equal(Point::init(10, 10), result.chunks[0].position);
        assert_points_equal(Point::init(10, 37), result.chunks[1].position);
        assert_points_equal(Point::init(42, 37), result.chunks[2].position);
        assert_points_equal(Point::init(10, 64), result.chunks[3].position);
        assert_points_equal(Point::init(10, 91), result.chunks[4].position);
        assert_points_equal(Point::init(27, 91), result.chunks[5].position);
    }

    #[test]
    fn layout_icon_multiline_text_tiny() {
        if !has_test_font() {
            return;
        }

        let result = layout_text(
            "Hello World Hello {{Sword}} LongerWorld {{Sword}} Board",
            &get_tiny_test_font(),
            LayoutRequest::init(10, 10, 32 + 39 /*sizeof Hello World*/, 10),
        )
        .unwrap();
        assert_eq!(5, result.chunks.len());
        assert_eq!("Hello World Hello", get_text(&result.chunks[0].value));
        assert!(get_icon(&result.chunks[1].value).is_sword());
        assert_eq!("LongerWorld", get_text(&result.chunks[2].value));
        assert!(get_icon(&result.chunks[3].value).is_sword());
        assert_eq!("Board", get_text(&result.chunks[4].value));
        assert_eq!(3, result.line_count);
        assert_points_equal(Point::init(10, 10), result.chunks[0].position);
        assert_points_equal(Point::init(10, 31), result.chunks[1].position);
        assert_points_equal(Point::init(27, 31), result.chunks[2].position);
        assert_points_equal(Point::init(79, 31), result.chunks[3].position);
        assert_points_equal(Point::init(10, 52), result.chunks[4].position);
    }

    #[test]
    fn layout_icon_paren_combat_text() {
        if !has_test_font() {
            return;
        }

        let result = layout_text("Player took 4 damage ({{Sword}} 4).", &get_test_font(), LayoutRequest::init(10, 10, 210, 10)).unwrap();
        assert_eq!(3, result.chunks.len());
        assert_eq!("Player took 4 damage (", get_text(&result.chunks[0].value));
        assert!(get_icon(&result.chunks[1].value).is_sword());
        assert_eq!("4).", get_text(&result.chunks[2].value));
        assert_eq!(1, result.line_count);
        assert_points_equal(Point::init(10, 10), result.chunks[0].position);
        assert_points_equal(Point::init(159, 10), result.chunks[1].position);
        assert_points_equal(Point::init(176, 10), result.chunks[2].position);
    }

    #[test]
    fn layout_line_with_link() {
        if !has_test_font() {
            return;
        }

        let result = layout_text(
            "Hello [[World]] Bye",
            &get_test_font(),
            LayoutRequest::init(10, 10, 32 + 39 /*sizeof Hello World*/, 10),
        )
        .unwrap();
        assert_eq!(3, result.chunks.len());
        assert_eq!("Hello", get_text(&result.chunks[0].value));
        assert_eq!("World", get_link(&result.chunks[1].value));
        assert_eq!("Bye", get_text(&result.chunks[2].value));
        assert_eq!(1, result.line_count);
        assert_points_equal(Point::init(10, 10), result.chunks[0].position);
        assert_points_equal(Point::init(45, 10), result.chunks[1].position);
        assert_points_equal(Point::init(87, 10), result.chunks[2].position);
    }

    #[test]
    fn layout_line_with_link_sandwhich() {
        if !has_test_font() {
            return;
        }

        let result = layout_text(
            "A [[World]] B",
            &get_test_font(),
            LayoutRequest::init(10, 10, 32 + 39 /*sizeof Hello World*/, 10),
        )
        .unwrap();
        assert_eq!(3, result.chunks.len());
        assert_eq!("A", get_text(&result.chunks[0].value));
        assert_eq!("World", get_link(&result.chunks[1].value));
        assert_eq!("B", get_text(&result.chunks[2].value));
        assert_eq!(1, result.line_count);
        assert_points_equal(Point::init(10, 10), result.chunks[0].position);
        assert_points_equal(Point::init(23, 10), result.chunks[1].position);
        assert_points_equal(Point::init(65, 10), result.chunks[2].position);
    }

    #[test]
    fn recognize_link_with_parens() {
        if !has_test_font() {
            return;
        }

        let result = layout_text("([[Sword]])", &get_test_font(), LayoutRequest::init(10, 10, 32 + 39 /*sizeof Hello World*/, 10)).unwrap();
        assert_eq!(3, result.chunks.len());
        assert_eq!("(", get_text(&result.chunks[0].value));
        assert_eq!("Sword", get_link(&result.chunks[1].value));
        assert_eq!(")", get_text(&result.chunks[2].value));
    }

    #[test]
    fn recognize_link_with_period() {
        if !has_test_font() {
            return;
        }

        let result = layout_text(
            "A [[Sword]].",
            &get_test_font(),
            LayoutRequest::init(10, 10, 32 + 39 /*sizeof Hello World*/, 10),
        )
        .unwrap();
        assert_eq!(3, result.chunks.len());
        assert_eq!("A", get_text(&result.chunks[0].value));
        assert_eq!("Sword", get_link(&result.chunks[1].value));
        assert_eq!(".", get_text(&result.chunks[2].value));
    }

    #[test]
    fn recognize_link_with_spaces() {
        if !has_test_font() {
            return;
        }

        let result = layout_text(
            "A [[Sword Strike]]",
            &get_test_font(),
            LayoutRequest::init(10, 10, 32 + 39 /*sizeof Hello World*/, 10),
        )
        .unwrap();
        assert_eq!(2, result.chunks.len());
        assert_eq!("A", get_text(&result.chunks[0].value));
        assert_eq!("Sword Strike", get_link(&result.chunks[1].value));
    }

    #[test]
    fn recognize_link_with_spaces_and_period() {
        if !has_test_font() {
            return;
        }

        let result = layout_text(
            "A [[Sword Strike]].",
            &get_test_font(),
            LayoutRequest::init(10, 10, 32 + 39 /*sizeof Hello World*/, 10),
        )
        .unwrap();
        assert_eq!(3, result.chunks.len());
        assert_eq!("A", get_text(&result.chunks[0].value));
        assert_eq!("Sword Strike", get_link(&result.chunks[1].value));
        assert_eq!(".", get_text(&result.chunks[2].value));
    }

    #[test]
    fn recognize_tabbed_words() {
        if !has_test_font() {
            return;
        }

        let result = layout_text(
            "|tab|A thing.",
            &get_test_font(),
            LayoutRequest::init(10, 10, 32 + 39 /*sizeof Hello World*/, 10),
        )
        .unwrap();
        assert_eq!(1, result.chunks.len());
        assert_eq!("     A thing.", get_text(&result.chunks[0].value));
        assert_points_equal(Point::init(10, 10), result.chunks[0].position);
    }

    #[test]
    fn recognize_tabbed_links() {
        if !has_test_font() {
            return;
        }

        let result = layout_text(
            "|tab|[[A Link]].",
            &get_test_font(),
            LayoutRequest::init(10, 10, 32 + 39 /*sizeof Hello World*/, 10),
        )
        .unwrap();
        assert_eq!(3, result.chunks.len());
        // 4 spaces not 5 due to https://github.com/chamons/ArenaGS/issues/222
        assert_eq!("    ", get_text(&result.chunks[0].value));
        assert_eq!("A Link", get_link(&result.chunks[1].value));
        assert_eq!(".", get_text(&result.chunks[2].value));
        assert_points_equal(Point::init(10, 10), result.chunks[0].position);
        assert_points_equal(Point::init(25, 10), result.chunks[1].position);
        assert_points_equal(Point::init(68, 10), result.chunks[2].position);
    }

    #[test]
    fn three_part_link() {
        if !has_test_font() {
            return;
        }

        let result = layout_text(
            "Elementalist used [[Invoke the Elements]].",
            &get_test_font(),
            LayoutRequest::init(10, 10, 150, 10),
        )
        .unwrap();
        assert_eq!(3, result.chunks.len());
        // 4 not 5 due to https://github.com/chamons/ArenaGS/issues/222
        assert_eq!("Elementalist used", get_text(&result.chunks[0].value));
        assert_eq!("Invoke the Elements", get_link(&result.chunks[1].value));
        assert_eq!(".", get_text(&result.chunks[2].value));

        assert_points_equal(Point::init(10, 10), result.chunks[0].position);
        assert_points_equal(Point::init(10, 37), result.chunks[1].position);
        assert_points_equal(Point::init(145, 37), result.chunks[2].position);
    }

    #[test]
    fn text_after_link() {
        if !has_test_font() {
            return;
        }

        let result = layout_text(
            "See [[Defenses in Depth]] for details..",
            &get_test_font(),
            LayoutRequest::init(10, 10, 240, 10),
        )
        .unwrap();
        assert_eq!(3, result.chunks.len());
        // 4 not 5 due to https://github.com/chamons/ArenaGS/issues/222
        assert_eq!("See", get_text(&result.chunks[0].value));
        assert_eq!("Defenses in Depth", get_link(&result.chunks[1].value));
        assert_eq!("for details..", get_text(&result.chunks[2].value));

        assert_points_equal(Point::init(10, 10), result.chunks[0].position);
        assert_points_equal(Point::init(38, 10), result.chunks[1].position);
        assert_points_equal(Point::init(158, 10), result.chunks[2].position);
    }
}
