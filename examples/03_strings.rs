//! Exercise 3: String and &str basics plus common string methods.
//! Run: cargo run --example 03_strings

fn main() {
    // A string literal is a borrowed string slice with a fixed size.
    let greeting: &str = "Hello";
    println!("greeting (&str) = {greeting}");

    // String owns its text and can grow or change at runtime.
    let mut message = String::from(greeting);
    message.push_str(", Rust!");
    println!("message (String) = {message}");

    // Borrowing a String as &str does not transfer ownership.
    let borrowed: &str = &message;
    println!("borrowed (&str) = {borrowed}");

    // Common string operations.
    let title = "Rust makes strings interesting";
    println!("uppercase = {}", title.to_uppercase());
    println!("lowercase = {}", title.to_lowercase());

    let capitalized = title
        .chars()
        .next()
        .map(|first| first.to_uppercase().chain(title.chars().skip(1)).collect::<String>());
    println!("capitalized = {}", capitalized.unwrap_or_default());

    // Use character positions for a Unicode-safe substring.
    let substring: String = title.chars().skip(0).take(4).collect();
    println!("substring = {substring}");

    let reversed: String = title.chars().rev().collect();
    println!("reversed = {reversed}");

    let tstring: &str = "test String";
    println!("tstring = {} characters long", tstring.len());
    println!("tstring is empty = {}", tstring.is_empty());

    // Trimming whitespace.
    let padded = "  padded  ";
    println!("trim = {:?}", padded.trim());
    println!("trim_start = {:?}", padded.trim_start());
    println!("trim_end = {:?}", padded.trim_end());

    // Searching.
    println!("contains 'Str' = {}", tstring.contains("Str"));
    println!("starts_with 'test' = {}", tstring.starts_with("test"));
    println!("ends_with 'ing' = {}", tstring.ends_with("ing"));
    println!("find 'Str' = {:?}", tstring.find("Str"));
    println!("rfind 't' = {:?}", tstring.rfind('t'));

    // Splitting and joining.
    let csv = "a,b,c";
    let parts: Vec<&str> = csv.split(',').collect();
    println!("split(',') = {:?}", parts);
    println!("splitn(2, ',') = {:?}", csv.splitn(2, ',').collect::<Vec<_>>());
    println!("join('-') = {}", parts.join("-"));
    println!("split_whitespace = {:?}", "  a  b  c  ".split_whitespace().collect::<Vec<_>>());
    let mut lines_text = "line1\nline2".lines();
    println!("lines() first = {:?}", lines_text.next());

    // Replacing.
    println!("replace = {}", tstring.replace("test", "best"));
    println!("replacen = {}", "aaa".replacen('a', "b", 2));

    // Repeating and concatenating.
    println!("repeat(3) = {}", "ab".repeat(3));
    println!("concat (+) = {}", String::from("foo") + "bar");
    println!("format! = {}", format!("{}-{}", "foo", "bar"));

    // Parsing and conversion.
    let parsed: i32 = "42".parse().unwrap();
    println!("parse::<i32> = {parsed}");
    println!("to_string = {}", 42.to_string());
    println!("as_bytes[0] = {}", tstring.as_bytes()[0]);
    println!("bytes().count() = {}", tstring.bytes().count());

    // Case checks and char methods.
    println!("is_ascii = {}", tstring.is_ascii());
    println!("eq_ignore_ascii_case = {}", "RUST".eq_ignore_ascii_case("rust"));

    // Slicing (byte-index based; must land on char boundaries).
    println!("slice[0..4] = {}", &tstring[0..4]);
}
