#[allow(unused_macros)]
macro_rules! echo {
    ($($e:expr),+) => ( { $(println!("{}", $e))+ } );
}

fn binary_search<
    T: std::ops::Add<Output = T> + std::ops::Div<Output = T> + PartialEq + From<u8> + Copy,
>(
    lower: T,
    upper: T,
    proc: impl Fn(T) -> bool,
) -> T {
    let mut lower = lower;
    let mut upper = upper;

    let div = T::from(2);

    loop {
        let mid = (lower + upper) / div;
        if lower == mid || mid == upper {
            break mid;
        }

        if proc(mid) {
            lower = mid;
        } else {
            upper = mid;
        }
    }
}

#[allow(unused_variables)]
fn main() {
    // input
    let n = 4;
    let k = 11;
    let l = vec![8.02, 7.43, 4.57, 5.39];

    let calc = |f| {
        let mut sum = 0;
        for e in &l {
            sum += (e / f) as usize;
        }
        sum >= k
    };
    let ans = binary_search(0.0, 10001.0, calc);

    println!("{:.2}", ans);
}
