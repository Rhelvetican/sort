pub fn gnome_sort<T: PartialOrd>(input: &mut [T]) {
    let mut i = 0;
    while i < input.len() {
        if i == 0 || input[i - 1] <= input[i] {
            i += 1;
        } else {
            input.swap(i - 1, i);
            i -= 1;
        }
    }
}

pub fn gnome_sort_cmp<T, F>(input: &mut [T], cmp: F)
where
    F: Fn(&T, &T) -> bool,
{
    let mut i = 0;
    while i < input.len() {
        if i == 0 || cmp(&input[i - 1], &input[i]) {
            i += 1;
        } else {
            input.swap(i - 1, i);
            i -= 1;
        }
    }
}
