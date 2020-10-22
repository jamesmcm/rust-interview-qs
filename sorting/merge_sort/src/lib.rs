pub trait Sort<T: PartialOrd> {
    fn sort(slice: &mut [T]) -> &mut [T];
}

struct MergeSort {}

impl MergeSort {
    pub fn mergesort<T: PartialOrd>(slice: &mut [T]) -> &mut [T] {
        if slice.len() <= 1 {
            return slice;
        }
        let (left, right) = slice.split_at_mut(slice.len() / 2);
        let left = Self::mergesort(left);
        let right = Self::mergesort(right);
        Self::merge(left, right)
    }

    pub fn merge<'a, T: PartialOrd>(left: &'a mut [T], right: &mut [T]) -> &'a mut [T] {
        // in-place is too hard :( - https://stackoverflow.com/questions/2571049/how-to-sort-in-place-using-the-merge-sort-algorithm
        let mut out_slice = Vec::with_capacity(left.len() + right.len());

        let mut i = 0;
        let mut j = 0;
        while i < left.len() || j < right.len() {
            if i < left.len() && (j == right.len() || left[i] <= right[j]) {
                // Here we really want to pop from front of slice
                let val = unsafe { std::ptr::read(&mut left[i]) };
                out_slice.push(val);
                i += 1;
            } else {
                // Here we really want to pop from front of slice
                let val = unsafe { std::ptr::read(&mut right[j]) };
                out_slice.push(val);
                j += 1;
            }
        }

        unsafe {
            // Copy our allocated vector back over the two adjacent, contiguous slices we were given
            // And return as a single slice
            let len = out_slice.len();
            let ptr = left.as_mut_ptr();
            let out_ptr = out_slice.as_ptr();

            // Forget so we don't trigger destructors when we overwrite with copy_nonoverlapping
            // TODO: Does this leak memory or does dropping the final new slice still drop the
            // elements?
            std::mem::forget(out_slice);
            // Note copy_nonoverlapping handles the size_of::<T> part for us
            std::ptr::copy_nonoverlapping(out_ptr, ptr, len);
            std::slice::from_raw_parts_mut(ptr, len)
        }
    }
}

impl<T: PartialOrd> Sort<T> for MergeSort {
    fn sort(slice: &mut [T]) -> &mut [T] {
        MergeSort::mergesort(slice)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut to_sort = vec![3, 4, 1, 2];
        let sorted = MergeSort::sort(&mut to_sort);
        assert_eq!(sorted, &[1, 2, 3, 4]);
    }
    #[test]
    fn it_works2() {
        let mut to_sort = vec![1, 5, 7, 2, 6, 8];
        let sorted = MergeSort::sort(&mut to_sort);
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
        let sorted = MergeSort::sort(&mut to_sort);
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
