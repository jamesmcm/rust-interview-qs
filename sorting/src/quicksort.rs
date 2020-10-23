use super::Sort;

struct QuickSort {}

impl QuickSort {
    /// Move element at index partition_elem to correct place in list]
    /// and return its new index
    /// We always use last element as pivot element otherwise splitting is painful!
    fn partition<T: PartialOrd>(slice: &mut [T]) -> usize {
        unsafe {
            // Pivot element is last element in slice
            let (pivot, slice) = slice.split_last_mut().unwrap();
            let mut i = 0;
            for j in 0..slice.len() {
                // Ensure all elements less than pivot are to the left of i
                if slice[j] < *pivot {
                    slice.swap(j, i);
                    i += 1;
                }
            }

            // Recombine split slice (i.e. combining with pivot element on the end)
            let len = slice.len() + 1;
            let ptr = slice.as_mut_ptr();
            let slice = std::slice::from_raw_parts_mut(ptr, len);
            // Swap the pivot element with the ith element
            // (i.e. the first element in the slice >= pivot)
            // Pivot element is now in the correct position :)
            slice.swap(len - 1, i);
            i
        }
    }

    pub fn quicksort<T: PartialOrd>(slice: &mut [T]) -> &mut [T] {
        if slice.len() <= 1 {
            return slice;
        }

        let orig_len = slice.len();

        let pos = Self::partition(slice);

        let (left, right) = slice.split_at_mut(pos);
        let (_partition, right) = right.split_first_mut().unwrap();

        let leftsort = Self::quicksort(left);
        let _rightsort = Self::quicksort(right);

        let ptr = leftsort.as_mut_ptr();
        unsafe { std::slice::from_raw_parts_mut(ptr, orig_len) }
    }
}

impl<T: PartialOrd> Sort<T> for QuickSort {
    fn sort(slice: &mut [T]) -> &mut [T] {
        QuickSort::quicksort(slice)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut to_sort = vec![3, 4, 1, 2];
        let sorted = QuickSort::sort(&mut to_sort);
        assert_eq!(sorted, &[1, 2, 3, 4]);
    }
    #[test]
    fn it_works2() {
        let mut to_sort = vec![1, 5, 7, 2, 6, 8];
        let sorted = QuickSort::sort(&mut to_sort);
        assert_eq!(sorted, &[1, 2, 5, 6, 7, 8]);
    }
    #[test]
    fn it_works3() {
        let mut to_sort = vec![
            "aa".to_string(),
            "ac".to_string(),
            "aa".to_string(),
            "ab".to_string(),
        ];
        let sorted = QuickSort::sort(&mut to_sort);
        assert_eq!(
            sorted,
            &[
                "aa".to_string(),
                "aa".to_string(),
                "ab".to_string(),
                "ac".to_string()
            ]
        );
    }
}
