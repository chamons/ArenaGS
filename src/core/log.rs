use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Log {
    pub messages: Vec<String>,
    pub index: usize,
}

impl Log {
    pub fn new() -> Self {
        Log { messages: vec![], index: 0 }
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

#[no_mangle]
pub fn set_message_index(mut events: EventReader<ScrollMessageEvent>) {
    for event in events.iter() {
        match event.kind {
            ScrollMessageKind::PageUp => println!("Page Up"),
            ScrollMessageKind::PageDown => println!("Page Down"),
            ScrollMessageKind::ScrollToEnd => println!("Scroll To End"),
        }
    }
}
