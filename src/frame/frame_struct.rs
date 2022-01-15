use crate::frame::message_types::MessageType;
use serde::{Serialize, Deserialize};
use bincode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frame<T> {
    pub id : u64,               //id to map asynchronous request/ responses. Request has same id as its response
    pub m_type : MessageType,   // defines the message type
    pub data : T,               // data for the message type
}

impl<T> Frame<T> {
    pub fn new(id : u64, m_type : MessageType, data : T) -> Self{
        Frame {
            id, m_type, data
        }
    }
}