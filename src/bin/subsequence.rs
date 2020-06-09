#[allow(unused_macros)]
macro_rules! echo {
    ($($e:expr),+) => ( { $(println!("{}", $e))+ } );
}

fn main() {
    // input
    let n = 10;
    let s = 15;
    let a = vec![5, 1, 3, 5, 10, 7, 4, 9, 2, 8];

    let mut begin = 0;
    let mut end = 0;
    let mut sum = 0;
    let mut res = n + 1; // 与えられる数列の長さより大きくしておく

    loop {
        while end < n && sum < s {
            sum += a[end];
            end += 1;
        }

        if sum < s {
            break;
        }

        res = std::cmp::min(res, end - begin);
        sum -= a[begin];
        begin += 1;
    }

    if res > n {
        echo!("0");
    } else {
        echo!(res);
    }
}
