//! Exercise 3: String, &str, and Vec — strings and vectors basics.
//! Run: cargo run --example 03_strings_vectors

fn main() {
    string_demo();
    vector_demo();
}

fn string_demo() {
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

    let capitalized = title.chars().next().map(|first| {
        first
            .to_uppercase()
            .chain(title.chars().skip(1))
            .collect::<String>()
    });
    println!("capitalized = {}", capitalized.unwrap_or_default());

    // Use character positions for a Unicode-safe substring.
    #[allow(clippy::iter_skip_zero)]
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
    println!(
        "splitn(2, ',') = {:?}",
        csv.splitn(2, ',').collect::<Vec<_>>()
    );
    println!("join('-') = {}", parts.join("-"));
    println!(
        "split_whitespace = {:?}",
        "  a  b  c  ".split_whitespace().collect::<Vec<_>>()
    );
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
    println!(
        "eq_ignore_ascii_case = {}",
        "RUST".eq_ignore_ascii_case("rust")
    );

    // Slicing (byte-index based; must land on char boundaries).
    println!("slice[0..4] = {}", &tstring[0..4]);

    // Split at a byte index (must land on a char boundary).
    let (head, tail) = tstring.split_at(4);
    println!("split_at(4) = {:?}, {:?}", head, tail);

    // Strip a prefix or suffix and return the remainder.
    println!(
        "strip_prefix('test ') = {:?}",
        tstring.strip_prefix("test ")
    );
    println!("strip_suffix('ing') = {:?}", tstring.strip_suffix("ing"));

    // Trim with a predicate instead of whitespace.
    let noisy = "...hello...";
    println!("trim_matches('.') = {:?}", noisy.trim_matches('.'));
    println!(
        "trim_start_matches('.') = {:?}",
        noisy.trim_start_matches('.')
    );
    println!("trim_end_matches('.') = {:?}", noisy.trim_end_matches('.'));

    // Split by a closure and by a terminator.
    let numbers = "10,20;30";
    let split_pred: Vec<&str> = numbers.split(|c| c == ',' || c == ';').collect();
    println!("split(predicate) = {:?}", split_pred);
    println!(
        "split_terminator(';') = {:?}",
        "a;b;c;".split_terminator(';').collect::<Vec<_>>()
    );
    println!(
        "rsplitn(2, ',') = {:?}",
        numbers.rsplitn(2, ',').collect::<Vec<_>>()
    );

    // Match a pattern and get positions.
    println!(
        "matches('t') = {:?}",
        tstring.matches('t').collect::<Vec<_>>()
    );
    println!(
        "match_indices('t') = {:?}",
        tstring.match_indices('t').collect::<Vec<_>>()
    );

    // Alignment/padding via format! (built into the standard library).
    println!("center 20 = {:^20}", "hi");
    println!("left_pad 10 = {:>10}", "hi");
    println!("right_pad 10 = {:<10}", "hi");

    // Escape and repr-style strings.
    println!(
        "escape_debug = {:?}",
        "\n\t".escape_debug().collect::<String>()
    );
    println!(
        "escape_default = {:?}",
        "\"hi\"".escape_default().collect::<String>()
    );
    println!(
        "escape_unicode = {:?}",
        "é".escape_unicode().collect::<String>()
    );

    // Comparisons and ordering.
    println!("eq = {}", "abc".eq("abc"));
    println!("cmp = {:?}", "abc".cmp("def"));
    println!("starts_with char = {}", "hello".starts_with('h'));

    // Mutable String operations.
    let mut owned = String::from("hello");
    owned.push('!');
    println!("push = {owned}");
    owned.pop();
    println!("pop = {owned}");
    owned.insert(0, 'H');
    println!("insert = {owned}");
    owned.insert_str(5, " world");
    println!("insert_str = {owned}");
    owned.remove(0);
    println!("remove = {owned}");
    owned.truncate(5);
    println!("truncate = {owned}");
    owned.clear();
    println!(
        "clear: len = {}, capacity = {}",
        owned.len(),
        owned.capacity()
    );
    owned.reserve(100);
    println!("reserve: capacity = {}", owned.capacity());
    owned.shrink_to_fit();
    println!("shrink_to_fit: capacity = {}", owned.capacity());

    let mut another = String::from("hello world");
    let tail = another.split_off(5);
    println!("split_off: {:?} + {:?}", another, tail);

    let cloned: String = tstring.to_owned();
    println!("to_owned = {cloned}");
    let boxed: Box<str> = cloned.into_boxed_str();
    println!("into_boxed_str len = {}", boxed.len());

    // Unicode-aware checks.
    println!("is_empty = {}", "".is_empty());
    println!("is_char_boundary(4) = {}", tstring.is_char_boundary(4));

    // Raw strings and byte strings.
    let raw = r"C:\Users\name\file.txt";
    println!("raw string = {raw}");
    let bytes: &[u8] = b"hello";
    println!("byte string = {:?}", bytes);
}

fn vector_demo() {
    // A Vec is a growable array on the heap. Create one with vec![].
    let mut numbers = vec![1, 2, 3];
    println!("numbers = {numbers:?}");
    println!("len = {}, capacity = {}", numbers.len(), numbers.capacity());

    // Push and pop add/remove from the end.
    numbers.push(4);
    numbers.push(5);
    println!("after push = {numbers:?}");
    let last = numbers.pop();
    println!("pop() = {last:?}, now = {numbers:?}");

    // Indexing (panics on out-of-bounds) vs. .get() (returns Option).
    println!("numbers[0] = {}", numbers[0]);
    println!("numbers.get(2) = {:?}", numbers.get(2));
    println!("numbers.get(99) = {:?}", numbers.get(99));

    // Insert and remove at arbitrary positions.
    numbers.insert(1, 99);
    println!("after insert(1, 99) = {numbers:?}");
    let removed = numbers.remove(1);
    println!("remove(1) = {removed}, now = {numbers:?}");

    // Vec can be created empty and extended.
    let mut v: Vec<i32> = Vec::new();
    v.extend([10, 20, 30]);
    println!("extend = {v:?}");

    // Resizing and pre-allocation.
    v.resize(5, 0);
    println!("resize(5, 0) = {v:?}");
    v.reserve(100);
    println!("after reserve, capacity = {}", v.capacity());
    v.shrink_to_fit();
    println!("after shrink_to_fit, capacity = {}", v.capacity());

    // Slicing a Vec (borrowing a portion of it).
    let nums = vec![1, 2, 3, 4, 5];
    let slice: &[i32] = &nums[1..4];
    println!("slice [1..4] = {slice:?}");

    // Iterating and transforming.
    let doubled: Vec<i32> = nums.iter().map(|n| n * 2).collect();
    println!("doubled = {doubled:?}");
    let evens: Vec<i32> = nums.into_iter().filter(|n| n % 2 == 0).collect();
    println!("evens = {evens:?}");

    // Vec of Strings.
    let mut words = vec![String::from("hello"), String::from("rust")];
    words.push(String::from("vec"));
    println!("words = {words:?}");

    // Common summary methods.
    #[allow(clippy::useless_vec)]
    let scores = vec![10, 20, 30, 40];
    println!("first = {:?}, last = {:?}", scores.first(), scores.last());
    println!("len = {}, is_empty = {}", scores.len(), scores.is_empty());
    println!("contains 20 = {}", scores.contains(&20));
}
