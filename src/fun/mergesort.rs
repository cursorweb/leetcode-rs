pub fn merge_sort<T: Clone + PartialOrd>(arr: Vec<T>) -> Vec<T> {
    if arr.len() < 2 {
        return arr;
    }

    let half = arr.len() / 2;

    let mut left = merge_sort(arr[..half].to_vec()).into_iter().peekable();
    let mut right = merge_sort(arr[half..].to_vec()).into_iter().peekable();

    let mut out = Vec::new();
    while !left.peek().is_none() && !right.peek().is_none() {
        if left.peek() < right.peek() {
            out.push(left.next().unwrap());
        } else {
            out.push(right.next().unwrap());
        }
    }

    // could still have extras
    while !left.peek().is_none() {
        out.push(left.next().unwrap())
    }
    while !right.peek().is_none() {
        out.push(right.next().unwrap())
    }

    out
}
