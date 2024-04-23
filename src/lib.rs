pub mod exchange;
pub mod hybrid;
pub mod merge;
pub mod select;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut input = vec![4, 2, 3, 1];
        exchange::bubble::bubble_sort(&mut input);
        assert_eq!(input, vec![1, 2, 3, 4]);
    }
}
