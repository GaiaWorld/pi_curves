use crate::{easing::{EEasingMode, function::get_easing_call}, hermite, bezier};

use super::{frame::{FrameDataValue, KeyFrameCurveValue, KeyFrame}, FrameIndex, FramePerSecond};

#[derive(Debug)]
pub enum EFrameCurveType {
    /// 线性
    Linear      = 0x01,
    /// Hermit 曲线
    MinMaxCurve = 0x02,
    /// 缓动描述
    Easing      = 0x03,
    /// 2D 三次贝塞尔曲线
    CubicBezier = 0x04,
}

pub struct FrameCurve<T: FrameDataValue> {
    /// 曲线类型类型 [Linear | Easing | MinMaxCurve | CubicBezier]
    curve: EFrameCurveType,
    /// 缓动类型 [Easing 缓动类型]
    easing_mode: EEasingMode,
    /// 曲线拓展数据 [CubicBezier的参数]
    extend_args: [KeyFrameCurveValue; 4],
    /// 设计每秒多少帧
    pub design_frame_per_second: FramePerSecond,
    /// 
    /// 关键帧数组 [Linear]
    /// 关键帧数组 [MinMaxCurve]
    /// 
    pub key_args: Vec<KeyFrame>,
    /// 帧序号值
    pub frames: Vec<FrameIndex>,

    /// 动画目标数据的起始值
    pub value_offset: T,

    /// 动画目标数据的变化域值
    pub value_scalar: T,
    /// 起始帧
    pub min_frame: FrameIndex,
    /// 结束帧
    pub max_frame: FrameIndex,
    /// 动画帧数
    pub frame_number: FrameIndex,
    pub call: fn(&Self, KeyFrameCurveValue) -> T,
    pub easing: fn(KeyFrameCurveValue) -> KeyFrameCurveValue,
}

impl<T: FrameDataValue> FrameCurve<T> {
    pub fn interple(&self, target_frame: KeyFrameCurveValue) -> T {
        let call = &self.call;
        let target_frame = target_frame * self.design_frame_per_second as KeyFrameCurveValue;
        call(&self, target_frame)
    }
    
    /// 曲线 - 线性插值曲线
    /// 
    /// * [from] - 动画数值起点
    /// * [scalar] - 动画数值变化域值
    /// 
    pub fn curve_linear(from: T, scalar: T, design_frame_per_second: FramePerSecond) -> FrameCurve<T> {
        FrameCurve {
            curve: EFrameCurveType::Linear,
            easing_mode: EEasingMode::None,
            extend_args: [0., 0., 1., 1.],
            key_args: Vec::default(),
            value_offset: from,
            value_scalar: scalar,
            min_frame: FrameIndex::MAX,
            max_frame: FrameIndex::MIN,
            design_frame_per_second,
            frame_number: 0,
            frames: vec![],
            call: FrameCurve::get_curve_call(EFrameCurveType::Linear),
            easing: get_easing_call(&EEasingMode::None),
        }
    }

    /// 曲线关键帧 - 线性插值曲线
    /// 
    /// * [framecurve] - 目标曲线
    /// * [frame] - 帧位置
    /// * [value] - 帧数值
    /// 
    pub fn curve_linear_frame(mut framecurve: FrameCurve<T>, frame: FrameIndex, value: KeyFrameCurveValue) -> FrameCurve<T> {

        let keyframe = KeyFrame::new(value, [0., 0.]);

        let index = framecurve.frames.binary_search(&frame).unwrap_or_else(|x| x);
        framecurve.frames.insert(index, frame);
        framecurve.key_args.insert(index, keyframe);

        let len = framecurve.frames.len();
        let min = framecurve.frames[0];
        let max = framecurve.frames[len - 1];

        framecurve.min_frame = min;
        framecurve.max_frame = max;
        framecurve.frame_number = max - min;

        return framecurve;
    }

    /// 曲线 -  Easing 缓动 - result = from + scalar * easing(t)
    /// 
    /// * [from] - 动画数值起点
    /// * [scalar] - 动画数值变化域值
    /// * [frame_count] - 变化时间阈值 (帧数)
    /// * [easing_mode] - 缓动模式 (https://easings.net/#)
    /// 
    pub fn curve_easing(from: T, scalar: T, frame_count: FrameIndex, design_frame_per_second: FramePerSecond, easing_mode: EEasingMode) -> FrameCurve<T> {
        FrameCurve {
            curve: EFrameCurveType::Easing,
            easing_mode,
            extend_args: [0., 0., 1., 1.],
            key_args: Vec::default(),
            value_offset: from,
            value_scalar: scalar,
            min_frame: 0,
            max_frame: frame_count,
            frame_number: frame_count,
            design_frame_per_second: design_frame_per_second,
            frames: vec![],
            call: FrameCurve::get_curve_call(EFrameCurveType::Easing),
            easing: get_easing_call(&easing_mode),
        }
    }

    /// 曲线 - CubicBezier 插值曲线
    /// 
    /// * [from] - 动画数值起点
    /// * [scalar] - 动画数值变化域值
    /// * [x1,y1,x2,y2] - CubicBezier 曲线参数 (https://cubic-bezier.com/)
    /// 
    pub fn curve_cubic_bezier(from: T, scalar: T, frame_count: FrameIndex, design_frame_per_second: FramePerSecond, x1: KeyFrameCurveValue, y1: KeyFrameCurveValue, x2: KeyFrameCurveValue, y2: KeyFrameCurveValue) -> FrameCurve<T> {
        FrameCurve {
            curve: EFrameCurveType::CubicBezier,
            easing_mode: EEasingMode::None,
            extend_args: [x1, y1, x2, y2],
            key_args: Vec::default(),
            value_offset: from,
            value_scalar: scalar,
            min_frame: 0,
            max_frame: frame_count,
            frame_number: frame_count,
            design_frame_per_second: design_frame_per_second,
            frames: vec![],
            call: FrameCurve::get_curve_call(EFrameCurveType::CubicBezier),
            easing: get_easing_call(&EEasingMode::None),
        }
    }


