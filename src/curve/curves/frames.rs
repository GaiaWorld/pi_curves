use crate::{curve::frame::{FrameDataValue, KeyFrameCurveValue}};

use super::FrameCurve;



pub fn interplate_frame_values<T: FrameDataValue>(curve: &FrameCurve<T>, target_frame: KeyFrameCurveValue) -> T {
    let (pre, next) = FrameCurve::<T>::get_pre_next_frame_index(&curve.frames, target_frame);
    let frame1 = curve.frames[pre];
    let value1 = curve.values.get(pre).unwrap();

    let frame2 = curve.frames[next];
    let value2 = curve.values.get(next).unwrap();

    let mut amount = if frame1 == frame2 {
        0.0
    } else {
        KeyFrameCurveValue::clamp(
            (target_frame - frame1 as KeyFrameCurveValue)
                / (frame2 as KeyFrameCurveValue - frame1 as KeyFrameCurveValue),
            0.,
            1.,
        )
    };

    log::trace!(
        "frame_values, target_frame: {}, frame1: {}, frame2: {}, amount: {}",
        target_frame,
        frame1,
        frame2,
        amount,
    );

    let call = &curve.easing;
    amount = call(amount);

    value1.interpolate(&value2, amount)
}


pub fn interplate_frame_values_step<T: FrameDataValue>(curve: &FrameCurve<T>, target_frame: KeyFrameCurveValue) -> T {
    let (pre, next) = FrameCurve::<T>::get_pre_next_frame_index(&curve.frames, target_frame);
    let frame1 = curve.frames[pre];
    let value1 = curve.values.get(pre).unwrap();

    let frame2 = curve.frames[next];
    let value2 = curve.values.get(next).unwrap();

    let amount = if frame1 == frame2 {
        0.0
    } else {
        KeyFrameCurveValue::clamp(
            (target_frame - frame1 as KeyFrameCurveValue)
                / (frame2 as KeyFrameCurveValue - frame1 as KeyFrameCurveValue),
            0.,
            1.,
        )
    };

    log::trace!(
        "frame_values, target_frame: {}, frame1: {}, frame2: {}, amount: {}",
        target_frame,
        frame1,
        frame2,
        amount,
    );

    if amount < 0.5 {
        value1.clone()
    } else {
        value2.clone()
    }
}