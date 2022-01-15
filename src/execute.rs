use serde::{Serialize, Deserialize};
use bincode;
use crate::kernel::kernel_functions::KernelFunctions;

//defined by the execution-unit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Execute {
    pub function : KernelFunctions
}