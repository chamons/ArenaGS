use super::RenderContext;

pub enum EventStatus {
    Quit,
    Continue,
}

type EventHandler = fn(sdl2::event::Event) -> EventStatus;

pub fn pump_messages(render_context: &mut RenderContext, handler: EventHandler) -> EventStatus {
    for event in render_context.event_pump.poll_iter() {
        if let EventStatus::Quit = handler(event) {
            return EventStatus::Quit;
        }
    }
    EventStatus::Continue
}
