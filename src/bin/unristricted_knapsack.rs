#[allow(unused)]
fn main() {
    // input
    let n = 4;
    let a = vec![(2, 3), (1, 2), (3, 4), (2, 2)];
    let w = 5;

    let mut dp = vec![vec![0; w + 1]; n + 1];

    for i in 1..=n {
        let stuff = a[i - 1];

        for j in 1..=w {
            if j < stuff.0 {
                dp[i][j] = dp[i - 1][j];
            } else {
                dp[i][j] = std::cmp::max(dp[i - 1][j], dp[i][j - stuff.0] + stuff.1);
            }
        }
    }

    println!("{}", dp[n][w]);
}
