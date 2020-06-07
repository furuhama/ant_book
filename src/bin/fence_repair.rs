#[allow(unused)]
fn main() {
    // input
    let n = 3;
    let l = vec![8, 5, 8];

    let mut bh = std::collections::BinaryHeap::new();
    let mut ans = 0;
    for e in l.into_iter() {
        bh.push(std::cmp::Reverse(e));
    }
    while let Some(std::cmp::Reverse(x)) = bh.pop() {
        if let Some(std::cmp::Reverse(y)) = bh.pop() {
            let sum = x + y;
            bh.push(std::cmp::Reverse(sum));
            ans += sum;
        } else {
            break;
        }
    }
    println!("{}", ans);
}
