fn main() {
    let search_term = "million";
    let quote = "\
Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?"; //1. Multi-lined strings do not require special syntax. The slash (\) character on line 4 escapes the new line

    for (i, line) in quote.lines().enumerate() {
        //2. The lines() method returns an iterator over quote. Each iteration will be a line of text.
        if line.contains(search_term) {
            let line_num = i + 1;
            println!("{}: {}", line_num, line);
        }
    }
}
