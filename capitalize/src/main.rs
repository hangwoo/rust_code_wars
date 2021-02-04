#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test() {
        assert_eq!(capitalize("abcdef"),["AbCdEf", "aBcDeF"]);
        assert_eq!(capitalize("codewars"),["CoDeWaRs", "cOdEwArS"]);
        assert_eq!(capitalize("abracadabra"),["AbRaCaDaBrA", "aBrAcAdAbRa"]);
        assert_eq!(capitalize("codewarriors"),["CoDeWaRrIoRs", "cOdEwArRiOrS"]);
    }

    #[test]
    fn test2() {
        assert_eq!(capitalize2("abcdef"),["AbCdEf", "aBcDeF"]);
        assert_eq!(capitalize2("codewars"),["CoDeWaRs", "cOdEwArS"]);
        assert_eq!(capitalize2("abracadabra"),["AbRaCaDaBrA", "aBrAcAdAbRa"]);
        assert_eq!(capitalize2("codewarriors"),["CoDeWaRrIoRs", "cOdEwArRiOrS"]);
    }
}

fn capitalize(s: &str) -> Vec<String> {
    vec![s.chars().enumerate().map(|(i, c)| {
        if i % 2 == 0 {
            return c.to_uppercase().to_string();
        }
        return c.to_string();
    }).collect(),
         s.chars().enumerate().map(|(i, c)| {
             if i % 2 == 1 {
                 return c.to_uppercase().to_string();
             }
             return c.to_string();
         }).collect()
    ]
}

fn capitalize2(s: &str) -> Vec<String> {
    vec![s.chars().enumerate().map(|(i, c)| {
        if i % 2 == 0 {
            return c.to_ascii_uppercase();
        }
        return c;
    }).collect(),
         s.chars().enumerate().map(|(i, c)| {
             if i % 2 == 1 {
                return c.to_ascii_uppercase();
             }
             return c;
         }).collect()
    ]
}


fn main() {
    println!("Hello, world!");
}
