use std::ops::Add; //1. Bringing the add trait into scope.
use std::time::Duration; //2. Bringing the Duration type
fn add<T: Add<Output = T>>(i: T, j: T) -> T {
    //3. The arguments to add() can accept any type that implements std::ops::Add
    i + j
}
fn main() {
    let floats = add(1.2, 3.4); //4. Calling add() with floating point values
    let ints = add(10, 20); //5. Calling add() with integer values
    let durations = add(Duration::new(5, 0), Duration::new(10, 0)); //6. Call add() with Duration values, which represent a duration between two points in time.
    println!("{}", floats);
    println!("{}", ints);
    println!("{:?}", durations); //7.std::time::Duration does not implement the std::fmt::Display trait and so we can fall back to requesting std::fmt::Debug
}
