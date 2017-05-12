pub mod video;

use self::video::VideoCommunicator;

pub struct Communicator {
    pub video: VideoCommunicator
}

impl Communicator {
    pub fn new() -> Communicator {
        Communicator { video: VideoCommunicator::new() }
    }
}