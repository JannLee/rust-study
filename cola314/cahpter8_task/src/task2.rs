
pub fn to_pig_latin(str: &str) -> String {
    let vowel = "aeiou";
    for c in vowel.chars() {
        if c == str.chars().nth(0).unwrap() {
            return str.to_string() + "-hay"
        }
    }
    str[1..].to_string() + "-" + &str[..1] + "ay"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_pig_latin_starts_with_consonant_test() {
        let expected = "irst-fay";
        let result = to_pig_latin("first");
        assert_eq!(result, expected);
    }

    #[test]
    fn to_pig_latin_starts_with_vowel_test() {
        let expected = "apple-hay";
        let result = to_pig_latin("apple");
        assert_eq!(result, expected);
    }   
}