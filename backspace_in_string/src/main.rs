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

    #[test]
    fn test2() {
        assert_eq!(clean_string2("abc#d##c"), "ac");
        assert_eq!(clean_string2("abc####d##c#"), "");
        assert_eq!(clean_string2("#######"), "");
        assert_eq!(clean_string2(""), "");
    }
}

// Assume "#" is like a backspace in string. This means that string "a#bc#d" actually is "bd"
//
// Your task is to process a string with "#" symbols.

fn clean_string(s: &str) -> String {
    let mut chars: Vec<char> = vec![];
    // your code here
    for c in s.chars() {
        match c {
            '#' => {
                chars.pop();
            },
            _ => {
                chars.push(c);
            }
        }
    }
    chars.iter().collect::<String>()
}

fn clean_string2(s: &str) -> String {
    let mut result = String::new();
    s.chars().for_each(|x| {
        match x {
            '#' => {
                result.pop();
            },
            _ => {
                result.push(x);
            }
        }
    });
    result
}

fn main() {
    println!("Hello, world!");
}
