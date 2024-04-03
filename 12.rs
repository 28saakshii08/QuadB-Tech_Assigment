fn max_subarray_sum(arr: &[i32]) -> i32 {
    let mut max_so_far = arr[0];
    let mut max_ending_here = arr[0];

    for &num in &arr[1..] {
        max_ending_here = i32::max(num, max_ending_here + num);
        max_so_far = i32::max(max_so_far, max_ending_here);
    }

    max_so_far
}
