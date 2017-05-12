const DEFAULT_DISPLAY_WIDTH: usize = 64;
const DEFAULT_DISPLAY_HEIGHT: usize = 32;

pub enum VideoSignal {
    None,
    Clear,
    Refresh,
}

pub struct VideoCommunicator {
    pub display: Vec<bool>,
    pub width: usize,
    pub height: usize,
    pub signal: VideoSignal,
}

impl VideoCommunicator {
    pub fn new() -> VideoCommunicator {
        VideoCommunicator {
            display: vec![false; DEFAULT_DISPLAY_WIDTH * DEFAULT_DISPLAY_HEIGHT],
            width: DEFAULT_DISPLAY_WIDTH,
            height: DEFAULT_DISPLAY_HEIGHT,
            signal: VideoSignal::None,
        }
    }
}