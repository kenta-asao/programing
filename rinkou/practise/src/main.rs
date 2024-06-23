use std::collections::HashSet;

fn main() {
    let mut y_ok = vec![];
    let array2 = [5, 6, 7, 8, 9];

    for m in 0..3 {
        y_ok.push(m);
    }

    if has_common_elements(&y_ok, &array2) {
        println!("The arrays have common elements.");
    } else {
        println!("The arrays do not have common elements.");
    }
}

fn has_common_elements<T: std::cmp::Eq + std::hash::Hash>(array1: &[T], array2: &[T]) -> bool {
    let set1: HashSet<_> = array1.iter().collect();
    for element in array2.iter() {
        if set1.contains(element) {
            return true;
        }
    }
    false
}
