//! 缓动处理

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum EEasingMode {
    None            = 0x00,

    BackIn          = 0x01,
    BackOut         = 0x02,
    BackInOut       = 0x03,

    CircleIn        = 0x04,
    CircleOut       = 0x05,
    CircleInOut     = 0x06,

    CubicIn         = 0x07,
    CubicOut        = 0x08,
    CubicInOut      = 0x09,

    SineIn          = 0x11,
    SineOut         = 0x12,
    SineInOut       = 0x13,

    QuadIn          = 0x14,
    QuadOut         = 0x15,
    QuadInOut       = 0x16,

    QuartIn         = 0x17,
    QuartOut        = 0x18,
    QuartInOut      = 0x19,

    QuintIn         = 0x21,
    QuintOut        = 0x22,
    QuintInOut      = 0x23,

    ExpoIn          = 0x24,
    ExpoOut         = 0x25,
    ExpoInOut       = 0x26,

    ElasticIn       = 0x27,
    ElasticOut      = 0x28,
    ElasticInOut    = 0x29,

    BounceIn        = 0x31,
    BounceOut       = 0x32,
    BounceInOut     = 0x33,
}

pub mod function;