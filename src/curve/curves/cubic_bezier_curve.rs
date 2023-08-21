use crate::{curve::{frame_curve::FrameCurve, frame::{FrameDataValue, KeyFrameCurveValue}}, bezier, amount::AnimationAmountCalc};


pub fn interplate_cubebezier<T: FrameDataValue>(curve: &FrameCurve<T>, target_frame: f32, _amountcalc: &AnimationAmountCalc) -> T {
    let amount = KeyFrameCurveValue::clamp(
        target_frame / curve.frame_number as KeyFrameCurveValue,
        0.,
        1.,
    );

    let amount = bezier::cubic_bezier(
        curve.cubic_bezier_args[0],
        curve.cubic_bezier_args[1],
        curve.cubic_bezier_args[2],
        curve.cubic_bezier_args[3],
        amount,
    );

    log::trace!(
        "cubebezier, target_frame: {}, x1: {:?}, y1: {:?}, x2: {:?}, y2s: {:?}",
        target_frame,
        curve.cubic_bezier_args[0],
        curve.cubic_bezier_args[1],
        curve.cubic_bezier_args[2],
        curve.cubic_bezier_args[3],
    );

    curve.value_offset.as_ref().unwrap().append(curve.value_scalar.as_ref().unwrap(), amount)
}