    /// 曲线 - Hermit插值曲线
    /// 
    /// * [from] - 动画数值起点
    /// * [scalar] - 动画数值变化域值
    /// 
    pub fn curve_minmax_curve(from: T, scalar: T, design_frame_per_second: FramePerSecond) -> FrameCurve<T> {
        FrameCurve {
            curve: EFrameCurveType::MinMaxCurve,
            easing_mode: EEasingMode::None,
            extend_args: [0., 0., 1., 1.],
            key_args: Vec::default(),
            value_offset: from,
            value_scalar: scalar,
            min_frame: FrameIndex::MAX,
            max_frame: FrameIndex::MIN,
            frame_number: 0,
            design_frame_per_second,
            frames: vec![],
            call: FrameCurve::get_curve_call(EFrameCurveType::MinMaxCurve),
            easing: get_easing_call(&EEasingMode::None),
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
    pub fn curve_minmax_curve_frame(mut framecurve: FrameCurve<T>, frame: FrameIndex, value: KeyFrameCurveValue, intangent: KeyFrameCurveValue, outtangent: KeyFrameCurveValue) -> FrameCurve<T> {

        let keyframe = KeyFrame::new(value, [intangent, outtangent]);

        let index = framecurve.frames.binary_search(&frame).unwrap_or_else(|x| x);
        framecurve.frames.insert(index, frame);
        framecurve.key_args.insert(index, keyframe);

        let len = framecurve.frames.len();
        let min = framecurve.frames[0];
        let max = framecurve.frames[len - 1];

        framecurve.min_frame = min;
        framecurve.max_frame = max;
        framecurve.frame_number = max - min;

        return framecurve;
    }

    pub fn get_curve_call(mode: EFrameCurveType) -> fn(&FrameCurve<T>, KeyFrameCurveValue) -> T {
        match mode {
            EFrameCurveType::Linear => FrameCurve::<T>::linear,
            EFrameCurveType::MinMaxCurve => FrameCurve::<T>::minmaxcurve,
            EFrameCurveType::Easing => FrameCurve::<T>::easing,
            EFrameCurveType::CubicBezier => FrameCurve::<T>::cubebezier,
        }
    }

    pub fn linear(curve: &FrameCurve<T>, target_frame: KeyFrameCurveValue) -> T {
        let (pre, next) = FrameCurve::<T>::get_pre_next_frame_index(&curve.frames, target_frame);
        let frame1 = curve.frames[pre];
        let value1 = curve.key_args[pre].value();
        
        let frame2 = curve.frames[next];
        let value2 = curve.key_args[next].value();


        let amount = KeyFrameCurveValue::clamp((target_frame - frame1 as KeyFrameCurveValue) / (frame2 as KeyFrameCurveValue - frame1 as KeyFrameCurveValue), 0., 1.);

        curve.value_offset + curve.value_scalar.scale(value1 + (value2 - value1) * amount)
    }

    pub fn minmaxcurve(curve: &FrameCurve<T>, target_frame: KeyFrameCurveValue) -> T {
        let (pre, next) = FrameCurve::<T>::get_pre_next_frame_index(&curve.frames, target_frame);

        let frame1 = curve.frames[pre];
        let frame2 = curve.frames[next];

        let value1 = curve.key_args[pre].value();
        let value2 = curve.key_args[next].value();

        let tangent1 = curve.key_args[pre].outtangent();
        let tangent2 = curve.key_args[next].intangent();


        let amount = KeyFrameCurveValue::clamp((target_frame - frame1 as KeyFrameCurveValue) / (frame2 as KeyFrameCurveValue - frame1 as KeyFrameCurveValue), 0., 1.);
        let amount = hermite::hermite(value1, tangent1, value2, tangent2, amount);

        curve.value_offset + curve.value_scalar.scale(amount)
    }

    pub fn easing(curve: &FrameCurve<T>, target_frame: KeyFrameCurveValue) -> T {
        let amount = KeyFrameCurveValue::clamp(target_frame / curve.frame_number as KeyFrameCurveValue, 0., 1.);


        let call = &curve.easing;

        let amount = call(amount);

        curve.value_offset + curve.value_scalar.scale(amount)
    }

    pub fn cubebezier(curve: &FrameCurve<T>, target_frame: f32) -> T {
        let amount = KeyFrameCurveValue::clamp(target_frame / curve.frame_number as KeyFrameCurveValue, 0., 1.);

        let amount = bezier::cubic_bezier(curve.extend_args[0], curve.extend_args[1], curve.extend_args[2], curve.extend_args[3], amount);

        curve.value_offset + curve.value_scalar.scale(amount)
    }

    /// 获取目标帧的前后帧在帧数组中的序号
    pub fn get_pre_next_frame_index(frames: &Vec<FrameIndex>, target_frame: KeyFrameCurveValue) -> (usize, usize) {
        let total_num = frames.len();
        match frames.binary_search(&(target_frame as FrameIndex)) {
            Ok(index) => {
                if index >= total_num - 1 {
                    (index, index)
                } else {
                    (index, index + 1)
                }
            },
            Err(index) => {
                if index == 0 {
                    (0, 0)
                } else {
                    (index - 1, index - 1)
                }
            },
        }
    }

}

