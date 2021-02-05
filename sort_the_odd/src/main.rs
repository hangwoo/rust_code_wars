#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(sort_array(&[5, 3, 2, 8, 1, 4]), [1, 3, 2, 8, 5, 4]);
        assert_eq!(sort_array(&[5, 3, 1, 8, 0]), [1, 3, 5, 8, 0]);
        assert_eq!(sort_array(&[]), []);
    }

    #[test]
    fn basic2() {
        assert_eq!(sort_array2(&[5, 3, 2, 8, 1, 4]), [1, 3, 2, 8, 5, 4]);
        assert_eq!(sort_array2(&[5, 3, 1, 8, 0]), [1, 3, 5, 8, 0]);
        assert_eq!(sort_array2(&[]), []);
    }
}

fn sort_array(arr: &[i32]) -> Vec<i32> {
    let mut odds = arr.iter()
        .filter(|&num| num % 2 == 1)
        .map(|&num| num)
        .collect::<Vec<i32>>();
    odds.sort();
    odds.reverse();
    arr.iter()
        .enumerate()
        .map(|(_, &num)| {
            if num % 2 == 1 {
                return odds.pop().unwrap();
            }
            num
        })
        .collect::<Vec<i32>>()
}

fn sort_array2(arr: &[i32]) -> Vec<i32> {
    let mut copied = arr.iter().map(|&n| n).collect::<Vec<_>>();
    copied.sort();
    let mut odds =
        copied.iter()
        .filter(|&num| num % 2 == 1)
        .map(|&num| num);
    arr.iter()
        .enumerate()
        .map(|(_, &num)| {
            if num % 2 == 1 {
                return odds.next().unwrap();
            }
            num
        })
        .collect::<Vec<i32>>()
}

fn main() {
    println!("Hello, world!");
}
