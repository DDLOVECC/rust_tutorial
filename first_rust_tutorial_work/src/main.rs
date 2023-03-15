fn bubble_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn sort<T: PartialOrd>(array: &mut [T]) {
    for i in 0..array.len() {
        for j in i + 1..array.len() {
            if array[i] > array[j] {
                array.swap(i, j);
            }
        }
    }
}

fn main() {
    let mut arr = [5, 12, 1, 8, 6, 3];

    //指定类型
    bubble_sort(&mut arr);
    println!("指定类型:{:?}", arr);
    let mut arrs = [15, 12, 33,78, 6, 3];
    //任意类型
    sort(&mut arrs);
    println!("任意类型:{:?}", arrs)
}