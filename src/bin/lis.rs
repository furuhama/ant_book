trait BinarySearchable<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}

impl<T: Ord> BinarySearchable<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut left = 0;
        let mut right = self.len();

        while left != right {
            let mid = (left + right) / 2;
            match self[mid].cmp(x) {
                std::cmp::Ordering::Less => {
                    left = mid + 1;
                }
                std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => {
                    right = mid;
                }
            }
        }
        left
    }

    fn upper_bound(&self, x: &T) -> usize {
        let mut left = 0;
        let mut right = self.len();

        while left != right {
            let mid = (left + right) / 2;
            match self[mid].cmp(x) {
                std::cmp::Ordering::Less | std::cmp::Ordering::Equal => {
                    left = mid + 1;
                }
                std::cmp::Ordering::Greater => {
                    right = mid;
                }
            }
        }
        left
    }
}

fn main() {
    // input
    let n = 5;
    let a = vec![4, 2, 3, 1, 5];

    let mut dp = vec![std::usize::MAX; n];

    for i in 0..n {
        let idx = dp.lower_bound(&a[i]);
        dp[idx] = a[i];
    }

    println!("{}", dp.lower_bound(&std::usize::MAX));
}
