#[allow(unused_macros)]
macro_rules! echo {
    ($($e:expr),+) => ( { $(println!("{}", $e))+ } );
}

#[allow(unused_variables)]
fn main() {
    // input
    let n = 3;
    let k = 2;
    let wv = vec![(2, 2), (5, 3), (2, 1)];

    let mut a = wv
        .iter()
        .enumerate()
        .map(|(idx, wv)| (wv.1 / wv.0, idx))
        .collect::<Vec<_>>();
    a.sort();
    a.reverse();

    let res = a
        .into_iter()
        .take(k)
        .map(|(_, idx)| idx)
        .collect::<Vec<_>>();

    let mut numerator = 0;
    let mut denominator = 0;

    for i in res {
        let t = wv[i];
        numerator += t.1;
        denominator += t.0;
    }

    let ans = numerator as f64 / denominator as f64;
    println!("{}", ans);
}
