use serde::{Serialize, Deserialize};
use bincode;

//defined by the execution-unit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Narray {
    pub data : Vec<u8>
}