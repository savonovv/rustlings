#[macro_export]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

mod macros {
    fn call_macro() {
        crate::my_macro!();
    }
}

fn main() {
    my_macro!();
}
