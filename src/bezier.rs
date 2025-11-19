//!  实现 bezier 曲线相关函数

use nalgebra::RealField;

pub fn cubic_bezier<T: RealField + Copy>(_x1: T, _y1: T, _x2: T, _y2: T, t: T) -> T {
    let _0  = T::from_f32(0.0).unwrap();
    let _1  = T::from_f32(1.0).unwrap();
    let _2  = T::from_f32(2.0).unwrap();
    let _3  = T::from_f32(3.0).unwrap();
    let _6  = T::from_f32(6.0).unwrap();

    let f0  = _1 - _3 * _x2 + _3 * _x1;
    let f1  = _3 * _x2 - _6 * _x1;
    let f2  = _3 * _x1;

    let mut refined_t = t;

    for _ in 0..5 {
        let refined_t2 = refined_t * refined_t;
        let refined_t3 = refined_t2 * refined_t;

        let x = f0 * refined_t3 + f1 * refined_t2 + f2 * refined_t;
        let slop = _1 / (_3 * f0 * refined_t2 + _2 * f1 * refined_t + f2);

        refined_t -= (x - t) * slop;
        refined_t = T::min(_1, T::max(_0, refined_t));
    };

    _3 * T::powi(_1 - refined_t, 2) * refined_t * _y1 + _3 * (_1 - refined_t) * T::powi(refined_t, 2) * _y2 + T::powi(refined_t, 3)
}

#[test]
pub fn test_cubc_bezier() {

}