// https://leetcode.com/problems/unique-paths/
pub fn paths(height: i32, width: i32) -> i32 {
    math_path(height, width)
}

fn math_path(height: i32, width: i32) -> i32 {
    let top = fact(height + width - 2);
    let bottom = fact(width - 1) * fact(height - 1);
    top / bottom
}

fn fact(max: i32) -> i32 {
    let mut out = 1;
    let mut i = max;
    while i > 1 {
        out *= i;
        i -= 1;
    }

    out
}

/*
fn calc_path(sum: i32, x: i32, y: i32, height: i32, width: i32) -> i32 {
    if x == width - 1 && y == height - 1 {
        return 1 + sum;
    }

    let mut out = sum;
    if x < width {
        out += calc_path(sum, x + 1, y, height, width);
    }
    if y < height {
        out += calc_path(sum, x, y + 1, height, width);
    }

    out
}
*/