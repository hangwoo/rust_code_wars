#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn returns_expected() {
        assert_eq!(true, solution("abc", "c"));
        assert_eq!(false, solution("strawberry", "banana"));
        assert_eq!(false, solution("banana", "strawberry"));
    }

    #[test]
    fn returns_expected2() {
        assert_eq!(true, solution2("abc", "c"));
        assert_eq!(false, solution2("strawberry", "banana"));
        assert_eq!(false, solution2("banana", "strawberry"));
    }

    #[test]
    fn returns_expected3() {
        assert_eq!(true, solution3("abc", "c"));
        assert_eq!(false, solution3("strawberry", "banana"));
        assert_eq!(false, solution3("banana", "strawberry"));
    }
}

pub fn solution(word: &str, ending: &str) -> bool {
    let start: usize = if word.len() >= ending.len() {
        word.len() - ending.len()
    } else {
        0
    };

    if start > 0 {
        return &word[start..] == ending;
    } else if start == 0 {
        return word == ending;
    }

    false
}

pub fn solution2(word: &str, ending: &str) -> bool {
    if word.len() < ending.len() {
        return false;
    }

    &word[word.len() - ending.len() ..] == ending
}

pub fn solution3(word: &str, ending: &str) -> bool {
    word.ends_with(ending)
}

fn main() {
}
