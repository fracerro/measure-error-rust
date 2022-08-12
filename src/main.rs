mod measures;
use measures::Measure;

fn main() {
    let m1 = Measure::<f32> {
        value: 10.58,
        error: 0.018
    };
    let m2 = Measure::<f32> {
        value: 6.00,
        error: 0.300
    };

    //println!("{}", m1 * m2);
    println!("{}", m1 * std::f32::consts::PI);
    println!("{}", "(1.0±2.2)".parse::<Measure<f32>>().unwrap());
}
