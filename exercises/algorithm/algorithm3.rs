/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/


fn merge_sort<T: std::cmp::PartialOrd + Copy>(array: &mut [T]) {
    if array.len() <= 1 {
        return;
    }

    let mid = (0 + array.len()) >> 1;
    merge_sort(&mut array[0..mid]);
    merge_sort(&mut array[mid..]);
    
    let mut buf = Vec::with_capacity(array.len());
    
    let mut a = 0;
    let mut b = mid;
    while a < mid && b < array.len() {
        if array[a] < array[b] {
            buf.push(array[a]);
            a += 1;
        } else {
            buf.push(array[b]);
            b += 1;
        }
    }
    while a < mid {
        buf.push(array[a]);
        a += 1;
    }
    while b < array.len() {
        buf.push(array[b]);
        b += 1;
    }

    for (i, elem) in array.iter_mut().enumerate() {
        *elem = buf[i];
    }
}

fn sort<T: std::cmp::PartialOrd + Copy>(array: &mut [T]){
    merge_sort(array);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}