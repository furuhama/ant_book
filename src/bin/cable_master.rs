#[allow(unused_macros)]
macro_rules! echo {
    ($($e:expr),+) => ( { $(println!("{}", $e))+ } );
}

fn calc(len: f64, l: &Vec<f64>, k: usize) -> bool {
    let mut sum = 0;
    for e in l {
        sum += (e / len) as usize;
    }

    sum >= k
}

#[allow(unused_variables)]
fn main() {
    // input
    let n = 4;
    let k = 11;
    let l = vec![8.02, 7.43, 4.57, 5.39];

    let mut mid = 0.0;
    let mut lb = 0.0;
    let mut ub = 10001.0;

    for _ in 0..100 {
        mid = (lb + ub) / 2.0;

        if calc(mid, &l, k) {
            lb = mid;
        } else {
            ub = mid;
        }
    }

    println!("{:.2}", mid);
}
