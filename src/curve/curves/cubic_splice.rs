use crate::{curve::{frame::{FrameDataValue, KeyFrameCurveValue}, curves::FrameCurve}, amount::AnimationAmountCalc};


pub fn interplate_cubic_splice<T: FrameDataValue>(curve: &FrameCurve<T>, target_frame: KeyFrameCurveValue, amountcalc: &AnimationAmountCalc) -> T {
    let (pre, next) = FrameCurve::<T>::get_pre_next_frame_index(&curve.frames, target_frame);

    let frame1 = curve.frames[pre];
    let frame2 = curve.frames[next];

    let value1 = curve.cubic_spline_values[pre].value();
    let value2 = curve.cubic_spline_values[next].value();

    let tangent1 = curve.cubic_spline_values[pre].outtangent();
    let tangent2 = curve.cubic_spline_values[next].intangent();

    let mut frame_delta = frame2 as KeyFrameCurveValue - frame1 as KeyFrameCurveValue;

    let amount = if frame1 == frame2 {
        0.0
    } else {
        KeyFrameCurveValue::clamp(
            amountcalc.calc(
            (target_frame - frame1 as KeyFrameCurveValue)
                / frame_delta
            ),
            0.,
            1.,
        )
    };

    log::trace!(
        "minmaxcurve, target_frame: {}, frame1: {}, frame2: {}, amount: {}",
        target_frame,
        frame1,
        frame2,
        amount,
    );

    frame_delta = frame_delta / (curve.design_frame_per_second as KeyFrameCurveValue);

    T::hermite(value1, tangent1, value2, tangent2, amount, frame_delta)
}