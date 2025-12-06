pub mod interval_container {
    use std::collections::BTreeSet;
    use std::ops::Bound::{Included, Unbounded};

    pub struct IntervalContainer {
        pub intervals: BTreeSet<(usize, usize)>,
        pub elements: usize,
    }

    impl IntervalContainer {
        pub fn new() -> Self {
            Self {
                intervals: BTreeSet::new(),
                elements: 0,
            }
        }

        pub fn add(&mut self, mut l: usize, mut r: usize) {
            while let Some((x, y)) = self
                .intervals
                .range((Included((l, l)), Unbounded))
                .next()
                .copied()
            {
                if x > r + 1 {
                    break;
                }

                self.intervals.remove(&(x, y));
                self.elements -= y - x + 1;
                l = l.min(x);
                r = r.max(y);
            }

            while let Some((x, y)) = self
                .intervals
                .range((Unbounded, Included((l, l))))
                .next_back()
                .copied()
            {
                if l > y + 1 {
                    break;
                }

                self.intervals.remove(&(x, y));
                self.elements -= y - x + 1;
                l = l.min(x);
                r = r.max(y);
            }

            self.intervals.insert((l, r));
            self.elements += r - l + 1;
        }
    }
}
use interval_container::*;
