fn first_occurrence(arr: &[i32], num: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left <= right {
        let mid = left + (right - left) / 2;

        if arr[mid] < num {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    if left < arr.len() && arr[left] == num {
        Some(left)
    } else {
        None
    }
}
