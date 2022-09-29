use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Log {
    pub messages: Vec<String>,
    pub last_index: usize,
}

impl Log {
    pub fn new() -> Self {
        Log {
            messages: vec![],
            last_index: 0,
        }
    }

    pub fn push(&mut self, message: &str) {
        self.messages.push(message.to_string());
    }
}

pub struct NewMessageEvent {
    pub message: String,
}

impl NewMessageEvent {
    pub fn new(message: &str) -> Self {
        NewMessageEvent { message: message.to_string() }
    }
}

pub enum ScrollMessageKind {
    PageUp,
    PageDown,
    ScrollToEnd,
}

pub struct ScrollMessageEvent {
    pub kind: ScrollMessageKind,
}

impl ScrollMessageEvent {
    pub fn page_up() -> Self {
        ScrollMessageEvent {
            kind: ScrollMessageKind::PageUp,
        }
    }

    pub fn page_down() -> Self {
        ScrollMessageEvent {
            kind: ScrollMessageKind::PageDown,
        }
    }

    pub fn scroll_to_end() -> Self {
        ScrollMessageEvent {
            kind: ScrollMessageKind::ScrollToEnd,
        }
    }
}

#[no_mangle]
pub fn process_new_messages(mut log: ResMut<Log>, mut events: EventReader<NewMessageEvent>, mut scroll_event: EventWriter<ScrollMessageEvent>) {
    for event in events.iter() {
        log.push(&event.message);
        scroll_event.send(ScrollMessageEvent::scroll_to_end());
    }
}

pub const LOG_ENTRIES_ON_SCREEN: usize = 8;

#[no_mangle]
pub fn set_message_index(mut log: ResMut<Log>, mut events: EventReader<ScrollMessageEvent>) {
    let message_count = log.messages.len();
    for event in events.iter() {
        let new_last_index = match event.kind {
            ScrollMessageKind::PageUp => std::cmp::max(log.last_index as i64 - LOG_ENTRIES_ON_SCREEN as i64, 0) as usize,
            ScrollMessageKind::PageDown => std::cmp::min(log.last_index + LOG_ENTRIES_ON_SCREEN, message_count),
            ScrollMessageKind::ScrollToEnd => message_count,
        };
        log.last_index = new_last_index;
    }
}
