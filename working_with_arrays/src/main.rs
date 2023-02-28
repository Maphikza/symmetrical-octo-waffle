fn main() {
    let one = [1, 2, 3]; //1. [1, 2, 3] denotes an array literal and Rust infers its type itself
    let two: [u8; 3] = [1, 2, 3]; //2. [u8; 3] explicitly declares type of the array: 3 elements of u8
    let blank1 = [0; 3]; //3. This form is known as a repeat expression ([0; 3]) that expects a constant (0) to be repeated n times (3).
    let blank2: [u8; 3] = [0; 3]; //4. Type signatures are also supported for repeat expressions
    let arrays = [one, two, blank1, blank2];
    for a in &arrays { //5. Taking a reference to an array returns a slice. Slices support iteration through arrays without needing to call iter().
        print!("{:?}: ", a);
        for n in a.iter() { //6. Arrays also have methods for iteration and manipulation
            print!("\t{} + 10 = {}", n, n + 10);
        }
        let mut sum = 0;
        for i in 0..a.len() {
            sum += a[i]; //7. All array indexing is bounds checked. Requesting an item that’s out of bounds will crash the program. This is called a panic.
        }

        println!("\t(Σ{:?} = {})", a, sum);
    }
}
