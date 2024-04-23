pub fn cocktail_sort<T: PartialOrd>(input: &mut [T]) {
    let mut swapped = true;
    let mut start = 0;
    let mut end = input.len() - 1;

    while swapped {
        swapped = false;

        for i in start..end {
            if input[i] > input[i + 1] {
                input.swap(i, i + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }

        swapped = false;
        end -= 1;

        for i in (start..end).rev() {
            if input[i] > input[i + 1] {
                input.swap(i, i + 1);
                swapped = true;
            }
        }

        start += 1;
    }
}

pub fn cocktail_sort_cmp<T, F>(input: &mut [T], cmp: F)
where
    F: Fn(&T, &T) -> bool,
{
    let mut swapped = true;
    let mut start = 0;
    let mut end = input.len() - 1;

    while swapped {
        swapped = false;

        for i in start..end {
            if cmp(&input[i], &input[i + 1]) {
                input.swap(i, i + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }

        swapped = false;
        end -= 1;

        for i in (start..end).rev() {
            if cmp(&input[i], &input[i + 1]) {
                input.swap(i, i + 1);
                swapped = true;
            }
        }

        start += 1;
    }
}
