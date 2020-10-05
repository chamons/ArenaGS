use std::path::Path;

use lazy_static::lazy_static;
use leak::Leak;

use super::*;
use crate::atlas::*;

lazy_static! {
    static ref TTF_CONTEXT: &'static sdl2::ttf::Sdl2TtfContext = Box::from(sdl2::ttf::init().unwrap()).leak();
}

pub fn has_test_font() -> bool {
    let font_path = Path::new(&get_exe_folder()).join("fonts").join("LibreFranklin-Regular.ttf");
    font_path.exists()
}

pub fn get_test_font() -> Font {
    let font_path = Path::new(&get_exe_folder()).join("fonts").join("LibreFranklin-Regular.ttf");
    let mut font = TTF_CONTEXT.load_font(font_path, 14).unwrap();
    font.set_style(sdl2::ttf::FontStyle::NORMAL);
    font
}

pub fn get_tiny_test_font() -> Font {
    let font_path = Path::new(&get_exe_folder()).join("fonts").join("LibreFranklin-Regular.ttf");
    let mut font = TTF_CONTEXT.load_font(font_path, 9).unwrap();
    font.set_style(sdl2::ttf::FontStyle::NORMAL);
    font
}
