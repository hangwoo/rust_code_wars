fn solution(word: &str, ending: &str) -> bool {
    word.chars().rev().take(ending.len()).collect::<String>() == ending
}

fn returns_expected() {
    assert_eq!(true, solution("abc", "c"));
    assert_eq!(false, solution("strawberry", "banana"));
}

fn main() {
    returns_expected();
}
