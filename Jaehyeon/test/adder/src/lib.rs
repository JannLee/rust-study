#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        let result = 2 + 2;

        assert!(false, "이렇게 하는게 맞나?");
    }
}
