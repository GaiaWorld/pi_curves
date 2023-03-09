use crate::{curve::frame::{FrameDataValue, KeyFrameCurveValue}, hermite};

use super::FrameCurve;


pub fn interplate_minmaxcurve<T: FrameDataValue>(curve: &FrameCurve<T>, target_frame: KeyFrameCurveValue) -> T {
    let (pre, next) = FrameCurve::<T>::get_pre_next_frame_index(&curve.frames, target_frame);

    let frame1 = curve.frames[pre];
    let frame2 = curve.frames[next];

    let value1 = curve.minmax_curve_values[pre].value();
    let value2 = curve.minmax_curve_values[next].value();

    let tangent1 = curve.minmax_curve_values[pre].outtangent();
    let tangent2 = curve.minmax_curve_values[next].intangent();

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
        "minmaxcurve, target_frame: {}, frame1: {}, frame2: {}, amount: {}",
        target_frame,
        frame1,
        frame2,
        amount,
    );

    let amount = hermite::hermite(*value1, *tangent1, *value2, *tangent2, amount);

    curve.value_offset.as_ref().unwrap().append(curve.value_scalar.as_ref().unwrap(), amount)
}
