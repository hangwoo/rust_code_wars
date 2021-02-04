#[cfg(test)]
mod tests {
    use crate::solution;

    #[test]
    fn returns_expected() {
        assert_eq!(true, solution("abc", "c"));
        assert_eq!(false, solution("strawberry", "banana"));
        assert_eq!(false, solution("banana", "strawberry"));
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

fn main() {
}
