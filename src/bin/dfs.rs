use proconio::input;

#[allow(unused_macros)]
macro_rules! echo {
    ($($e:expr),+) => ( { $(println!("{}", $e))+ } );
}

fn dfs(i: usize, sum: usize, a: &mut Vec<usize>, k: usize, n: usize) -> bool {
    if i == n {
        return sum == k;
    }

    if dfs(i + 1, sum, a, k, n) {
        return true;
    }

    if dfs(i + 1, sum + a[i], a, k, n) {
        return true;
    }

    false
}

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        k: usize,
    }

    if dfs(0, 0, &mut a, k, n) {
        echo!("Yes");
    } else {
        echo!("No");
    }
}
