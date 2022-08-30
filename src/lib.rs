use curve::frame::KeyFrameCurveValue;
use easing::{EEasingMode, function::*};
use steps::EStepMode;


extern crate nalgebra;

pub mod hermite;
pub mod bezier;
pub mod easing;
pub mod curve;
pub mod steps;

/// 进度计算参数
/// 对于 Step 模式 第一个参数为 步进数目
/// 对于 CubicBezier 模式 四个参数分别对应 (x1, y1, x2, y2)
#[derive(Debug, Clone, Copy)]
pub struct AmountParam(pub KeyFrameCurveValue, pub KeyFrameCurveValue, pub KeyFrameCurveValue, pub KeyFrameCurveValue);

impl Default for AmountParam {
    fn default() -> Self {
        Self( 0., 0., 0., 0. )
    }
}

/// 进度计算模式
#[derive(Debug, Clone, Copy)]
pub enum EAmountMode {
    None,
    Easing(EEasingMode),
    Steps(EStepMode),
    CubicBezier,
}

impl EAmountMode {
    pub fn get_transform_amount_call(mode: EAmountMode) -> fn(KeyFrameCurveValue, &AmountParam) -> KeyFrameCurveValue {
        match mode {
            EAmountMode::None => Self::calc_amount_none,
            EAmountMode::Easing(mode) => Self::get_calc_amount_easing(mode),
            EAmountMode::Steps(mode) => Self::get_calc_amount_steps(mode),
            EAmountMode::CubicBezier => Self::calc_amount_cubic_bezier,
        }
    }
    
    fn get_calc_amount_easing(mode: EEasingMode) -> fn(KeyFrameCurveValue, &AmountParam) -> KeyFrameCurveValue {
        match mode {
            EEasingMode::None           => amount_linear_in        ,
            EEasingMode::BackIn         => amount_back_in          ,
            EEasingMode::BackOut        => amount_back_out         ,
            EEasingMode::BackInOut      => amount_back_in_out      ,
            EEasingMode::CircleIn       => amount_circle_in        ,
            EEasingMode::CircleOut      => amount_circle_out       ,
            EEasingMode::CircleInOut    => amount_circle_in_out    ,
            EEasingMode::CubicIn        => amount_cubic_in         ,
            EEasingMode::CubicOut       => amount_cubic_out        ,
            EEasingMode::CubicInOut     => amount_cubic_in_out     ,
            EEasingMode::SineIn         => amount_sine_in          ,
            EEasingMode::SineOut        => amount_sine_out         ,
            EEasingMode::SineInOut      => amount_sine_in_out      ,
            EEasingMode::QuadIn         => amount_quad_in          ,
            EEasingMode::QuadOut        => amount_quad_out         ,
            EEasingMode::QuadInOut      => amount_quad_in_out      ,
            EEasingMode::QuartIn        => amount_quart_in         ,
            EEasingMode::QuartOut       => amount_quart_out        ,
            EEasingMode::QuartInOut     => amount_quart_in_out     ,
            EEasingMode::QuintIn        => amount_quint_in         ,
            EEasingMode::QuintOut       => amount_quint_out        ,
            EEasingMode::QuintInOut     => amount_quint_in_out     ,
            EEasingMode::ExpoIn         => amount_expo_in          ,
            EEasingMode::ExpoOut        => amount_expo_out         ,
            EEasingMode::ExpoInOut      => amount_expo_in_out      ,
            EEasingMode::ElasticIn      => amount_elastic_in       ,
            EEasingMode::ElasticOut     => amount_elastic_out      ,
            EEasingMode::ElasticInOut   => amount_elastic_in_out   ,
            EEasingMode::BounceIn       => amount_bounce_in        ,
            EEasingMode::BounceOut      => amount_bounce_out       ,
            EEasingMode::BounceInOut    => amount_bounce_in_out    ,
        }
    }


    fn get_calc_amount_steps(mode: EStepMode) -> fn(KeyFrameCurveValue, &AmountParam) -> KeyFrameCurveValue {
        match mode {
            EStepMode::JumpStart => amount_step_start,
            EStepMode::JumpEnd => amount_step_end,
            EStepMode::JumpNone => amount_step_none,
            EStepMode::JumpBoth => amount_step_both,
        }
    }

    fn calc_amount_none(amount: KeyFrameCurveValue, param: &AmountParam) -> KeyFrameCurveValue{
        amount
    }

    fn calc_amount_cubic_bezier(amount: KeyFrameCurveValue, param: &AmountParam) -> KeyFrameCurveValue {
        let x1 = param.0;
        let y1 = param.1;
        let x2 = param.2;
        let y2 = param.3;
        bezier::cubic_bezier(x1, y1, x2, y2, amount)
    }
}

pub fn amount_step_start(x: KeyFrameCurveValue, param: &AmountParam) -> KeyFrameCurveValue {
    let t = 1.0 / param.0 as KeyFrameCurveValue;
    let ix = (x / t).floor();
    let ix = (ix + 1.) * t;

    ix
}
pub fn amount_step_end(x: KeyFrameCurveValue, param: &AmountParam) -> KeyFrameCurveValue {
    let t = 1.0 / param.0 as KeyFrameCurveValue;
    let ix = (x / t).floor();
    let ix = ix * t;

    ix
}
pub fn amount_step_none(x: KeyFrameCurveValue, param: &AmountParam) -> KeyFrameCurveValue {
    let t = 1.0 / param.0 as KeyFrameCurveValue;
    let ix = (x / t).floor();
    let t = 1.0 / (param.0 as KeyFrameCurveValue - 1.0);
    let ix = ix * t;

    ix
}
pub fn amount_step_both(x: KeyFrameCurveValue, param: &AmountParam) -> KeyFrameCurveValue {
    let t = 1.0 / param.0 as KeyFrameCurveValue;
    let ix = (x / t).floor();
    let t = 1.0 / (param.0 as KeyFrameCurveValue + 1.0);
    let ix = (ix + 1.0) * t;

    ix
}

