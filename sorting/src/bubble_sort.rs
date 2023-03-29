use std::fmt::Debug;

use crate::SortingAlgorithm;

pub trait BubbleSort {
    fn bubble_sort(&mut self);
    fn bubble_step(&mut self) -> bool;
}

impl<T: BubbleSort> SortingAlgorithm for T {
    fn step(&mut self) -> bool {
        self.bubble_step()
    }

    fn sort(&mut self) {
        self.bubble_sort()
    }
}

impl<T> BubbleSort for Vec<T>
where T: PartialOrd + Debug
{
    fn bubble_sort(&mut self) {
        while !self.bubble_step() {}
    }

    fn bubble_step(&mut self) -> bool {
        let mut complete = true;
        let mut swap_list = Vec::new();

        for (index, [a, b]) in self.array_windows().enumerate() {
            if a > b {
                swap_list.push(index);
            }
        }

        for index in swap_list {
            complete = false;
            self.swap(index, index + 1);
        }

        complete
    }
}