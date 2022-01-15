use serde::{Serialize, Deserialize};
use bincode;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MessageType {
    Narray = 0,
    Kernel = 1,
    ExecuteFn = 2
}