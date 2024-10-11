/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T>(array: &mut [T]) 
where
    T: Copy + PartialOrd,  // 能够排序需要T能够排序, 能够swap需要T能够拷贝
{
	//TODO
    if array.len() <= 1 {
        return
    };

    for i in 0..array.len() {
        for j in (i + 1)..array.len() {
            // println!("current iter is {} {}", i, j);
            if array[i] > array[j] {
                let mut temp = array[i];
                array[i] = array[j];
                array[j] = temp
            }
        }
    }
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