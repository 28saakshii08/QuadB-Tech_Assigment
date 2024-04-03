use std::collections::BinaryHeap;

fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    let mut heap = BinaryHeap::new();

    for &num in arr {
        if heap.len() < k {
            heap.push(num);
        } else if num < *heap.peek().unwrap() {
            heap.pop();
            heap.push(num);
        }
    }

    heap.peek().cloned()
}
