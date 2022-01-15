use serde::{Serialize, Deserialize};
use bincode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KernelFunctions {
    FnAdd,
    FnSub,
    FnDiv,
    FnMul,
    FnDot,
    FnSqrt,
    FnTra
}