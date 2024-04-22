use crate::{amount::AnimationAmountCalc, curve::{curves::get_pre_next_frame_index, frame::{CurveFrameValue, FrameDataValue, KeyFrameCurveValue}, FrameIndex}, hermite};

use super::FrameCurve;


pub fn interplate_minmaxcurve<T: FrameDataValue>(curve: &FrameCurve<T>, target_frame: KeyFrameCurveValue, amountcalc: &AnimationAmountCalc) -> T {
    let amount = _interplate_minmaxcurve_amount(&curve.frames, target_frame, &curve.minmax_curve_values, amountcalc);
    curve.value_offset.as_ref().unwrap().append(curve.value_scalar.as_ref().unwrap(), amount)
}

fn _interplate_minmaxcurve_amount(frames: &Vec<FrameIndex>, target_frame: KeyFrameCurveValue, minmax_curve_values: &Vec<CurveFrameValue<KeyFrameCurveValue>>, amountcalc: &AnimationAmountCalc) -> KeyFrameCurveValue {
    let (pre, next) = get_pre_next_frame_index(frames, target_frame);

    let frame1 = frames[pre];
    let frame2 = frames[next];

    let value1 = minmax_curve_values[pre].value();
    let value2 = minmax_curve_values[next].value();

    let tangent1 = minmax_curve_values[pre].outtangent();
    let tangent2 = minmax_curve_values[next].intangent();

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
    //     "minmaxcurve, target_frame: {}, frame1: {}, frame2: {}, amount: {}",
    //     target_frame,
    //     frame1,
    //     frame2,
    //     amount,
    // );

    let amount = hermite::hermite(*value1, *tangent1, *value2, *tangent2, amount);
    return amount;
}
