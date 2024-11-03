use num_traits::ToPrimitive;

fn solve(a: f64, b: f64) -> f64 {
    (a.powi(2) + b.powi(2)).sqrt()
}

fn main() {
    // Can't take different type of numbers and
    // do arithmetic on them.
    let a: f32 = 3.0;
    let b: f32 = 4.0;

    let a_f64 = a as f64;
    let b_f64 = b.to_f64().unwrap(); // Works with 'use num_traits::ToPrimitive;'

    println!("{}", solve(a_f64, b_f64));
}
