fn merge_sorted_arrays(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    let mut i = 0;
    let mut j = 0;

    while i < a.len() && j < b.len() {
        if a[i] <= b[j] {
            result.push(a[i]);
            i += 1;
        } else {
            result.push(b[j]);
            j += 1;
        }
    }

    while i < a.len() {
        result.push(a[i]);
        i += 1;
    }

    while j < b.len() {
        result.push(b[j]);
        j += 1;
    }

    result
}
