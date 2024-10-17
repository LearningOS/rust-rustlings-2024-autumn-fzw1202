/*
    heap
    This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default + Copy,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default + Copy,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn swap(&mut self, idx: usize, parent_idx: usize) {
        let temp = self.items[idx];
        self.items[idx] = self.items[parent_idx];
        self.items[parent_idx] = temp;

        println!("idx: {} parent_idx: {}", idx, parent_idx);
    }

    pub fn add(&mut self, value: T) {
        //TODO
        self.count += 1;

        let mut idx = self.count;
        let mut parent_idx = self.parent_idx(idx);

        if self.items.len() < idx + 1 {
            self.items.push(value);
        } else {
            self.items[idx] = value;
        }

        while parent_idx > 0 && (self.comparator)(&self.items[idx], &self.items[parent_idx]) {
            self.swap(idx, parent_idx);
            idx = parent_idx;
            parent_idx = self.parent_idx(idx);
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, _idx: usize) -> usize {
        //TODO
        1
    }
}

impl<T> Heap<T>
where
    T: Default + Ord + Copy,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let res = self.items[1];
            self.items[1] = self.items[self.count];
            self.count -= 1;

            let mut idx = 1;
            let mut child_idx = self.left_child_idx(idx);

            while child_idx <= self.count {
                if child_idx + 1 > self.count {
                    if !(self.comparator)(&self.items[idx], &self.items[child_idx]) {
                        self.swap(idx, child_idx);
                    }
                    break;
                } else {
                    if (self.comparator)(&self.items[child_idx + 1], &self.items[child_idx]) {
                        child_idx += 1;
                    }

                    if !(self.comparator)(&self.items[idx], &self.items[child_idx]) {
                        self.swap(idx, child_idx);
                        idx = child_idx;
                        child_idx = self.left_child_idx(idx);
                    } else {
                        break;
                    }
                }
            }
            Some(res)
        }
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Copy,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Copy,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}