pub fn amount_back_in(x: KeyFrameCurveValue, _: &AmountParam) -> KeyFrameCurveValue {
    back_in(x)
}
pub fn amount_back_out(x: KeyFrameCurveValue, _: &AmountParam) -> KeyFrameCurveValue {
    back_out(x)
}
pub fn amount_back_in_out(x: KeyFrameCurveValue, _: &AmountParam) -> KeyFrameCurveValue {
    back_in_out(x)
}

pub fn amount_bounce_in(x: KeyFrameCurveValue, _: &AmountParam) -> KeyFrameCurveValue {
    bounce_in(x)
}
pub fn amount_bounce_out(x: KeyFrameCurveValue, _: &AmountParam) -> KeyFrameCurveValue {
    bounce_out(x)
}
pub fn amount_bounce_in_out(x: KeyFrameCurveValue, _: &AmountParam) -> KeyFrameCurveValue {
    bounce_in_out(x)
}

pub fn amount_circle_in(x: KeyFrameCurveValue, _: &AmountParam) -> KeyFrameCurveValue {
    circle_in(x)
}
pub fn amount_circle_out(x: KeyFrameCurveValue, _: &AmountParam) -> KeyFrameCurveValue {
    circle_out(x)
}
pub fn amount_circle_in_out(x: KeyFrameCurveValue, _: &AmountParam) -> KeyFrameCurveValue {
    circle_in_out(x)
}

pub fn amount_cubic_in(x: KeyFrameCurveValue, _: &AmountParam) -> KeyFrameCurveValue {
    cubic_in(x)
}
pub fn amount_cubic_out(x: KeyFrameCurveValue, _: &AmountParam) -> KeyFrameCurveValue {
    cubic_out(x)
}
pub fn amount_cubic_in_out(x: KeyFrameCurveValue, _: &AmountParam) -> KeyFrameCurveValue {
    cubic_in_out(x)
}

pub fn amount_quad_in(x: KeyFrameCurveValue, _: &AmountParam) -> KeyFrameCurveValue {
    quad_in(x)
}
pub fn amount_quad_out(x: KeyFrameCurveValue, _: &AmountParam) -> KeyFrameCurveValue {
    quad_out(x)
}
pub fn amount_quad_in_out(x: KeyFrameCurveValue, _: &AmountParam) -> KeyFrameCurveValue {
    quad_in_out(x)
}

pub fn amount_quart_in(x: KeyFrameCurveValue, _: &AmountParam) -> KeyFrameCurveValue {
    quart_in(x)
}
pub fn amount_quart_out(x: KeyFrameCurveValue, _: &AmountParam) -> KeyFrameCurveValue {
    quart_out(x)
}
pub fn amount_quart_in_out(x: KeyFrameCurveValue, _: &AmountParam) -> KeyFrameCurveValue {
    quart_in_out(x)
}

pub fn amount_quint_in(x: KeyFrameCurveValue, _: &AmountParam) -> KeyFrameCurveValue {
    quint_in(x)
}
pub fn amount_quint_out(x: KeyFrameCurveValue, _: &AmountParam) -> KeyFrameCurveValue {
    quint_out(x)
}
pub fn amount_quint_in_out(x: KeyFrameCurveValue, _: &AmountParam) -> KeyFrameCurveValue {
    quint_in_out(x)
}

pub fn amount_sine_in(x: KeyFrameCurveValue, _: &AmountParam) -> KeyFrameCurveValue {
    sine_in(x)
}
pub fn amount_sine_out(x: KeyFrameCurveValue, _: &AmountParam) -> KeyFrameCurveValue {
    sine_out(x)
}
pub fn amount_sine_in_out(x: KeyFrameCurveValue, _: &AmountParam) -> KeyFrameCurveValue {
    sine_in_out(x)
}

pub fn amount_expo_in(x: KeyFrameCurveValue, _: &AmountParam) -> KeyFrameCurveValue {
    expo_in(x)
}
pub fn amount_expo_out(x: KeyFrameCurveValue, _: &AmountParam) -> KeyFrameCurveValue {
    expo_out(x)
}
pub fn amount_expo_in_out(x: KeyFrameCurveValue, _: &AmountParam) -> KeyFrameCurveValue {
    expo_in_out(x)
}

pub fn amount_elastic_in(x: KeyFrameCurveValue, _: &AmountParam) -> KeyFrameCurveValue {
    elastic_in(x)
}
pub fn amount_elastic_out(x: KeyFrameCurveValue, _: &AmountParam) -> KeyFrameCurveValue {
    elastic_out(x)
}
pub fn amount_elastic_in_out(x: KeyFrameCurveValue, _: &AmountParam) -> KeyFrameCurveValue {
    elastic_in_out(x)
}

pub fn amount_linear_in(x: KeyFrameCurveValue, _: &AmountParam) -> KeyFrameCurveValue {
    linear_in(x)
}