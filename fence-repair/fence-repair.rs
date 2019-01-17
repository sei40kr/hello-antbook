use std::collections::BinaryHeap;

// fence-repair.rs --- Fence Repair
// author: Seong Yong-ju <sei40kr@gmail.com>

fn solve(ws: &Vec<i32>) {
    let mut queue = BinaryHeap::<i32>::from(ws.iter().map(|&w| -w).collect::<Vec<_>>());
    let mut ans = 0;

    while 1 < queue.len() {
        let w1 = queue.pop().unwrap();
        let w2 = queue.pop().unwrap();
        let w = w1 + w2;
        ans += w;
        queue.push(w);
    }

    println!("{}", -ans);
}
