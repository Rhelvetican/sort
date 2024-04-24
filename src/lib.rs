pub mod exchange;
pub mod hybrid;
pub mod merge;
pub mod select;

#[cfg(test)]
mod test {

    use crate::*;

    fn ge<T: PartialOrd>(a: &T, b: &T) -> bool {
        a >= b
    }

    fn le<T: PartialOrd>(a: &T, b: &T) -> bool {
        a <= b
    }

    #[test]
    fn test_bubble_sort() {
        let mut input = vec![4, 2, 3, 1];
        exchange::bubble::bubble_sort(&mut input);
        assert_eq!(input, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_bubble_sort_cmp() {
        let mut input = vec![4, 2, 3, 1];
        exchange::bubble::bubble_sort_cmp(&mut input, ge);
        assert_eq!(input, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_comb_sort() {
        let mut input = vec![4, 2, 3, 1];
        exchange::comb::comb_sort(&mut input);
        assert_eq!(input, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_comb_sort_cmp() {
        let mut input = vec![4, 2, 3, 1];
        exchange::comb::comb_sort_cmp(&mut input, ge);
        assert_eq!(input, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_cocktail_sort() {
        let mut input = vec![4, 2, 3, 1];
        exchange::cocktail::cocktail_sort(&mut input);
        assert_eq!(input, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_cocktail_sort_cmp() {
        let mut input = vec![4, 2, 3, 1];
        exchange::cocktail::cocktail_sort_cmp(&mut input, ge);
        assert_eq!(input, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_gnome_sort() {
        let mut input = vec![4, 2, 3, 1];
        exchange::gnome::gnome_sort(&mut input);
        assert_eq!(input, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_gnome_sort_cmp() {
        let mut input = vec![4, 2, 3, 1];
        exchange::gnome::gnome_sort_cmp(&mut input, le);
        assert_eq!(input, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_odd_even_sort() {
        let mut input = vec![4, 2, 3, 1];
        exchange::odd_even::odd_even_sort(&mut input);
        assert_eq!(input, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_odd_even_sort_cmp() {
        let mut input = vec![4, 2, 3, 1];
        exchange::odd_even::odd_even_sort_cmp(&mut input, ge);
        assert_eq!(input, vec![1, 2, 3, 4]);
    }
}
