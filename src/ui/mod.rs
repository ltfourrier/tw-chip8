mod events;

use std::error::Error;
use sdl2;
use sdl2::rect::Point;
use sdl2::pixels::Color;
use sdl2::render::Renderer;
use com::Communicator;
use com::video::{VideoCommunicator, VideoSignal};
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

    pub fn update(&mut self, com: &mut Communicator) {
        self.events.poll();

        // TODO: Do something with scale here too.
        if let Some(_) = self.events.immediate.repaint {
            let size = self.renderer
                .window()
                .map(|window| (window.size().0 as f32, window.size().1 as f32));
            if let Some(size) = size {
                if let Err(e) = self.renderer
                       .set_scale(size.0 / com.video.width as f32,
                                  size.1 / com.video.height as f32) {
                    warn!("Can't set renderer scale: {}.", e);
                }
            }
            self.render(&mut com.video);
        }

        match com.video.signal {
            VideoSignal::None => (),
            VideoSignal::Clear => self.clear(),
            VideoSignal::Refresh => self.render(&mut com.video),
        };
        com.video.signal = VideoSignal::None;
    }

    fn clear(&mut self) {
        self.renderer.clear();
        self.renderer.present();
    }

    fn render(&mut self, video_com: &mut VideoCommunicator) {
        self.renderer.clear();
        self.renderer.set_draw_color(Color::RGB(255, 255, 255));
        for (idx, pixel) in video_com.display.iter().enumerate() {
            if *pixel {
                if let Err(e) = self.renderer
                       .draw_point(Point::new((idx % video_com.width) as i32,
                                              (idx / video_com.width) as i32)) {
                    warn!("Can't draw point with renderer: {}.", e);
                }
            }
        }
        self.renderer.set_draw_color(Color::RGB(0, 0, 0));
        self.renderer.present();
    }
}