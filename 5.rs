fn median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 1 {
        arr[len / 2] as f64
    } else {
        (arr[len / 2 - 1] + arr[len / 2]) as f64 / 2.0
    }
}
