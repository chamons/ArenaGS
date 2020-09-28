use specs::prelude::*;

use super::ShortInfo;
use crate::after_image::LayoutChunkIcon;

pub enum HelpHeader {
    None,
    Image(String),
    Text(String),
}

pub struct HelpInfo {
    pub header: HelpHeader,
    pub text: Vec<String>,
}

impl HelpInfo {
    pub fn init() -> HelpInfo {
        HelpInfo {
            header: HelpHeader::None,
            text: vec![],
        }
    }

    pub fn no_header(text: &[&str]) -> HelpInfo {
        HelpInfo {
            header: HelpHeader::None,
            text: text.iter().map(|t| t.to_string()).collect(),
        }
    }

    fn get_error(key: &str) -> HelpInfo {
        HelpInfo::no_header(&[&format!(
            "Please file an issue at https://tinyurl.com/ArenaGS-Issue with a description ({}) on how you reached this message.",
            key
        )])
    }

    pub fn find(word: &str) -> HelpInfo {
        match word {
            _ => HelpInfo::get_error(word),
        }
    }

    pub fn find_icon(icon: LayoutChunkIcon) -> HelpInfo {
        HelpInfo::get_error(&format!("{:?}", icon))
    }

    pub fn find_entity(ecs: &World, entity: Entity) -> HelpInfo {
        HelpInfo::get_error(&ecs.get_name(&entity).unwrap_or("Unknown Entity".to_string()))
    }
}
