use std::fmt::Debug;

use crate::easing::{EEasingMode, function::get_easing_call};

use self::{easing_curve::interplate_easing, frames::interplate_frame_values, minmax_curve::interplate_minmaxcurve, cubic_splice::interplate_cubic_splice, cubic_bezier_curve::interplate_cubebezier};

use super::{frame::{FrameDataValue, KeyFrameCurveValue, CurveFrameValue}, FrameIndex, FramePerSecond};

pub mod frames;
pub mod cubic_bezier_curve;
pub mod cubic_splice;
pub mod minmax_curve;
pub mod easing_curve;

#[derive(Debug)]
pub enum EFrameCurveType {
    /// 关键帧数值 - Linear
    /// 帧数据数组[ frameIndex[], value[] ]
    FrameValues = 0x00,
    /// 基础值 + 缩放值 + 曲线(f32)
    FrameValuesStep = 0x01,
    /// Easing曲线
    /// 基础value + 缩放value + Easing模式
    EasingCurve = 0x02,
    /// Hermit 曲线
    /// 基础value + 缩放value + 帧数据数组[ frameIndex[], value(f32)[] ]
    MinMaxCurve = 0x03,
    /// 2D 三次贝塞尔曲线
    /// 基础value + 缩放value + 曲线参数
    CubicBezierCurve = 0x04,
    /// 帧数据数组[ frameIndex[], <intanget, value, outtanget>[] ]
    /// GLTF Cubic Spline interpolation 
    /// https://github.com/KhronosGroup/glTF-Tutorials/blob/master/gltfTutorial/gltfTutorial_007_Animations.md
    GLTFCubicSpline = 0x05,
}

pub struct FrameCurve<T: FrameDataValue> {
    /// 设计每秒多少帧
    pub design_frame_per_second: FramePerSecond,

    /// 动画目标数据的起始值
    pub value_offset: Option<T>,
    /// 动画目标数据的变化域值
    pub value_scalar: Option<T>,

    /// 缓动类型 [Easing 缓动类型]
    easing_mode: EEasingMode,
    /// 曲线拓展数据 [CubicBezier的参数]
    cubic_bezier_args: [KeyFrameCurveValue; 4],

    /// 帧序号值
    pub frames: Vec<FrameIndex>,

    /// For MinMaxCurve
    pub minmax_curve_values: Vec<CurveFrameValue<KeyFrameCurveValue>>,

    /// For FrameValues | FrameValuesStep
    pub values: Vec<T>,
    /// For GLTFCubicSpline
    pub cubic_spline_values: Vec<CurveFrameValue<T>>,

    /// 起始帧
    pub min_frame: FrameIndex,
    /// 结束帧
    pub max_frame: FrameIndex,
    /// 动画帧数
    pub frame_number: FrameIndex,
    pub call: fn(&Self, KeyFrameCurveValue) -> T,
    pub easing: fn(KeyFrameCurveValue) -> KeyFrameCurveValue,
}

impl<F: FrameDataValue> AsRef<FrameCurve<F>> for FrameCurve<F> {
    fn as_ref(&self) -> &FrameCurve<F> {
        self
    }
}

impl<T: Debug + FrameDataValue> Debug for FrameCurve<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FrameCurve")
            .field("easing_mode", &self.easing_mode)
            .field("cubic_bezier_args", &self.cubic_bezier_args)
            .field("design_frame_per_second", &self.design_frame_per_second)
            .field("curve_values", &self.minmax_curve_values)
            .field("frames", &self.frames)
            .field("values", &self.values)
            .field("value_offset", &self.value_offset)
            .field("value_scalar", &self.value_scalar)
            .field("min_frame", &self.min_frame)
            .field("max_frame", &self.max_frame)
            .field("frame_number", &self.frame_number)
            .finish()
    }
}

impl<T: FrameDataValue> FrameCurve<T> {
    pub fn size(&self) -> usize {
        1 + 1 + 4 * 4 + 2 + 2 + 2 + 2 + 8 + 8
    }
    pub fn interple(&self, target_frame: KeyFrameCurveValue) -> T {
        let call = &self.call;
        let target_frame = target_frame * self.design_frame_per_second as KeyFrameCurveValue;
        call(&self, target_frame)
    }
    
