use sdl2::EventPump;
use sdl2::event::Event;

/*
pub enum KeyEvent {
    None,
    Clicked,
    Pressed,
    Released,
}
*/

pub struct Events {
    pump: EventPump,
    pub quit: Option<Event>,
}

impl Events {
    pub fn new(pump: EventPump) -> Events {
        Events {
            pump: pump,
            quit: None,
        }
    }

    pub fn poll(&mut self) {
        for event in self.pump.poll_iter() {
            match event {
                Event::Quit { .. } => self.quit = Some(event),
                _ => (),
            }
        }
    }
}