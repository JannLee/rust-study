use std::collections::HashMap;

pub fn get_average(list: &[i32]) -> f32 {
    let mut vec = list.to_vec();
    for i in list {
        vec.push(*i);
    }
    let sum = vec.iter().sum::<i32>() as f32;
    let len = vec.len() as f32;
    sum / len
}

pub fn get_median(list: &[i32]) -> i32 {
    let mut vec = list.to_vec();
    vec.sort();
    if vec.len() % 2 == 0 {
        let first = vec.len() / 2 - 1;
        let second = vec.len() / 2;
        (vec[first] + vec[second]) / 2
    } else {
        vec[vec.len() / 2]
    }
}

pub fn get_mode(list: &[i32]) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for i in list {
        let count = map.entry(*i).or_insert(0);
        *count += 1;
    }
    let mut max_count = 0;
    let mut max_value = 0;
    for i in list {
        let count = map.entry(*i).or_default();
        if max_count < *count {
            max_count = *count;
            max_value = *i;
        }
    }
    max_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_average_test() {
        let list = [200, 300, 100];
        let expected = 200.0;
        let result = get_average(&list);
        assert_eq!(result, expected);
    }

    #[test]
    fn get_median_test() {
        let list = [1, 2, 100];
        let expected = 2;
        let result = get_median(&list);
        assert_eq!(result, expected);
    }

    #[test]
    fn get_median_when_even_length_list_test() {
        let list = [1, 10, 90, 200];
        let expected = 50;
        let result = get_median(&list);
        assert_eq!(result, expected);
    }

    #[test]
    fn get_mode_test() {
        let list = [1, 1, 3, 4];
        let expected = 1;
        let result = get_mode(&list);
        assert_eq!(result, expected);
    }
    
}