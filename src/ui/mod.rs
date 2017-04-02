mod context;
mod events;

use std::thread;
use std::time::Duration;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::error::Error;
use self::context::Context;

pub enum InboundSignal {
    Quit
}

pub enum OutboundSignal {
    Quit
}

pub struct UiThread {
    pub signal_tx: Sender<InboundSignal>,
    pub signal_rx: Receiver<OutboundSignal>,
    pub handle: thread::JoinHandle<()>,
}

pub fn spawn_ui(title: &'static str) -> Result<UiThread, Box<Error>> {
    let (out_tx, out_rx) = channel();
    let (in_tx, in_rx) = channel();
    let handle = thread::spawn(move || {
        let mut ctx = Context::new(title).unwrap();

        let mut running = true;
        while running {
            // Check the inbound signal buffer before we do anything.
            for signal in in_rx.try_iter() {
                match signal {
                    InboundSignal::Quit => running = false,
                };
            }

            ctx.events.poll();
            if ctx.events.quit.is_some() {
                running = false;
                out_tx.send(OutboundSignal::Quit).unwrap();
            }

            thread::sleep(Duration::from_millis(16));
        }
    });

    Ok(UiThread {
        signal_tx: in_tx,
        signal_rx: out_rx,
        handle: handle,
    })
}