use pi_curves;
extern crate nalgebra;

#[test]
fn test() {

    env_logger::init();

    let result = pi_curves::bezier::cubic_bezier(0.0f32, 0.1f32, 1.0f32, 0.0f32, 0.5f32);
    log::info!("bezier_curve {:?}", result);
}