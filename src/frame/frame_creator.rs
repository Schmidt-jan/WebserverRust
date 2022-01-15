use crate::frame::frame_struct::Frame;
use crate::frame::message_types::MessageType;

pub struct FrameCreator {
    id : u64
}

impl FrameCreator {
    pub fn new() -> Self{
        FrameCreator {
            id : 0
        }
    }

    pub fn create<T>(&mut self, m_type : MessageType, data : T) -> Frame<T> {
        let frame = Frame::new(self.id, m_type, data);
        self.id += 1;
        frame
    }

    /*
    pub fn create_ping<T>(&mut self, kernel : Kernel) -> Frame<T> {
        let data: Vec<u8> = vec![0; 0];
        let frame = Frame::new(self.id, MessageType::Ping, kernel, data);
        self.id += 1;
        frame
    }
    */

}