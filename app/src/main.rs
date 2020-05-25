mod variables;
mod strings;
mod tuples;
mod arrays;
mod vectors;
mod conditions;
mod loops;

fn main() {
    println!("Hello, world!");
    variables::run();
    strings::run();
    tuples::run();
    arrays::run();
    vectors::run();
    conditions::run();
    loops::run();
}
