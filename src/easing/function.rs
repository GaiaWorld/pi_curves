//! 实现 Easing 缓动函数

use nalgebra::RealField;
use crate::easing::EEasingMode;

/// https://easings.net/# - 缓动函数实现
pub fn easing_call<T: RealField + Copy>(x: T, mode: &EEasingMode) -> T {
    match mode {
        EEasingMode::None           => linear_in        (x),
        EEasingMode::BackIn         => back_in          (x),
        EEasingMode::BackOut        => back_out         (x),
        EEasingMode::BackInOut      => back_in_out      (x),
        EEasingMode::CircleIn       => circle_in        (x),
        EEasingMode::CircleOut      => circle_out       (x),
        EEasingMode::CircleInOut    => circle_in_out    (x),
        EEasingMode::CubicIn        => cubic_in         (x),
        EEasingMode::CubicOut       => cubic_out        (x),
        EEasingMode::CubicInOut     => cubic_in_out     (x),
        EEasingMode::SineIn         => sine_in          (x),
        EEasingMode::SineOut        => sine_out         (x),
        EEasingMode::SineInOut      => sine_in_out      (x),
        EEasingMode::QuadIn         => quad_in          (x),
        EEasingMode::QuadOut        => quad_out         (x),
        EEasingMode::QuadInOut      => quad_in_out      (x),
        EEasingMode::QuartIn        => quart_in         (x),
        EEasingMode::QuartOut       => quart_out        (x),
        EEasingMode::QuartInOut     => quart_in_out     (x),
        EEasingMode::QuintIn        => quint_in         (x),
        EEasingMode::QuintOut       => quint_out        (x),
        EEasingMode::QuintInOut     => quint_in_out     (x),
        EEasingMode::ExpoIn         => expo_in          (x),
        EEasingMode::ExpoOut        => expo_out         (x),
        EEasingMode::ExpoInOut      => expo_in_out      (x),
        EEasingMode::ElasticIn      => elastic_in       (x),
        EEasingMode::ElasticOut     => elastic_out      (x),
        EEasingMode::ElasticInOut   => elastic_in_out   (x),
        EEasingMode::BounceIn       => bounce_in        (x),
        EEasingMode::BounceOut      => bounce_out       (x),
        EEasingMode::BounceInOut    => bounce_in_out    (x),
    }
}

pub fn get_easing_call<T: RealField + Copy>(mode: &EEasingMode) -> fn(T) -> T {
    match mode {
        EEasingMode::None           => linear_in        ,
        EEasingMode::BackIn         => back_in          ,
        EEasingMode::BackOut        => back_out         ,
        EEasingMode::BackInOut      => back_in_out      ,
        EEasingMode::CircleIn       => circle_in        ,
        EEasingMode::CircleOut      => circle_out       ,
        EEasingMode::CircleInOut    => circle_in_out    ,
        EEasingMode::CubicIn        => cubic_in         ,
        EEasingMode::CubicOut       => cubic_out        ,
        EEasingMode::CubicInOut     => cubic_in_out     ,
        EEasingMode::SineIn         => sine_in          ,
        EEasingMode::SineOut        => sine_out         ,
        EEasingMode::SineInOut      => sine_in_out      ,
        EEasingMode::QuadIn         => quad_in          ,
        EEasingMode::QuadOut        => quad_out         ,
        EEasingMode::QuadInOut      => quad_in_out      ,
        EEasingMode::QuartIn        => quart_in         ,
        EEasingMode::QuartOut       => quart_out        ,
        EEasingMode::QuartInOut     => quart_in_out     ,
        EEasingMode::QuintIn        => quint_in         ,
        EEasingMode::QuintOut       => quint_out        ,
        EEasingMode::QuintInOut     => quint_in_out     ,
        EEasingMode::ExpoIn         => expo_in          ,
        EEasingMode::ExpoOut        => expo_out         ,
        EEasingMode::ExpoInOut      => expo_in_out      ,
        EEasingMode::ElasticIn      => elastic_in       ,
        EEasingMode::ElasticOut     => elastic_out      ,
        EEasingMode::ElasticInOut   => elastic_in_out   ,
        EEasingMode::BounceIn       => bounce_in        ,
        EEasingMode::BounceOut      => bounce_out       ,
        EEasingMode::BounceInOut    => bounce_in_out    ,
    }
}

pub fn back_in<T: RealField + Copy>(x: T) -> T {
    let xx = x * x;
    let c1 = T::from_f32(1.70158).unwrap();
    let c3 = c1 + T::one();

    return c3 * x * xx - c1 * xx;
}
pub fn back_out<T: RealField + Copy>(x: T) -> T {
    let c1 = T::from_f32(1.70158).unwrap();
    let c3 = c1 + T::one();

    let temp = x - T::one();

    return T::one() + c3 * temp.powi(3) + c1 * temp.powi(2);
}
pub fn back_in_out<T: RealField + Copy>(x: T) -> T {
    let c1 = T::from_f32(1.70158).unwrap();
    let c2 = c1 * T::from_f32(1.525).unwrap();

    let _1 = T::one();
    let _2 = T::from_u8(2).unwrap();
    let _2x = x * T::from_u8(2).unwrap();

    if x < T::from_f32(0.5).unwrap() {
        T::powi(_2x, 2) * ((c2 + _1) * _2x - c2) / _2
    }
    else {
        (T::powi(_2x - _2, 2) * ((c2 + _1) * (_2x - _2) + c2) + _2) / _2
    }
}

