mod events;

use std::error::Error;
use sdl2;
use sdl2::pixels::Color;
use sdl2::render::Renderer;
use self::events::Events;

pub struct UiContext<'window> {
    pub renderer: Renderer<'window>,
    pub events: Events,
}

impl<'window> UiContext<'window> {
    pub fn new(title: &'static str) -> Result<UiContext, Box<Error>> {
        let sdl_context = sdl2::init()?;
        let video = sdl_context.video()?;
        let window = video
            .window(title, 512, 256)
            .position_centered()
            .resizable()
            .build()?;

        Ok(UiContext {
               renderer: window.renderer().accelerated().build()?,
               events: Events::new(sdl_context.event_pump()?),
           })
    }

    pub fn update(&mut self) {
        self.events.poll();
        if let Some(_) = self.events.immediate.repaint {
            self.renderer.clear();
            self.renderer.present();
        }
    }
}