    /// 曲线 - 线性插值帧 - 无曲线描述,仅关键 帧-值
    ///
    pub fn curve_frame_values(design_frame_per_second: FramePerSecond) -> FrameCurve<T> {
        FrameCurve {
            design_frame_per_second,
            value_offset: None,
            value_scalar: None,
            easing_mode: EEasingMode::None,
            cubic_bezier_args: [0., 0., 1., 1.],
            frames: vec![],
            minmax_curve_values: vec![],
            values: vec![],
            cubic_spline_values: vec![],
            min_frame: FrameIndex::MAX,
            max_frame: FrameIndex::MIN,
            frame_number: 0 as FrameIndex,
            call: interplate_frame_values::<T>,
            easing: get_easing_call(EEasingMode::None),
        }
    }
    /// 曲线关键帧 - 线性插值帧 - 无曲线描述,仅关键 帧-值
    ///
    /// * [framecurve] - 目标曲线
    /// * [frame] - 帧位置
    /// * [value] - 帧数值
    ///
    pub fn curve_frame_values_frame(&mut self, frame: FrameIndex, value: T) {
        let index = self.frames.binary_search(&frame).unwrap_or_else(|x| x);
        self.frames.insert(index, frame);
        self.values.insert(index, value);

        let len = self.frames.len();
        let min = self.frames[0];
        let max = self.frames[len - 1];

        self.min_frame = min;
        self.max_frame = max;
        self.frame_number = max - min;
    }
    /// 曲线 - 线性插值帧 - 无曲线描述,仅关键 帧-值
    ///
    pub fn curve_cubic_spline(design_frame_per_second: FramePerSecond) -> FrameCurve<T> {
        FrameCurve {
            design_frame_per_second,
            value_offset: None,
            value_scalar: None,
            easing_mode: EEasingMode::None,
            cubic_bezier_args: [0., 0., 1., 1.],
            frames: vec![],
            minmax_curve_values: vec![],
            values: vec![],
            cubic_spline_values: vec![],
            min_frame: FrameIndex::MAX,
            max_frame: FrameIndex::MIN,
            frame_number: 0 as FrameIndex,
            call: interplate_cubic_splice::<T>,
            easing: get_easing_call(EEasingMode::None),
        }
    }
    /// 曲线关键帧 - 线性插值帧 - 无曲线描述,仅关键 帧-值
    ///
    /// * [framecurve] - 目标曲线
    /// * [frame] - 帧位置
    /// * [value] - 帧数值
    ///
    pub fn curve_cubic_splice_frame(&mut self, frame: FrameIndex, value: T, intangent: T, outtangent: T) {
        let keyframe = CurveFrameValue::new(value, [intangent, outtangent]);

        let index = self.frames.binary_search(&frame).unwrap_or_else(|x| x);
        self.frames.insert(index, frame);
        self.cubic_spline_values.insert(index, keyframe);

        let len = self.frames.len();
        let min = self.frames[0];
        let max = self.frames[len - 1];

        self.min_frame = min;
        self.max_frame = max;
        self.frame_number = max - min;
    }

    /// 曲线 - Hermit插值曲线
    ///
    /// * [from] - 动画数值起点
    /// * [scalar] - 动画数值变化域值
    ///
    pub fn curve_minmax_curve(
        from: T,
        scalar: T,
        design_frame_per_second: FramePerSecond,
    ) -> FrameCurve<T> {
        FrameCurve {
            design_frame_per_second,
            value_offset: Some(from),
            value_scalar: Some(scalar),
            easing_mode: EEasingMode::None,
            cubic_bezier_args: [0., 0., 1., 1.],
            frames: vec![],
            minmax_curve_values: vec![],
            values: vec![],
            cubic_spline_values: vec![],
            min_frame: FrameIndex::MAX,
            max_frame: FrameIndex::MIN,
            frame_number: 0 as FrameIndex,
            call: interplate_minmaxcurve::<T>,
            easing: get_easing_call(EEasingMode::None),
        }
    }

