fn main() {
    reference();
    iterating();
    into_iter_option();
}

// &reference for iterating.

fn reference() {
    println!("This is for the reference version.");
    let needle = 0o52;
    let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];
    for item in &haystack {
        if *item == needle {
            println!("{}", item);
        }
    }
}

// In some cases there are types that will not allow the reference method. In those cases you can use .iter().

fn iterating() {
    println!("This is for the .iter() version.");
    let needle = 0o52;
    let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];
    for item in haystack.iter() {
        if *item == needle {
            println!("{}", item);
        }
    }
}

// This option will allow you to not use *item or deference. into_iter() returns the actual value and thus makes it unnecessary to dereference.

fn into_iter_option() {
    println!("This is for the .into_iter() version.");
    let needle = 0o204;
    let haystack = vec![1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862, 16796];
    for item in haystack.into_iter() {
        if item == needle {
            println!("{}", item);
        }
    }
}
