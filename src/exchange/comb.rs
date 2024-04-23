pub fn comb_sort<T: PartialOrd>(input: &mut [T]) {
    let mut gap = input.len();
    const SHRINK: f64 = 1.3;
    let mut sorted = false;

    while !sorted {
        gap = f64::floor(gap as f64 / SHRINK) as usize;
        if gap <= 1 {
            gap = 1;
            sorted = true;
        } else if gap == 9 || gap == 10 {
            gap = 11;
        }

        let mut i = 0;
        while i + gap < input.len() {
            if input[i] > input[i + gap] {
                input.swap(i, i + gap);
                sorted = false;
            }
            i += 1;
        }
    }
}
pub fn comb_sort_cmp<T, F>(input: &mut [T], cmp: F)
where
    F: Fn(&T, &T) -> bool,
{
    let mut gap = input.len();
    const SHRINK: f64 = 1.3;
    let mut sorted = false;

    while !sorted {
        gap = f64::floor(gap as f64 / SHRINK) as usize;
        if gap <= 1 {
            gap = 1;
            sorted = true;
        } else if gap == 9 || gap == 10 {
            gap = 11;
        }

        let mut i = 0;
        while i + gap < input.len() {
            if cmp(&input[i], &input[i + gap]) {
                input.swap(i, i + gap);
                sorted = false;
            }
            i += 1;
        }
    }
}