    /// 曲线关键帧 - Hermit插值曲线
    ///
    /// * [framecurve] - 目标曲线
    /// * [frame] - 帧位置
    /// * [value] - 帧数值
    /// * [intangent] - In Tangent
    /// * [outtangent] - Out Tangent
    ///
    pub fn curve_minmax_curve_frame(
        &mut self,
        frame: FrameIndex,
        value: KeyFrameCurveValue,
        intangent: KeyFrameCurveValue,
        outtangent: KeyFrameCurveValue,
    ) {
        let keyframe = CurveFrameValue::new(value, [intangent, outtangent]);

        let index = self.frames.binary_search(&frame).unwrap_or_else(|x| x);
        self.frames.insert(index, frame);
        self.minmax_curve_values.insert(index, keyframe);

        let len = self.frames.len();
        let min = self.frames[0];
        let max = self.frames[len - 1];

        self.min_frame = min;
        self.max_frame = max;
        self.frame_number = max - min;
    }
    
    /// 曲线 -  Easing 缓动 - result = from + scalar * easing(t)
    ///
    /// * [from] - 动画数值起点
    /// * [scalar] - 动画数值变化域值
    /// * [frame_count] - 变化时间阈值 (帧数)
    /// * [easing_mode] - 缓动模式 (https://easings.net/#)
    ///
    pub fn curve_easing(
        from: T,
        scalar: T,
        frame_count: FrameIndex,
        design_frame_per_second: FramePerSecond,
        easing_mode: EEasingMode,
    ) -> FrameCurve<T> {
        FrameCurve {
            design_frame_per_second,
            value_offset: Some(from),
            value_scalar: Some(scalar),
            easing_mode,
            cubic_bezier_args: [0., 0., 1., 1.],
            frames: vec![],
            minmax_curve_values: vec![],
            values: vec![],
            cubic_spline_values: vec![],
            min_frame: 0 as FrameIndex,
            max_frame: frame_count,
            frame_number: frame_count,
            call: interplate_easing::<T>,
            easing: get_easing_call(easing_mode),
        }
    }
    
    /// 曲线 - CubicBezier 插值曲线
    ///
    /// * [from] - 动画数值起点
    /// * [scalar] - 动画数值变化域值
    /// * [x1,y1,x2,y2] - CubicBezier 曲线参数 (https://cubic-bezier.com/)
    ///
    pub fn curve_cubic_bezier(
        from: T,
        scalar: T,
        frame_count: FrameIndex,
        design_frame_per_second: FramePerSecond,
        x1: KeyFrameCurveValue,
        y1: KeyFrameCurveValue,
        x2: KeyFrameCurveValue,
        y2: KeyFrameCurveValue,
    ) -> FrameCurve<T> {
        FrameCurve {
            design_frame_per_second,
            value_offset: Some(from),
            value_scalar: Some(scalar),
            easing_mode: EEasingMode::None,
            cubic_bezier_args: [x1, y1, x2, y2],
            frames: vec![],
            minmax_curve_values: vec![],
            values: vec![],
            cubic_spline_values: vec![],
            min_frame: 0 as FrameIndex,
            max_frame: frame_count,
            frame_number: frame_count,
            call: interplate_cubebezier::<T>,
            easing: get_easing_call(EEasingMode::None),
        }
    }

    /// 获取目标帧的前后帧在帧数组中的序号
    pub fn get_pre_next_frame_index(
        frames: &Vec<FrameIndex>,
        target_frame: KeyFrameCurveValue,
    ) -> (usize, usize) {
        let total_num = frames.len();
        let index = frames
            .binary_search(&(target_frame as FrameIndex))
            .unwrap_or_else(|x| x);
        if index == 0 {
            // println!("AA {}, {}", index, target_frame);
            (index, index)
        } else if index <= total_num - 1 {
            // println!("BB {}, {}", index, target_frame);
            (index - 1, index)
        } else {
            // println!("CC {}, {}", index, target_frame);
            (index - 1, index - 1)
        }
    }
}