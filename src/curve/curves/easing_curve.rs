use crate::curve::{frame_curve::FrameCurve, frame::{FrameDataValue, KeyFrameCurveValue}};



pub fn interplate_easing<T: FrameDataValue>(curve: &FrameCurve<T>, target_frame: KeyFrameCurveValue) -> T {
    log::trace!(
        "easing, target_frame: {}, frame_number: {}",
        target_frame,
        curve.frame_number
    );
    let mut amount = KeyFrameCurveValue::clamp(
        target_frame / curve.frame_number as KeyFrameCurveValue,
        0.,
        1.,
    );

    let call = &curve.easing;
    amount = call(amount);

    curve.value_offset.as_ref().unwrap().append(curve.value_scalar.as_ref().unwrap(), amount)
}
