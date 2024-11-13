use num_traits::ToPrimitive;

/*
    ### 'solve' function ###
        - First version - We can pass in both f32 & f64
        via 'Float' trait
        - Second version - We can pass in any type of number
        via 'ToPrimitive' trait

    ### Trait bounds ###
        - 'Float' is a 'trait'
        - 'Float' is being used as a 'trait bound'

        ### What is a trait? ###
        - A trait is a set of methods
        - It can contain abstract methods which don't have
        implementation
        - It can contain default methods which have
        implementation

        - A struct/enum/primitive can implement a trait
        - The implementor has to provide an implementation
        for all of the abstract methods
        - The implementor can optionally override the
        default methods

        - Behind the scenes, f32 and f64 both implement the
        'Float' trait

    ### Super Solve Flexibility ###
        - Use 'ToPrimitive' trait instead of 'Float' trait
        in 'solve' func
*/

trait Vehicle {
    // Abstract method
    fn start(&self);

    // Default method
    fn stop(&self) {
        println!("Stopped");
    }
}

struct Car {}

impl Vehicle for Car {
    fn start(&self) {
        println!("Started");
    }
}

fn start_and_stop<T: Vehicle>(vehicle: T) {
    vehicle.start();
    vehicle.stop();
}

// If we use 'T: ToPrimitive' and a: T, b: T and pass in an
// integer and a float as arguments we get an error. This is
// so because a: T, b: T implies that 'a' and 'b' are both of
// similar type
fn solve<T: ToPrimitive, U: ToPrimitive>(a: T, b: U) -> f64 {
    // Works with 'use num_traits::ToPrimitive;'
    let a_f64 = a.to_f64().unwrap(); // !Error: 'a as f64'

    // Works with 'use num_traits::ToPrimitive;'
    let b_f64 = b.to_f64().unwrap();

    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}

fn main() {
    // Can't take different type of numbers and do
    // arithmetic on them
    let a: f32 = 3.0;
    let b: f64 = 4.0;

    println!("{}", solve::<f32, f64>(a, b));

    // Here, we are relying upon inference
    println!("{}", solve(6.0, 8.0));

    println!();

    let car = Car {};
    start_and_stop(car);

    println!();

    // Super solve flexibility (with 'ToPrimitive')
    // This won't work with 'Float' trait
    println!("{}", solve(30, 40.0));
}
