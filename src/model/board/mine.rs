use super::{Point, MINE_COUNT, SIZE};
use rand::Rng;

pub(super) fn mine_positions() -> Vec<Point> {
    let mut rng = rand::thread_rng();

    // 長さがMINE_COUNTのpointの配列
    let mut p: Vec<Point> = vec![];

    while p.len() < MINE_COUNT {
        let new_p = Point {
            x: rng.gen_range(1..=SIZE),
            y: rng.gen_range(1..=SIZE),
        };
        if !p.contains(&new_p) {
            p.push(new_p);
        }
    }
    p
}
