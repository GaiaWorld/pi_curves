#![feature(test)]
extern crate test;

#[cfg(test)]
mod test_frame {

    use std::ops::Add;

    use pi_curves::{curve::{frame::{FrameDataValue, KeyFrameCurveValue, FrameValueScale}, frame_curve::FrameCurve, FrameIndex}, easing::{EEasingMode, function::sine_in_out}, amount::AnimationAmountCalc};
    use test::Bencher;

    #[bench]
    fn test_minmaxcurve_peformance(b: &mut Bencher) {
    
            let frame_count = 60 as FrameIndex;
            let mut curves = vec![];

        for i in 0..1_000_000 {

            // MinMaxCurve
            let mut key_frames = FrameCurve::curve_minmax_curve(0.0f32, 1.0f32, 60);
            FrameCurve::curve_minmax_curve_frame(&mut key_frames, 0, 0.0f32, 2.0f32, 2.0f32);
            FrameCurve::curve_minmax_curve_frame(&mut key_frames, (frame_count/2) as FrameIndex, 0.5f32, 0.0f32, 0.0f32);
            FrameCurve::curve_minmax_curve_frame(&mut key_frames, frame_count as FrameIndex, 1.0f32, 2.0f32, 2.0f32);

            curves.push(
                key_frames
            );
        }
        b.iter(move || {
            let mut v = 0.;
            for i in 0..1_000_000 {
                v = v + curves.get(i).unwrap().interple(10.0, &AnimationAmountCalc::default());
            }
        });
    }
    
    #[bench]
    fn test_linear_peformance(b: &mut Bencher) {
    
            let frame_count = 60;
            let mut curves = vec![];

        for i in 0..1_000_000 {
            let mut key_frames = FrameCurve::curve_frame_values(60);
            FrameCurve::curve_frame_values_frame(&mut key_frames, 0, 0.0f32);
            FrameCurve::curve_frame_values_frame(&mut key_frames, frame_count as FrameIndex, 1.0f32);

            curves.push(
                key_frames
            );
        }
        b.iter(move || {
            let mut v = 0.;
            for i in 0..1_000_000 {
                v = v + curves.get(i).unwrap().interple(10.0, &AnimationAmountCalc::default());
            }
        });
    }   
    #[bench]
    fn test_easing_peformance(b: &mut Bencher) {
    
            let frame_count = 60;
            let mut curves = vec![];

        for i in 0..1_000_000 {
            let key_frames = FrameCurve::curve_easing(0.0, 1.0, frame_count as FrameIndex, frame_count, EEasingMode::None);

            curves.push(
                key_frames
            );
        }

        let mut vs = Vec::with_capacity(30000);
        let mut amount = 0.5;
        let target_frame = 10.0;
        b.iter(|| {
            // for i in 0..1_000_000 {
            let mut v = 0.0;
            for i in 0..1_000_000 {
                let frames = curves.get(i).unwrap();

                v += frames.interple(amount, &AnimationAmountCalc::default());

                // amount = target_frame / frames.frame_number as f32;
                // amount = f32::clamp(amount, 0., 1.0);
                // v += frames.value_offset.unwrap().x + sine_in_out(0.5) * frames.value_scalar.unwrap().x *  frames.value_scalar.unwrap().x;
            }
            vs.push(v);
        });
        println!("============= v.len() = {}", vs.len());
    }
    
    #[bench]
    fn test_steps_peformance(b: &mut Bencher) {
    
            let frame_count = 60;
            let mut curves = vec![];

        for i in 0..1_000_000 {
            let mut key_frames = FrameCurve::curve_frame_values(60);
            FrameCurve::curve_frame_values_frame(&mut key_frames, 0, 0.0f32);
            FrameCurve::curve_frame_values_frame(&mut key_frames, 30, 0.2f32);
            FrameCurve::curve_frame_values_frame(&mut key_frames, frame_count as FrameIndex, 1.0f32);

            curves.push(
                key_frames
            );
        }
        b.iter(move || {
            let mut v = 0.;
            for i in 0..1_000_000 {
                v = v + curves.get(i).unwrap().interple(10.0, &AnimationAmountCalc::default());
            }
        });
    }   
}