pub fn bounce_in<T: RealField + Copy>(x: T) -> T {
    T::one() - bounce_out(T::one() - x)
}
pub fn bounce_out<T: RealField + Copy>(mut x: T) -> T {
    let n1 = T::from_f32(7.5625).unwrap();
    let d1 = T::from_f32(2.75).unwrap();

    if x < T::one() / d1 {
        return n1 * x * x;
    } else if x < T::from_f32(2.0).unwrap() / d1 {
        x -= T::from_f32(1.5).unwrap() / d1;

        return n1 * x * x + T::from_f32(0.75).unwrap();
    } else if x < T::from_f32(2.5).unwrap() / d1 {
        x -= T::from_f32(2.25).unwrap() / d1;

        return n1 * x * x + T::from_f32(0.9375).unwrap();
    } else {
        x -= T::from_f32(2.625).unwrap() / d1;

        return n1 * x * x + T::from_f32(0.984375).unwrap();
    }
}
pub fn bounce_in_out<T: RealField + Copy>(x: T) -> T {
    let _0_5 = T::from_f32(0.5).unwrap();
    let _1 = T::one();

    if x < _0_5 {
        (_1 - bounce_out(_1 - (x + x))) * _0_5
    }
    else {
        (_1 + bounce_out((x + x) - _1)) * _0_5
    }
}

pub fn circle_in<T: RealField + Copy>(x: T) -> T {
    let _0 = T::zero();
    let _1 = T::one();

    _1 - T::try_sqrt(_1 - T::powi(x, 2)).unwrap()
}
pub fn circle_out<T: RealField + Copy>(x: T) -> T {
    let _1 = T::one();

    // _1 - circle_in(_1 - x)
    T::try_sqrt(_1 - T::powi(_1 - x, 2)).unwrap()
}
pub fn circle_in_out<T: RealField + Copy>(x: T) -> T {
    let _0_5 = T::from_f32(0.5).unwrap();
    let _2 = T::from_u8(2).unwrap();
    let _1 = T::one();

    if x < _0_5 {
        circle_in(x * _2) / _2
    }
    else {
        (T::try_sqrt(_1 - T::powi(_2 - x - x, 2)).unwrap() + _1) / _2
    }
}

pub fn cubic_in<T: RealField + Copy>(x: T) -> T {
    T::powi(x, 3)
}
pub fn cubic_out<T: RealField + Copy>(x: T) -> T {
    let _1 = T::one();

    _1 - cubic_in(_1 - x)
}
pub fn cubic_in_out<T: RealField + Copy>(x: T) -> T {
    let _0_5 = T::from_f32(0.5).unwrap();
    let _1 = T::one();

    if x < _0_5 {
        let _16 = T::from_u8(16).unwrap();
        let xx = x * x;
        _16 * xx * xx * x
    }
    else {
        let _2 = T::from_u8(2).unwrap();
        _1 - T::powi(- _2 * x + _2, 3) / _2
    }
}

pub fn quad_in<T: RealField + Copy>(x: T) -> T {
    x * x
}
pub fn quad_out<T: RealField + Copy>(x: T) -> T {
    let _1 = T::one();

    _1 - quad_in(_1 - x)
}
pub fn quad_in_out<T: RealField + Copy>(x: T) -> T {
    let _0_5 = T::from_f32(0.5).unwrap();
    let _1 = T::one();
    let _2 = T::from_u8(2).unwrap();

    if x < _0_5 {
        quad_in(x) * _2
    }
    else {
        _1 - quad_in(_2 * (_1 - x)) * _0_5
    }
}

pub fn quart_in<T: RealField + Copy>(x: T) -> T {
    T::powi(x, 4)
}
pub fn quart_out<T: RealField + Copy>(x: T) -> T {
    let _1 = T::one();

    _1 - T::powi(_1 - x, 4)
}
pub fn quart_in_out<T: RealField + Copy>(x: T) -> T {
    let _0_5 = T::from_f32(0.5).unwrap();
    let _1 = T::one();
    let _8 = T::from_u8(8).unwrap();

    if x < _0_5 {
        _8 * T::powi(x, 4)
    }
    else {
        let t = -x -x + _1 + _1; // 2-2x
        _1 - T::powi(t, 4) * _0_5
    }
}

