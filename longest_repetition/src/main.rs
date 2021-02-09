use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn longest_at_the_beginning() {
        assert_eq!(longest_repetition(&"aaaabbb"), Some(('a', 4)));
    }

    #[test]
    fn longest_at_the_end() {
        assert_eq!(longest_repetition(&"abbbbb"), Some(('b', 5)));
        assert_eq!(longest_repetition(&"bbbaaabaaaa"), Some(('a', 4)));
    }

    #[test]
    fn longest_in_the_middle() {
        assert_eq!(longest_repetition(&"cbdeuuu900"), Some(('u', 3)));
    }

    #[test]
    fn multiple_longest() {
        assert_eq!(longest_repetition(&"aabb"), Some(('a', 2)));
        assert_eq!(longest_repetition(&"ba"), Some(('b', 1)));
    }

    #[test]
    fn single_character_only() {
        assert_eq!(longest_repetition(&"aaaaaa"), Some(('a', 6)));
    }

    #[test]
    fn empty_string() {
        assert_eq!(longest_repetition(&""), None);
    }
}

fn longest_repetition(s: &str) -> Option<(char, usize)> {
    let mut hash: HashMap<char, usize> = HashMap::new();
    if s.len() == 0 {
        return None;
    }
    s.chars().for_each(|x| {
        match hash.get_mut(&x) {
            Some(v) => {
                *v = *v + 1;
            },
            None => {
                hash.insert(x, 1);
            }
        }
    });
    let mut v = hash.iter()
        .map(|(&c, &v)| (c, v))
        .collect::<Vec<_>>() ;
    v.sort_by(|(_, size1), (_, size2)| size1.partial_cmp(size2).unwrap());
    v.pop()
}

fn main() {
    println!("Hello, world!");
}
