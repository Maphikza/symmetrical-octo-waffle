fn main() {
    let context_lines = 2;
    let needle = "oo";
    let haystack = "\
Every face, every shop,
bedroom window, public-house, and
dark square is a picture
feverishly turned--in search of what?
It is the same with books.
What do we seek
through millions of pages?";
    let mut tags: Vec<usize> = Vec::new(); //1. tags holds line numbers where matches occur
    let mut ctx: Vec<Vec<(usize, String)>> = Vec::new(); //2. ctx contains a vector per match to hold that match’s context lines
    for (i, line) in haystack.lines().enumerate() { //3.Iterate through the lines, recording line numbers where matches are encountered
        if line.contains(needle) {
            tags.push(i);
            println!("i is:{}", i);
            println!("tags is {:?}", tags);
            let v = Vec::with_capacity(2 * context_lines + 1); //4-5. Vec::with_capacity(n) reserves space for n items - No explicit type signature is required here, as it can be inferred via the definition of ctx on line 15.
            println!("{:?}", v.capacity());
            println!("ctx before: {:?}", ctx);
            ctx.push(v);
            println!("added capacity for ctx vec is: {:?}", ctx.capacity());
        }
    }
    println!("ctx after: {:?}", ctx);
    if tags.len() == 0 { //6.When there are no matches, exit early
        return;
    }
    for (i, line) in haystack.lines().enumerate() { //7.For each tag, at every line, check to see if we are nearby a match. When we are, add that line to the relevant Vec<T> within ctx.
        for (j, tag) in tags.iter().enumerate() {
            let lower_bound = tag.saturating_sub(context_lines); //8. usize.saturating_sub() is subtraction that returns 0 on integer underflow rather than crashing the program (CPUs don’t like attempting to send usize below zero)
            // println!("lower bound {}", lower_bound.to_string());
            let upper_bound = tag + context_lines;
            // println!("upper bound {}", upper_bound.to_string());
            if (i >= lower_bound) && (i <= upper_bound) {
                let line_as_string = String::from(line); //9. Copy line into a new String and store that locally for each match
                let local_ctx = (i, line_as_string);
                println!("local ctx: {:?}", local_ctx);
                ctx[j].push(local_ctx);
                println!("ctx[j]: {:?}", ctx[j]);
            }
        }
    }
    println!("ctx at the end: {:?}", ctx);
    //This line unpacks all the items added to ctx and then iterates on them to print the line numbers in the expected form line by line.
    for local_ctx in ctx.iter() {
        for &(i, ref line) in local_ctx.iter() { //10. ref line informs the compiler that we wish to borrow this value, rather than move it. These two terms are explained fully later in later chapters.
            let line_num = i + 1;
            println!("{}: {}", line_num, line);
        }
    }
}
