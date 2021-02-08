#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn basic_tests() {
        assert!(!is_prime(0), "0 is not prime");
        assert!(!is_prime(1), "1 is not prime");
        assert!(is_prime(2), "2 is prime");
        assert!(is_prime(73), "73 is prime");
        assert!(!is_prime(75), "75 is not prime");
        assert!(!is_prime(-1), "-1 is not prime");
    }

    #[test]
    fn prime_tests() {
        assert!(is_prime(3), "3 is prime");
        assert!(is_prime(5), "5 is prime");
        assert!(is_prime(7), "7 is prime");
        assert!(is_prime(41), "41 is prime");
        assert!(is_prime(5099), "5099 is prime");
    }

    #[test]
    fn not_prime_tests() {
        assert!(!is_prime(4), "4 is not prime");
        assert!(!is_prime(6), "6 is not prime");
        assert!(!is_prime(8), "8 is not prime");
        assert!(!is_prime(9), "9 is not prime");
        assert!(!is_prime(45), "45 is not prime");
        assert!(!is_prime(-5), "-5 is not prime");
        assert!(!is_prime(-8), "-8 is not prime");
        assert!(!is_prime(-41), "-41 is not prime");
    }

    #[test]
    fn not_prime_tests2() {
        assert!(!is_prime2(4), "4 is not prime");
        assert!(!is_prime2(6), "6 is not prime");
        assert!(!is_prime2(8), "8 is not prime");
        assert!(!is_prime2(9), "9 is not prime");
        assert!(!is_prime2(45), "45 is not prime");
        assert!(!is_prime2(-5), "-5 is not prime");
        assert!(!is_prime2(-8), "-8 is not prime");
        assert!(!is_prime2(-41), "-41 is not prime");
    }
}

fn is_prime(number: i32) -> bool {
    return if number <= 1 {
        false
    } else if number % 2 == 0 {
        return number == 2
    } else {
        let mut i = 3;
        while i as f64 <= (number as f64).sqrt() {
            if number % i == 0 {
                return false;
            }
            i += 2;
        }
        true
    }
}

fn is_prime2(number: i64) -> bool {
    if number == 2 {
        return true;
    }
    if number < 2 || number % 2 == 0 {
        return false;
    }
    (3..).step_by(2)// 2 step 씩 iterator
        .take_while(|i| i * i <= number) // sqrt 가 number 보다 작은 것들만 take
        .all(|i| number % i != 0)// 그 숫자로 모두 나눴을때 0 이 안돼면 true
}

fn main() {
}
