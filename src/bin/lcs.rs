macro_rules! max {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => {{
        std::cmp::max($x, max!($($z),*))
    }}
}

#[allow(unused)]
fn main() {
    // input
    let n = 4;
    let m = 4;
    let s = "abcd".chars().collect::<Vec<_>>();
    let t = "becd".chars().collect::<Vec<_>>();

    let mut dp = vec![vec![0; m + 1]; n + 1];

    for i in 1..=n {
        let c_s = s[i - 1];
        for j in 1..=m {
            let c_t = t[j - 1];

            if c_s == c_t {
                dp[i][j] = max!(dp[i - 1][j], dp[i][j - 1], dp[i - 1][j - 1] + 1);
            } else {
                dp[i][j] = max!(dp[i - 1][j], dp[i][j - 1]);
            }
        }
    }

    println!("{}", dp[n][m]);
}
