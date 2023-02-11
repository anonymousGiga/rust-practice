use std::ops::Index;

fn bubble_sort(v: &mut Vec<i32>) {
    for i in 0..v.len() {
        for j in 1..(v.len() - i) {
            if v[j - 1] > v[j] {
                v.swap(j - 1, j);
            }
        }
    }
}

fn insert_sort(v: &mut Vec<i32>) {
    for i in 1..v.len() {
        let (mut j, value) = (i, v[i]);
        while j > 0 && v[j - 1] > value {
            v[j] = v[j - 1];
            j -= 1;
        }
        v[j] = value;
    }
}

fn selection_sort(v: &mut Vec<i32>) {
    for i in 0..v.len() {
        let mut value = i;
        for j in (i + 1)..v.len() {
            if v[value] > v[j] {
                value = j;
            }
        }
        v.swap(i, value);
    }
}

fn partition(v: &mut [i32], s: usize, e: usize) -> usize {
    let mut left = s;
    let mut right = e - 1;

    while left < right {
        while v[left] <= v[s - 1] && left < right {
            left += 1;
        }
        while v[right] >= v[s - 1] && left < right {
            right -= 1;
        }
        v.swap(left, right);
    }
    left
}

fn quick_sort(v: &mut [i32], s: usize, e: usize) {
    if s < e {
        let left = partition(v, s, e);
        if v[left] < v[s - 1] {
            v.swap(left, s - 1);
        }
        quick_sort(v, s, left);
        quick_sort(v, left + 1, e);
    }
}

fn merge(v: &mut Vec<i32>, left: usize, middle: usize, right: usize) {
    let (mut tmp, mut l, mut r): (Vec<i32>, usize, usize) = (Vec::new(), left, middle);
    while l < middle && r < right {
        if v[l] <= v[r] {
            tmp.push(v[l]);
            l += 1;
        } else {
            tmp.push(v[r]);
            r += 1;
        }
    }
    while l < middle {
        tmp.push(v[l]);
        l += 1;
    }
    while r < right {
        tmp.push(v[r]);
        r += 1;
    }
    let mut index = 0;
    for i in left..right {
        v[i] = tmp[index];
        index += 1;
    }
}

fn merge_sort(v: &mut Vec<i32>) {
    let length = v.len();
    let mut step = 2;
    let mut index = 0;

    while step <= length {
        index = 0;
        while index + step <= length {
            merge(v, index, index + step / 2, index + step);
            index += step;
        }
        merge(v, index, index + step / 2, length);
        step *= 2;
    }
    merge(v, 0, step / 2, length);
}

fn main() {
    let mut v1 = vec![12, 2, 1, 5, 7, 10, 3, 4];
    bubble_sort(&mut v1);
    println!("{:?}", v1);
    println!("++++++++++++++++++");

    let mut v2 = vec![12, 2, 1, 5, 7, 10, 3, 4];
    insert_sort(&mut v2);
    println!("{:?}", v2);
    println!("++++++++++++++++++");

    let mut v3 = vec![12, 2, 1, 5, 7, 10, 3, 4];
    selection_sort(&mut v3);
    println!("{:?}", v3);
    println!("++++++++++++++++++");

    let mut v4 = vec![12, 2, 1, 5, 7, 10, 3, 4];
    let n = v4.len();
    quick_sort(&mut v4, 1, n);
    println!("{:?}", v4);
    println!("++++++++++++++++++");

    let mut v5 = vec![12, 2, 1, 5, 7, 10, 3, 4];
    merge_sort(&mut v5);
    println!("{:?}", v5);
    println!("++++++++++++++++++");
}
