fn in_place_merge<T: PartialOrd + Clone>(arr: &mut [T], s: usize, m: usize, e: usize) {
    let mut s = s;
    let mut m = m;
    let mut s2 = m + 1;

    if arr[m] <= arr[s2] {
        return;
    }

    while s <= m && s2 <= e {
        if arr[s] <= arr[s2] {
            s += 1;
        } else {
            let v = arr[s2].clone();
            let mut i = s2;

            while i != s {
                arr[i] = arr[i - 1].clone();
                i -= 1;
            }
            arr[s] = v;
            s += 1;
            m += 1;
            s2 += 1;
        }
    }
}

fn in_place_merge_sorter<T: PartialOrd + Clone>(arr: &mut [T], s: usize, e: usize) {
    if s < e {
        let m = s + (e - s) / 2;
        in_place_merge_sorter(arr, s, m);
        in_place_merge_sorter(arr, m + 1, e);
        in_place_merge(arr, s, m, e);
    }
}

pub fn in_place_merge_sort<T: PartialOrd + Clone>(arr: &mut [T]) {
    in_place_merge_sorter(arr, 0, arr.len() - 1);
}
