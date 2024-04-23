pub mod bubble;
pub mod cocktail;
pub mod comb;
pub mod gnome;
pub mod odd_even;

pub fn exchange_sort<T: PartialOrd>(input: &mut [T]) {
    let l = input.len() - 1;
    for j in 0..l {
        for i in 0..(l - j) {
            if input[i] > input[i + 1] {
                input.swap(i, i + 1);
            }
        }
    }
}
pub fn exchange_sort_cmp<T, F>(input: &mut [T], cmp: F)
where
    F: Fn(&T, &T) -> bool,
{
    let l = input.len() - 1;
    for j in 0..l {
        for i in 0..(l - j) {
            if cmp(&input[i], &input[i + 1]) {
                input.swap(i, i + 1);
            }
        }
    }
}
