use pi_curves;
extern crate nalgebra;

// 简单的三次贝塞尔函数（使用 De Casteljau 算法）
fn simple_cubic_bezier(t: f32, x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let t2 = t * t;
    let t3 = t2 * t;
    (y1 * (1.0 - x2) * t3 + (y2 - 2.0 * y1 + 2.0) * t2 + y1 * t) / 
    ((1.0 - x2) * t3 + (3.0 * x2 - 2.0) * t2 + (2.0 - 3.0 * x2) * t + 1.0)
}

#[test]
fn test() {

    env_logger::init();

    let x1 = 0.42;
    let y1 = 0.;
    let x2 = 1.;
    let y2 = 1.;

    // let d = 101;
    // let df = (d - 1) as f32;
    // for i in 0..d {
    //     let result = pi_curves::bezier::cubic_bezier(x1, y1, x2, y2, (d - 1 - i) as f32 / df);
    //     let idx = (result * df) as usize;
    //     let mut str = String::from("");
    //     for j in 0..d {
    //         if idx == j {
    //             str += "*"
    //         } else {
    //             str += " ";
    //         }
    //     }
    //     println!("{:?}", str);
    // }
    

    
    for i in 0..100 {
        let t = i as f32 / 100.;
    let result = pi_curves::bezier::cubic_bezier(x1, y1, x2, y2, t);
    let result2 = simple_cubic_bezier(t, x1, y1, x2, y2);
    println!("{:?}", (result, result2));
    }
}