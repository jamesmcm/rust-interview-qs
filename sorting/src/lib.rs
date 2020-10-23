pub mod mergesort;
pub mod quicksort;

pub trait Sort<T: PartialOrd> {
    fn sort(slice: &mut [T]) -> &mut [T];
}
