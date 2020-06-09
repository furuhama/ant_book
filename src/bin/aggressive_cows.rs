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

fn main() {
    // input
    let n = 5;
    let m = 3;
    let x = vec![1, 2, 8, 4, 9];

    let mut x = x;
    x.sort();

    let calc = |len| {
        let mut last = 0;
        for _ in 1..m {
            let mut current = last + 1;
            while current < n && x[current] - x[last] < len {
                current += 1;
            }
            if current == n {
                return false;
            }
            last = current;
        }
        true
    };

    let ans = binary_search(0, 10usize.pow(9), calc);

    echo!(ans);
}
