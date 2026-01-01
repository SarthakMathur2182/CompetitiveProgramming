/// # Monotonic Stack ([source](https://github.com/SarthakMathur2182/CompetitiveProgramming/blob/main/DataStructures/MonotonicStacks/monotonic_stacks.rs))
///
/// Added some common utils (next greater, prev smaller, etc.) and generic util (for prev and next).
pub mod monotonic_stacks {
    use std::collections::VecDeque;

    /// `cmp` will be called like `cmp(curr_value, next_value)`
    ///
    /// If we want the next smaller element, then
    ///
    /// `cmp = |curr_value, next_value| -> next_value < curr_value`
    pub fn monotonic_stack_next<T, F: Fn(&T, &T) -> bool>(a: &[T], cmp: F) -> Vec<usize> {
        let n = a.len();
        let mut st = VecDeque::new();
        let mut next = vec![0; n];
        for i in (0..n).rev() {
            while let Some(&j) = st.back() {
                if cmp(&a[i], &a[j]) {
                    break;
                } else {
                    st.pop_back();
                }
            }
            next[i] = st.back().copied().unwrap_or(n);
            st.push_back(i);
        }
        next
    }

    /// `cmp` will be called like `cmp(curr_value, prev_value)`
    ///
    /// If we want the previous smaller element, then
    ///
    /// `cmp = |curr_value, prev_value| -> prev_value < curr_value`
    pub fn monotonic_stack_prev<T, F: Fn(&T, &T) -> bool>(a: &[T], cmp: F) -> Vec<isize> {
        let n = a.len();
        let mut st: VecDeque<isize> = VecDeque::new();
        let mut prev = vec![0; n];
        for i in 0..n {
            while let Some(&j) = st.back() {
                if cmp(&a[i], &a[j as usize]) {
                    break;
                } else {
                    st.pop_back();
                }
            }
            prev[i] = st.back().copied().unwrap_or(-1);
            st.push_back(i as isize);
        }
        prev
    }

    pub fn next_greater<T: PartialOrd>(a: &[T]) -> Vec<usize> {
        monotonic_stack_next(a, |curr_value, next_value| next_value > curr_value)
    }

    pub fn next_greater_equal<T: PartialOrd>(a: &[T]) -> Vec<usize> {
        monotonic_stack_next(a, |curr_value, next_value| next_value >= curr_value)
    }

    pub fn next_smaller<T: PartialOrd>(a: &[T]) -> Vec<usize> {
        monotonic_stack_next(a, |curr_value, next_value| next_value < curr_value)
    }

    pub fn next_smaller_equal<T: PartialOrd>(a: &[T]) -> Vec<usize> {
        monotonic_stack_next(a, |curr_value, next_value| next_value <= curr_value)
    }

    pub fn prev_greater<T: PartialOrd>(a: &[T]) -> Vec<isize> {
        monotonic_stack_prev(a, |curr_value, prev_value| prev_value > curr_value)
    }

    pub fn prev_greater_equal<T: PartialOrd>(a: &[T]) -> Vec<isize> {
        monotonic_stack_prev(a, |curr_value, prev_value| prev_value >= curr_value)
    }

    pub fn prev_smaller<T: PartialOrd>(a: &[T]) -> Vec<isize> {
        monotonic_stack_prev(a, |curr_value, prev_value| prev_value < curr_value)
    }

    pub fn prev_smaller_equal<T: PartialOrd>(a: &[T]) -> Vec<isize> {
        monotonic_stack_prev(a, |curr_value, prev_value| prev_value <= curr_value)
    }
}
use monotonic_stacks::*;
