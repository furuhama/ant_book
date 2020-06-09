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
    // input
    let n = 4;
    let mut a = vec![1, 2, 4, 7];
    let k = 13;

    if dfs(0, 0, &mut a, k, n) {
        echo!("Yes");
    } else {
        echo!("No");
    }
}
