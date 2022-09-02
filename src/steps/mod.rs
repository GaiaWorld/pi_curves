use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum EStepMode {
    JumpStart       = 0x00,
    JumpEnd,
    JumpNone,
    JumpBoth,
}

