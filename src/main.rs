#[macro_use]
mod read; // マクロ定義は ./read.rs
use std::io::Read;

fn main() {
    let (h, w) = read!(usize, usize);
    read! {
        let sy: usize;
        let sx: usize;
        let  n: usize;
        let mut stage: [['seqc; w]; h];
    }
    let mut moves = read![(i32, char); n];

    moves.push((100, '-'));
    stage[sy][sx] = '*';

    let mut pos = (sy, sx);
    let mut dir = 0;
    let mut now = 0;

    'entire_loop: for (move_till, left_or_right) in moves {
        while now < move_till {
            now += 1;
            pos = go(dir, pos);
            if !(inside((0, 0), (h, w), pos) && stage[pos.0][pos.1] == '.') {
                break 'entire_loop;
            }
            stage[pos.0][pos.1] = '*';
        }
        dir = turn(left_or_right, dir);
    }

    for line in stage {
        println!("{}", line.into_iter().collect::<String>());
    }
}

// 方角: north: 0, east: 1, south: 2, west: 3
fn turn(to: char, direction: i32) -> i32 {
    match to {
        'R' => (direction + 1) % 4,
        'L' => (direction + 3) % 4,
        _ => direction,
    }
}

fn go(direction: i32, current: (usize, usize)) -> (usize, usize) {
    match direction {
        0 => (current.0 - 1, current.1),
        1 => (current.0, current.1 + 1),
        2 => (current.0 + 1, current.1),
        3 => (current.0, current.1 - 1),
        _ => current,
    }
}

fn inside<T: Ord>((miny, minx): (T, T), (maxy, maxx): (T, T), (y, x): (T, T)) -> bool {
    miny <= y && y < maxy && minx <= x && x < maxx
}
