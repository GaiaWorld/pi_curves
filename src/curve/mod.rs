//!
//! 关键帧数据结构

use super::easing::{EEasingMode, function::{easing_call, get_easing_call}};

pub type FrameIndex = u16;
pub type FramePerSecond = u16;

pub type InOutTangent<T> = Vec<T>;
pub type CubicBezier<T> = Vec<T>;

#[derive(Debug)]
pub enum ErrorCurve {
    KeyFrameDataTypeCannotAllocMore,
}

pub mod frame;
pub mod frame_curve;
pub mod curves;


pub trait Repeat<N> {
    fn repeat(val: N) -> Self;
}
