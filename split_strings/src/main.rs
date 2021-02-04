#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(solution("abcdef"), ["ab", "cd", "ef"]);
        assert_eq!(solution("abcdefg"), ["ab", "cd", "ef", "g_"]);
        assert_eq!(solution(""), [] as [&str; 0]);
    }

    #[test]
    fn basic2() {
        assert_eq!(solution2("abcdef"), ["ab", "cd", "ef"]);
        assert_eq!(solution2("abcdefg"), ["ab", "cd", "ef", "g_"]);
        assert_eq!(solution2(""), [] as [&str; 0]);
    }

    #[test]
    fn basic3() {
        assert_eq!(solution3("abcdef"), ["ab", "cd", "ef"]);
        assert_eq!(solution3("abcdefg"), ["ab", "cd", "ef", "g_"]);
        assert_eq!(solution3(""), [] as [&str; 0]);
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

fn solution2(s: &str) -> Vec<String> {
    let str = if s.len() % 2 != 0 {
        format!("{}_", s)
    } else {
        String::from(s)
    };
    str.chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>()
}

fn solution3(s: &str) -> Vec<String> {
    let str = if s.len() % 2 != 0 {
        format!("{}_", s)
    } else {
        String::from(s)
    };
    let odd = str
        .chars()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, c)| c);
    let even = str
        .chars()
        .enumerate()
        .filter(|(i, _)| i % 2 == 1)
        .map(|(_, c)| c);
    odd.zip(even).map(|(c1, c2)| [c1, c2].iter().collect::<String>()).collect::<Vec<String>>()
}

fn main() {
    println!("Hello, world!");
}
