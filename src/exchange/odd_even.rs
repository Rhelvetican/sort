pub fn odd_even_sort<T: PartialOrd>(input: &mut [T]) {
    let mut sorted = false;
    while !sorted {
        sorted = true;
        for i in (1..input.len() - 1).step_by(2) {
            if input[i] > input[i + 1] {
                input.swap(i, i + 1);
                sorted = false;
            }
        }
        for i in (0..input.len() - 1).step_by(2) {
            if input[i] > input[i + 1] {
                input.swap(i, i + 1);
                sorted = false;
            }
        }
    }
}

pub fn odd_even_sort_cmp<T, F>(input: &mut [T], cmp: F)
where
    F: Fn(&T, &T) -> bool,
{
    let mut sorted = false;
    while !sorted {
        sorted = true;
        for i in (1..input.len() - 1).step_by(2) {
            if cmp(&input[i], &input[i + 1]) {
                input.swap(i, i + 1);
                sorted = false;
            }
        }
        for i in (0..input.len() - 1).step_by(2) {
            if cmp(&input[i], &input[i + 1]) {
                input.swap(i, i + 1);
                sorted = false;
            }
        }
    }
}
