
use std::{ops::Add, fmt::Display};

use super::{FrameIndex, ErrorCurve};

/// 关键帧曲线数值类型
pub type KeyFrameCurveValue = f32;

/// 构建帧数据结构
#[derive(Debug)]
pub struct KeyFrame {
    /// 帧数据值
    value: KeyFrameCurveValue,
    args: [KeyFrameCurveValue; 2]
}

impl KeyFrame {
    pub fn new(value: KeyFrameCurveValue, args: [KeyFrameCurveValue; 2]) -> Self {
        KeyFrame {
            value,
            args
        }
    }
    pub fn value(&self) -> KeyFrameCurveValue {
        self.value
    }
    pub fn intangent(&self) -> KeyFrameCurveValue {
        self.args[0]
    }
    pub fn outtangent(&self) -> KeyFrameCurveValue {
        self.args[1]
    }
}

pub trait FrameValueScale {
    fn scale(&self, rhs: KeyFrameCurveValue) -> Self;
}

pub type KeyFrameDataType = usize;

/// 动画数据类型分配器
pub struct KeyFrameDataTypeAllocator {
    counter: KeyFrameDataType,
}

impl KeyFrameDataTypeAllocator {
    pub fn default() -> Self {
        Self {
            counter: 0
        }
    }
    pub fn alloc(
        &mut self,
    ) -> Result<KeyFrameDataType, ErrorCurve> {
        if self.counter == KeyFrameDataType::MAX {
            Err(ErrorCurve::KeyFrameDataTypeCannotAllocMore)
        } else {
            let id = self.counter;
            self.counter += 1;
            Ok(id)
        }
    }
}

// pub trait FrameDataValue: Clone + Copy + FrameValueScale + FrameValueInterpolate + Add<Output = Self> {
// }
pub trait FrameDataValue: Clone + Copy + FrameValueScale + Add<Output = Self> {
    fn interpolate(&self, rhs: &Self, amount: KeyFrameCurveValue) -> Self {
        self.scale(1.0 - amount) + rhs.scale(amount)
    }
}

/// f32
impl FrameValueScale for f32 {
    fn scale(&self, rhs: KeyFrameCurveValue) -> Self {
        self * rhs
    }
}
impl FrameDataValue for f32 {}

/// f64
impl FrameValueScale for f64 {
    fn scale(&self, rhs: KeyFrameCurveValue) -> Self {
        self * rhs as f64
    }
}
impl FrameDataValue for f64 {}

/// u8
impl FrameValueScale for u8 {
    fn scale(&self, rhs: KeyFrameCurveValue) -> Self {
        (*self as KeyFrameCurveValue * rhs) as u8
    }
}
impl FrameDataValue for u8 {}

/// u16
impl FrameValueScale for u16 {
    fn scale(&self, rhs: KeyFrameCurveValue) -> Self {
        (*self as KeyFrameCurveValue * rhs) as u16
    }
}
impl FrameDataValue for u16 {}

/// u32
impl FrameValueScale for u32 {
    fn scale(&self, rhs: KeyFrameCurveValue) -> Self {
        (*self as KeyFrameCurveValue * rhs) as u32
    }
}
impl FrameDataValue for u32 {}

/// u64
impl FrameValueScale for u64 {
    fn scale(&self, rhs: KeyFrameCurveValue) -> Self {
        (*self as KeyFrameCurveValue * rhs) as u64
    }
}
impl FrameDataValue for u64 {}

/// usize
impl FrameValueScale for usize {
    fn scale(&self, rhs: KeyFrameCurveValue) -> Self {
        (*self as KeyFrameCurveValue * rhs) as usize
    }
}
impl FrameDataValue for usize {}
