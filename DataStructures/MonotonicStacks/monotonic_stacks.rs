pub mod monotonic_stacks {
    use std::collections::VecDeque;

    pub fn monotonic_stack_next<T, F: Fn(&T, &T) -> bool>(a: &[T], cmp: F) -> Vec<usize> {
        let n = a.len();
        let mut st = VecDeque::new();
        let mut next = vec![0; n];
        for i in (0..n).rev() {
            while let Some(&j) = st.back() {
                if cmp(&a[i], &a[j]) {
                    st.pop_back();
                } else {
                    break;
                }
            }
            next[i] = st.back().copied().unwrap_or(n);
            st.push_back(i);
        }
        next
    }

    pub fn monotonic_stack_prev<T, F: Fn(&T, &T) -> bool>(a: &[T], cmp: F) -> Vec<isize> {
        let n = a.len();
        let mut st: VecDeque<isize> = VecDeque::new();
        let mut prev = vec![0; n];
        for i in 0..n {
            while let Some(&j) = st.back() {
                if cmp(&a[i], &a[j as usize]) {
                    st.pop_back();
                } else {
                    break;
                }
            }
            prev[i] = st.back().copied().unwrap_or(-1);
            st.push_back(i as isize);
        }
        prev
    }

    pub fn next_greater<T: PartialOrd>(a: &[T]) -> Vec<usize> {
        monotonic_stack_next(a, |&a, &b| a > b)
    }

    pub fn next_greater_equal<T: PartialOrd>(a: &[T]) -> Vec<usize> {
        monotonic_stack_next(a, |&a, &b| a >= b)
    }

    pub fn next_smaller<T: PartialOrd>(a: &[T]) -> Vec<usize> {
        monotonic_stack_next(a, |&a, &b| a < b)
    }

    pub fn next_smaller_equal<T: PartialOrd>(a: &[T]) -> Vec<usize> {
        monotonic_stack_next(a, |&a, &b| a <= b)
    }

    pub fn prev_greater<T: PartialOrd>(a: &[T]) -> Vec<isize> {
        monotonic_stack_prev(a, |&a, &b| a > b)
    }

    pub fn prev_greater_equal<T: PartialOrd>(a: &[T]) -> Vec<isize> {
        monotonic_stack_prev(a, |&a, &b| a >= b)
    }

    pub fn prev_smaller<T: PartialOrd>(a: &[T]) -> Vec<isize> {
        monotonic_stack_prev(a, |&a, &b| a < b)
    }

    pub fn prev_smaller_equal<T: PartialOrd>(a: &[T]) -> Vec<isize> {
        monotonic_stack_prev(a, |&a, &b| a <= b)
    }
}
use monotonic_stacks::*;
