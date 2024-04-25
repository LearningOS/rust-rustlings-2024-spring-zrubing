/*
    heap
    This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default + std::fmt::Debug,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn shift_up(&mut self, mut i: usize) {

        println!("shift up:{}",i);
        loop {
            // 已经是堆顶，结束
            if i == 0 {
                break;
            }
            let p = self.parent_idx(i);
            println!("loop shift up index:{},parent index is:{}",i,p);

            if (self.comparator)(self.items.get(p).unwrap(), self.items.get(i).unwrap()) {
                break;
            }

            println!("loop shift up index:{},parent index is:{},do swap",i,p);
            self.items.swap(i, p);

            i = p;
        }
    }

    pub fn add(&mut self, value: T) {
        //TODO

        dbg!(&self.items);
        //添加节点
        self.items.push(value);

        self.count += 1;

        //从底到顶堆化
        self.shift_up(self.count - 1);


    }

    fn shift_down(&mut self, mut i:usize) {
        loop {
           println!("shift down:{}", i);
           // 最大节点，记为max 
           let (l,r,mut ma) =( self.left_child_idx(i), self.right_child_idx(i),i );

           let left_val = self.items.get(l);
           let right_val = self.items.get(r);
           let ma_val = self.items.get(ma).unwrap();

           if l < self.items.len() && (self.comparator)(left_val.unwrap(),ma_val) {
            ma = l;
           }
           if r < self.items.len() && (self.comparator)(right_val.unwrap(),ma_val){
            ma = r;
           }

           if ma == i {
             break;
           }
           self.items.swap(i, ma);

           i = ma;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        println!("pop count:{}",self.count);

        //交换堆顶和堆底
        self.items.swap(0, self.count - 1);
        let val = self.items.remove(self.count-1);

        self.count -=1;

        self.shift_down(0);

        Some(val)

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

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
        0
    }
}

impl<T> Heap<T>
where
    T: Default + Ord + std::fmt::Debug,
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
    T: Default + std::fmt::Debug,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO

        self.pop()
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + std::fmt::Debug,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + std::fmt::Debug,
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
        dbg!("here",&heap.items);
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
