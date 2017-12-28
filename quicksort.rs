

fn quicksort(array: &mut[isize], first: usize, last: usize) {
    if first < last {
        let midpoint = partition(array, first, last);
        quicksort(array, first, midpoint - 1);
        quicksort(array, midpoint + 1, last);
    }
}


fn partition(array: &mut[isize], first: usize, last: usize) -> usize {
    let pivot = array[last];

    let i: isize = first as isize;
    let mut i: isize = i - 1;

    let mut j = first;
    while j <= last - 1 {
        if array[j] < pivot {
            i = i + 1;
            let k: usize = i as usize;
            swap(array, k, j);
        }
        j = j + 1;
    }

    let k: isize = i + 1;
    let k: usize = k as usize;
    if array[last] < array[k] {
        swap(array, k, last);
    }
    return k;
}


fn swap(array: &mut[isize], a: usize, b: usize) {
    let temp = array[a];
    array[a] = array[b];
    array[b] = temp;
}


fn main() {
    let mut array = [3, 5, 1, 4, 2, 6, 9, 8, 7];
    quicksort(&mut array, 0, 8);

    println!("{:?}", array);
}
