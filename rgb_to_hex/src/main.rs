#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(rgb2(0, 0, 0), "000000");
        assert_eq!(rgb2(1, 2, 3), "010203");
        assert_eq!(rgb2(255, 255, 255), "FFFFFF");
        assert_eq!(rgb2(254, 253, 252), "FEFDFC");
        assert_eq!(rgb2(-20, 275, 125), "00FF7D");
        assert_eq!(rgb2(0, 0, 0), "000000");
        assert_eq!(rgb2(1, 2, 3), "010203");
        assert_eq!(rgb2(255, 255, 255), "FFFFFF");
        assert_eq!(rgb2(254, 253, 252), "FEFDFC");
        assert_eq!(rgb2(-20, 275, 125), "00FF7D");
    }
}

fn to_hex(r: i32) -> String {
    if r < 0 {
        return String::from("00");
    } else if r > 255 {
        return String::from("FF");
    }

    let s = format!("{:X}", r);

    return if s.len() < 2 {
        format!("0{}", s)
    } else {
        s
    }
}

fn to_hex2(r: i32) -> String {
    let s = format!("{:X}", r.max(0).min(255));

    return if s.len() < 2 {
        format!("0{}", s)
    } else {
        s
    }
}

fn rgb(r: i32, g: i32, b: i32) -> String {
    format!("{}{}{}", to_hex(r), to_hex(g), to_hex(b))
}

fn rgb2(r: i32, g: i32, b: i32) -> String {
    format!("{}{}{}", to_hex2(r), to_hex2(g), to_hex2(b))
}

fn main() {
    println!("Hello, world!");
}