pub fn quint_in<T: RealField + Copy>(x: T) -> T {
    T::powi(x, 5)
}
pub fn quint_out<T: RealField + Copy>(x: T) -> T {
    let _1 = T::one();
    
    _1 - quint_in(_1 - x)
}
pub fn quint_in_out<T: RealField + Copy>(x: T) -> T {
    let _0_5 = T::from_f32(0.5).unwrap();
    let _16 = T::from_u8(16).unwrap();
    let _1 = T::one();

    if x < _0_5 {
        _16 * quint_in(x)
    }
    else {
        _1 - quint_in(_1 + _1 - x - x) * _0_5
    }
}

pub fn sine_in<T: RealField + Copy>(x: T) -> T {
    let _1 = T::one();
    let _2 = T::from_u8(2).unwrap();

    _1 - (x * T::pi() / _2).cos()
}
pub fn sine_out<T: RealField + Copy>(x: T) -> T {
    let _2 = T::from_u8(2).unwrap();

    (x * T::pi() / _2).sin()
}
pub fn sine_in_out<T: RealField + Copy>(x: T) -> T {
    let _1 = T::one();
    let _2 = T::from_u8(2).unwrap();

    -((x * T::pi()).cos() - _1) / _2
}

pub fn expo_in<T: RealField + Copy>(x: T) -> T {
    if x == T::zero() {
        T::zero()
    }
    else {
        let _2 = T::from_usize(2).unwrap();
        let _10 = T::from_usize(10).unwrap();
        _2.powc(_10 * x - _10)
    }
}
pub fn expo_out<T: RealField + Copy>(x: T) -> T {
    let _1 = T::one();
    if x == _1 {
        T::one()
    }
    else {
        let _2 = T::from_usize(2).unwrap();
        let _10 = T::from_usize(10).unwrap();
        _1 - _2.powc(-_10 * x)
    }
}
pub fn expo_in_out<T: RealField + Copy>(x: T) -> T {
    
    let _1 = T::one();
    let _0_5 = T::from_f32(0.5).unwrap();

    if x == T::zero() {
        T::zero()
    }
    else if x == _1 {
        T::one()
    }
    else if x < _0_5 {
        let _2 = T::from_usize(2).unwrap();
        let _10 = T::from_usize(10).unwrap();
        let _20 = T::from_usize(20).unwrap();

        _2.powc(_20 * x - _10) / _2
    }
    else {
        let _2 = T::from_usize(2).unwrap();
        let _10 = T::from_usize(10).unwrap();
        let _20 = T::from_usize(20).unwrap();

        _1 - _2.powc(-_20 * x + _10) / _2
    }
}

pub fn elastic_in<T: RealField + Copy>(x: T) -> T {
    let _0 = T::zero();
    let _0_5 = T::from_f32(0.5).unwrap();
    let _1 = T::one();
    let _2 = T::from_usize(2).unwrap();
    let _3 = T::from_usize(3).unwrap();
    let _10 = T::from_usize(10).unwrap();
    let _20 = T::from_usize(20).unwrap();
    let _t = T::from_f32(10.75).unwrap();

    let c4 = _2 * T::pi() /_3;
    
    if x == _0 {
        _0
    }
    else if x == _1 {
        _1
    }
    else {
        -_2.powc(_10 * x - _10) * ((_10 * x - _t) * c4).sin()
    }
}
pub fn elastic_out<T: RealField + Copy>(x: T) -> T {
    let _0 = T::zero();
    let _0_5 = T::from_f32(0.5).unwrap();
    let _1 = T::one();
    let _2 = T::from_usize(2).unwrap();
    let _3 = T::from_usize(3).unwrap();
    let _10 = T::from_usize(10).unwrap();
    let _20 = T::from_usize(20).unwrap();

    let _t = T::from_f32(0.75).unwrap();

    let c4 = _2 * T::pi() /_3;
    
    if x == _0 {
        _0
    }
    else if x == _1 {
        _1
    }
    else {
        _2.powc(-_10 * x) * ((_10 * x - _t) * c4).sin() + _1
    }
}
pub fn elastic_in_out<T: RealField + Copy>(x: T) -> T {
    let _0 = T::zero();
    let _0_5 = T::from_f32(0.5).unwrap();
    let _1 = T::one();
    let _2 = T::from_usize(2).unwrap();
    let _3 = T::from_usize(3).unwrap();
    let _10 = T::from_usize(10).unwrap();
    let _20 = T::from_usize(20).unwrap();

    let _4_5 = T::from_f32(4.5).unwrap();
    let _t = T::from_f32(11.125).unwrap();

    let c5 = _2 * T::pi() / _4_5;
    
    if x == _0 {
        _0
    }
    else if x == _1 {
        _1
    }
    else if x < _0_5 {
        -_2.powc(_20 * x - _10) * ((_20 * x - _t) * c5).sin() / _2
    }
    else {
        _2.powc(-_20 * x + _10) * ((_20 * x - _t) * c5).sin() / _2 + _1
    }
}

pub fn linear_in<T: RealField + Copy>(x: T) -> T {
    x
}