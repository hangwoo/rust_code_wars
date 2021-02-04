#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(solution("abcdef"), ["ab", "cd", "ef"]);
        assert_eq!(solution("abcdefg"), ["ab", "cd", "ef", "g_"]);
        assert_eq!(solution(""), [] as [&str; 0]);
    }
}

fn solution(s: &str) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    let str = if s.len() % 2 != 0 {
        format!("{}_", s)
    } else {
        String::from(s)
    };
    let mut odd = str
        .chars()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, c)| c).collect::<String>();
    let mut even = str
        .chars()
        .enumerate()
        .filter(|(i, _)| i % 2 == 1)
        .map(|(_, c)| c).collect::<String>();
    loop {
        if let Some(o) = odd.pop() {
            if let Some(e) = even.pop() {
                result.push(format!("{}{}", o, e));
            }
        } else {
            break;
        }
    }
    result.reverse();
    result
}

fn main() {
    println!("Hello, world!");
}
