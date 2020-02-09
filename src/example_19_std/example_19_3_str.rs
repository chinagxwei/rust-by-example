fn strings() {
    let pangram = "the quick brown fox jumps over the lazy dog";
    println!("Pangram: {}", pangram);

    println!("Words in reverse");
    for word in pangram.split_whitespace().rev() {
        println!("> {}", word);
    }

    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();

    let mut string = String::new();
    for ch in chars {
        string.push(ch);

        string.push_str(", ");
    }

    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str = string.trim_matches(chars_to_trim);
    println!("Used characters: {}", trimmed_str);

    let alice = String::from("I like dogs");

    let bob = alice.replace("dog", "cat");

    println!("Alice says: {}", alice);
    println!("Bob says: {}", bob);
}

fn escapes() {
    /// 1.
    let byte_escape = "I'm writing \x52\x75\x73\x74";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE_STRUCK CAPITAL R\"";

    println!("Unicode character {} (U+211D) is called {}", unicode_codepoint, character_name);

    let long_string = "String literals can span multiple lines. The linebreak an indentation here ->\
    <- can be escaped too!";

    println!("{}", long_string);

    /// 2.

    let raw_str = r"Escapes don't work here: \x3f \u{211d}";
    println!("{}", raw_str);

    let quotes = r#"And the I said: "There is no escape!""#;
    println!("{}", quotes);

    let long_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", long_delimiter);

    /// 3.

    let bytestring = b"this is a bytestring";

    println!("A bytestring: {:?}", bytestring);

    let escaped = b"\x52\x75\x73\x74 as bytes";
    println!("Some escaped bytes: {:?}", escaped);

    let raw_bytestring = br"\u{211D} is not escaped here";
    println!("{:?}", raw_bytestring);

    if let Ok(my_str) = std::str::from_utf8(raw_bytestring) {
        println!("And the same as text: '{}'", my_str);
    }

    let quotes = br#"You can also use "fancier" formatting, \
    like with normal raw strings"#;

    let shift_jis = b"\x82\xe6\x82\xa8\x82\xb1\x82";

    match std::str::from_utf8(shift_jis) {
        Ok(my_str) => println!("Conversion successful: {}", my_str),
        Err(e) => println!("Conversion failed: {:?}", e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_19_3_str() {
        strings();
        escapes();
    }
}