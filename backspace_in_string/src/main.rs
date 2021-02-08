#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test() {
        assert_eq!(clean_string("abc#d##c"), "ac");
        assert_eq!(clean_string("abc####d##c#"), "");
        assert_eq!(clean_string("#######"), "");
        assert_eq!(clean_string(""), "");
    }
}

// Assume "#" is like a backspace in string. This means that string "a#bc#d" actually is "bd"
//
// Your task is to process a string with "#" symbols.

fn clean_string(s: &str) -> String {
    // your code here
    String::new()
}

fn main() {
    println!("Hello, world!");
}
