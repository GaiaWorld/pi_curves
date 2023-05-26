
use std::ops::Add;

use nalgebra::{Vector3, Vector4, Vector2};

use super::ErrorCurve;

/// 关键帧曲线数值类型
pub type KeyFrameCurveValue = f32;

/// 构建帧数据结构
#[derive(Debug)]
pub struct CurveFrameValue<T: FrameDataValue> {
    /// 帧数据值
    value: T,
    args: [T; 2]
}

impl<T: FrameDataValue> CurveFrameValue<T> {
    pub fn new(value: T, args: [T; 2]) -> Self {
        CurveFrameValue {
            value,
            args
        }
    }
    pub fn value(&self) -> &T {
        &self.value
    }
    pub fn intangent(&self) -> &T {
        &self.args[0]
    }
    pub fn outtangent(&self) -> &T {
        &self.args[1]
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
pub trait FrameDataValue: Clone {
    fn interpolate(&self, rhs: &Self, amount: KeyFrameCurveValue) -> Self;
    fn append(&self, rhs: &Self, amount: KeyFrameCurveValue) -> Self;
    fn hermite(value1: &Self, tangent1: &Self, value2: &Self, tangent2: &Self, amount: KeyFrameCurveValue) -> Self;
    fn size() -> usize;
}

impl<T: Clone + FrameValueScale + Add<Output = Self>> FrameDataValue for T {
    fn interpolate(&self, rhs: &Self, amount: KeyFrameCurveValue) -> Self {
        self.scale(1.0 - amount) + rhs.scale(amount)
    }
    fn hermite(value1: &Self, tangent1: &Self, value2: &Self, tangent2: &Self, amount: KeyFrameCurveValue) -> Self {
        let _1 = 1 as KeyFrameCurveValue;
        let _2 = 2 as KeyFrameCurveValue;
        let _3 = 3 as KeyFrameCurveValue;

        let squared = amount * amount;
        let cubed = amount * squared;
        let part1 = ((_2 * cubed) - (_3 * squared)) + _1;
        let part2 = (-_2 * cubed) + (_3 * squared);
        let part3 = (cubed - (_2 * squared)) + amount;
        let part4 = cubed - squared;

        return (((value1.scale(part1)) + (value2.scale(part2))) + (tangent1.scale(part3))) + (tangent2.scale(part4));
    }
    fn append(&self, rhs: &Self, amount: KeyFrameCurveValue) -> Self {
        self.clone() + rhs.scale(amount)
    }
    fn size() -> usize {
        8
    }
}

/// f32
impl FrameValueScale for f32 {
    fn scale(&self, rhs: KeyFrameCurveValue) -> Self {
        self * rhs
    }
}

/// f64
impl FrameValueScale for f64 {
    fn scale(&self, rhs: KeyFrameCurveValue) -> Self {
        self * rhs as f64
    }
}

/// u8
impl FrameValueScale for u8 {
    fn scale(&self, rhs: KeyFrameCurveValue) -> Self {
        (*self as KeyFrameCurveValue * rhs) as u8
    }
}

/// u16
impl FrameValueScale for u16 {
    fn scale(&self, rhs: KeyFrameCurveValue) -> Self {
        (*self as KeyFrameCurveValue * rhs) as u16
    }
}

/// u32
impl FrameValueScale for u32 {
    fn scale(&self, rhs: KeyFrameCurveValue) -> Self {
        (*self as KeyFrameCurveValue * rhs) as u32
    }
}

/// u64
impl FrameValueScale for u64 {
    fn scale(&self, rhs: KeyFrameCurveValue) -> Self {
        (*self as KeyFrameCurveValue * rhs) as u64
    }
}

/// usize
impl FrameValueScale for usize {
    fn scale(&self, rhs: KeyFrameCurveValue) -> Self {
        (*self as KeyFrameCurveValue * rhs) as usize
    }
}

/// usize
impl FrameValueScale for Vector2<f32> {
    fn scale(&self, rhs: KeyFrameCurveValue) -> Self {
        (self * rhs) as Vector2<f32>
    }
}

/// usize
impl FrameValueScale for Vector3<f32> {
    fn scale(&self, rhs: KeyFrameCurveValue) -> Self {
        (self * rhs) as Vector3<f32>
    }
}

/// usize
impl FrameValueScale for Vector4<f32> {
    fn scale(&self, rhs: KeyFrameCurveValue) -> Self {
        (self * rhs) as Vector4<f32>
    }
}
