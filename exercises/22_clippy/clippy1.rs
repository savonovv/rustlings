// The Clippy tool is a collection of lints to analyze your code so you can
// catch common mistakes and improve your Rust code.
//
// For these exercises, the code will fail to compile when there are Clippy
// warnings. Check Clippy's suggestions from the output to solve the exercise.

fn main() {
    // TODO: Fix the Clippy lint in this line.
    let pi = 3.14;
    let radius: f64 = 5.0;

    let area = pi * radius.powi(2);

    println!("The area of a circle with radius {radius:.2} is {area}"); // f64 to f64, no format needed
}
