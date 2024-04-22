use crate::{amount::AnimationAmountCalc, curve::{curves::get_pre_next_frame_index, frame::{FrameDataValue, KeyFrameCurveValue}, FrameIndex}};

use super::FrameCurve;



pub fn interplate_frame_values<T: FrameDataValue>(curve: &FrameCurve<T>, target_frame: KeyFrameCurveValue, amountcalc: &AnimationAmountCalc) -> T {
    let (pre, next, amount) = _interplate_frame_values_amount(&curve.frames, target_frame, amountcalc);
    let value1 = curve.values.get(pre).unwrap();
    let value2 = curve.values.get(next).unwrap();
    value1.interpolate(&value2, amount)
}


pub fn interplate_frame_values_step<T: FrameDataValue>(curve: &FrameCurve<T>, target_frame: KeyFrameCurveValue, amountcalc: &AnimationAmountCalc) -> T {
    let (pre, next, amount) = _interplate_frame_values_amount(&curve.frames, target_frame, amountcalc);
    let value1 = curve.values.get(pre).unwrap();
    let value2 = curve.values.get(next).unwrap();

    if amount < 0.5 {
        value1.clone()
    } else {
        value2.clone()
    }
}

fn _interplate_frame_values_amount(frames: &Vec<FrameIndex>, target_frame: KeyFrameCurveValue, amountcalc: &AnimationAmountCalc) -> (usize, usize, KeyFrameCurveValue) {
    let (pre, next) = get_pre_next_frame_index(frames, target_frame);
    let frame1 = frames[pre];

    let frame2 = frames[next];

    let amount = if frame1 == frame2 {
        0.0
    } else {
        KeyFrameCurveValue::clamp(
            amountcalc.calc(
                (target_frame - frame1 as KeyFrameCurveValue)
                / (frame2 as KeyFrameCurveValue - frame1 as KeyFrameCurveValue)
            ),
            0.,
            1.,
        )
    };

    // log::trace!(
    //     "frame_values, target_frame: {}, frame1: {}, frame2: {}, amount: {}",
    //     target_frame,
    //     frame1,
    //     frame2,
    //     amount,
    // );

    return (pre, next, amount); 
}