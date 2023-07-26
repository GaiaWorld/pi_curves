use pi_curves;
extern crate nalgebra;

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

    
    let result = pi_curves::bezier::cubic_bezier(x1, y1, x2, y2, 1.);
    println!("{}", result);
}