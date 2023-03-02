use std::thread;

fn main() {
    spawn_option_one();
    spawn_option_two();
}

fn f() {
    println!("This is another thread!");

    let id = thread::current().id();
    println!("My thread with id: {id:?}");
}

fn spawn_option_one() {
    let thread1 = thread::spawn(f);
    let thread2 = thread::spawn(f);

    println!("This is the main thread.");

    thread1.join().unwrap();
    thread2.join().unwrap();
}

// using closure.
fn spawn_option_two() {
    let numbers = vec![1, 2, 3];

    thread::spawn(move || {
        let mut sum_numbers = 0;
        for n in numbers {
            println!("{n}");
            sum_numbers += n
        }
        println!("Sum of numbers in Vec is: {}", sum_numbers);
    }).join().unwrap();
}
