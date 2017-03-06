use std::error::Error;

use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct Window {
    context: sdl2::Sdl,
    video: sdl2::VideoSubsystem,
    handle: sdl2::video::Window,
    pump: sdl2::EventPump,
    running: bool,
}

impl Window {
    pub fn create() -> Result<Window, Box<Error>> {
        let sdl_context = sdl2::init()?;
        let video = sdl_context.video()?;
        let window = video.window("TW-Chip8", 64, 32)
            .position_centered()
            .opengl()
            .build()?;
        let event_pump = sdl_context.event_pump()?;
        
        Ok(Window{
            context: sdl_context,
            video: video,
            handle: window,
            pump: event_pump,
            running: true,
        })
    }

    pub fn update(&mut self) {
        for event in self.pump.poll_iter() {
            match event {
                Event::Quit { .. } => self.running = false,
                _ => (),
            };
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }
}