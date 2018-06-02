//Test-drive a method or function to accept a list of integers and return the one that is closest to zero.
//It should be an error for the list to be empty.
//If two different numbers tie for distance from zero (for example, 2 and -2), always return the positive one.

use std::option::Option::None;
use std::option::Option::Some;

#[allow(dead_code)]
fn closest_to_zero_mins<'a>(numbers: &'a Vec<i32>) -> Option<&'a i32> {
    let least_positive: Option<&'a i32> = numbers.iter().filter(|x| **x >= 0).min();
    let least_negative: Option<&'a i32> = numbers.iter().filter(|x| **x < 0).max();
    match (least_positive, least_negative) {
        (Some(_p), Some(_n)) => if _p < &_n.abs() { least_positive } else { least_negative },
        (Some(_p), None) => least_positive,
        (None, Some(_n)) => least_negative,
        (None, None) => None,
    }
}

pub fn closest_to_zero(numbers: &Vec<i32>) -> Option<&i32> {
    numbers.iter().fold(None, |acc, val| {
        if acc.is_none() { Some(val) } else {
            acc.map(|closest| {
                if val.abs() == closest.abs() && *val > 0 { val } else {
                    if val < closest && val.abs() < closest.abs() { val } else { closest }
                }
            })
        }
    })
}

#[cfg(test)]
mod tests {
    use zero_kata::closest_to_zero;

    #[test]
    fn closest_to_zero_simple() {
        let numbers = vec![0];

        assert_eq!(*closest_to_zero(&numbers).unwrap(), 0)
    }

    #[test]
    fn must_choose_lowest_positive_integer() {
        let numbers = vec![2, 1, 3];

        assert_eq!(1, *closest_to_zero(&numbers).unwrap())
    }

    #[test]
    fn must_choose_closer_negative() {
        let numbers = vec![2, -1, -3];

        assert_eq!(*closest_to_zero(&numbers).unwrap(), -1)
    }

    #[test]
    fn same_magnitude_positive_must_be_chosen() {
        let numbers = vec![2, -1, 1];

        assert_eq!(*closest_to_zero(&numbers).unwrap(), 1)
    }
}

