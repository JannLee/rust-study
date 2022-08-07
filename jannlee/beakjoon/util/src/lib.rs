#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub mod cli {
    pub mod input {
        use std::io::stdin;

        /// Get 32bit integers from input string.
        ///
        /// # Panics
        ///
        /// Panics if .
        pub fn get_i32s() -> Vec<i32> {
            let mut input = String::new();
    
            stdin().read_line(& mut input).unwrap();
            input.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        }
    }
}
