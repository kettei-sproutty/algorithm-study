pub fn linear_search(arr: Vec<&str>, search: &str) -> bool {
    for element in arr {
        if element.eq(search) {
            return true;
        }
    }

    return false;
}

pub fn binary_search(arr: Vec<&str>, search: &str) -> bool {
    let mut maximum: usize = arr.len() - 1;
    let mut minimum: usize = 0;

    let calculate_index = |max: usize, min: usize| {
        return min + (max - min) / 2;
    };

    let mut index = calculate_index(maximum, minimum);

    let element = search.parse::<usize>().unwrap();
    loop {
        if arr.get(index).is_none() {
            return false;
        }

        if maximum.lt(&minimum) {
            return false;
        }

        let item = arr.get(index).unwrap();

        if item.eq(&search) {
            return true;
        }

        if minimum.eq(&maximum) || (minimum + 1).eq(&maximum) {
            return false;
        }

        if item.parse::<usize>().unwrap().gt(&element) {
            maximum = index + 1;
            index = calculate_index(maximum, minimum) + 1;
        } else {
            minimum = index;
            index = calculate_index(maximum, minimum);
        }
    }
}

fn main() {
    println!("Run `cargo test` for running all algorithms tests");
}

#[cfg(test)]
pub mod test {
    mod linear {
        use crate::search::linear_search;

        #[test]
        fn should_be_contained() {
            let input: Vec<&str> = vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "l"];
            let is_contained = linear_search(input, "f");
            assert_eq!(is_contained, true)
        }

        #[test]
        fn should_not_be_contained() {
            let input: Vec<&str> = vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "l"];
            let is_contained = linear_search(input, "z");
            assert_eq!(is_contained, false);
        }
    }

    mod binary {
        use crate::search::binary_search;

        #[test]
        fn binary_search_contained() {
            let input: Vec<&str> = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"];
            let is_contained = binary_search(input, "5");
            assert_eq!(is_contained, true);
        }

        #[test]
        fn binary_search_not_contained() {
            let input: Vec<&str> = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"];
            let is_contained = binary_search(input, "15");
            assert_eq!(is_contained, false);
        }
    }
}
