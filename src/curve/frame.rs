
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
pub trait FrameValueAdd {
    fn add(&self, rhs: &Self) -> Self;
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

pub trait FrameDataValue: Clone + Copy + FrameValueScale + Add<Output = Self> {
}