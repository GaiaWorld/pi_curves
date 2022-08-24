//!  实现 hermite 曲线相关函数

use nalgebra::RealField;

pub fn hermite<T: RealField + Copy>(value1: T, tangent1: T, value2: T, tangent2: T, amount: T) -> T {
    let _1 = T::from_u8(1).unwrap();
    let _2 = T::from_u8(2).unwrap();
    let _3 = T::from_u8(3).unwrap();

    let squared = amount * amount;
    let cubed = amount * squared;
    let part1 = ((_2 * cubed) - (_3 * squared)) + _1;
    let part2 = (-_2 * cubed) + (_3 * squared);
    let part3 = (cubed - (_2 * squared)) + amount;
    let part4 = cubed - squared;

    return (((value1 * part1) + (value2 * part2)) + (tangent1 * part3)) + (tangent2 * part4);
}