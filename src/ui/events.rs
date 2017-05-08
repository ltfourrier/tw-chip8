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

pub struct ImmediateEvents {
    pub repaint: Option<Event>,
}

impl ImmediateEvents {
    pub fn new() -> ImmediateEvents {
        ImmediateEvents {
            repaint: None,
        }
    }
}

pub struct Events {
    pump: EventPump,
    pub immediate: ImmediateEvents,
    pub quit: bool,
}

impl Events {
    pub fn new(pump: EventPump) -> Events {
        Events {
            pump: pump,
            immediate: ImmediateEvents::new(),
            quit: false,
        }
    }

    pub fn poll(&mut self) {
        self.immediate = ImmediateEvents::new();
        for event in self.pump.poll_iter() {
            match event {
                Event::Quit { .. } => self.quit = true,
                Event::Window { .. } => self.immediate.repaint = Some(event),
                _ => (),
            }
        }
    }
}