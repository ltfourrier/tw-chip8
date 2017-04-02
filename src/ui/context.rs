use std::error::Error;
use sdl2;
use sdl2::render::Renderer;
use super::events::Events;

pub struct Context<'window> {
    pub renderer: Renderer<'window>,
    pub events: Events,
}

impl<'window> Context<'window> {
    pub fn new(title: &'static str) -> Result<Context, Box<Error>> {
        let sdl_context = sdl2::init()?;
        let video = sdl_context.video()?;
        let window = video.window(title, 64, 32)
            .position_centered()
            .opengl()
            .resizable()
            .build()?;

        Ok(Context {
            renderer: window.renderer().accelerated().build()?,
            events: Events::new(sdl_context.event_pump()?),
        })
    }
}