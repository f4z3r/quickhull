//! Library module for quickhull algo
//!
//! Jakob Beckmann

use std::fmt;

pub struct Point {
    pub x: f64,
    pub y: f64
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

// Actual algorithm functions
pub fn run(input: &Vec<Point>, hull_idxs: &mut Vec<usize>) {
    let point_min_x = find_extreme(input, true);
    let point_max_x = find_extreme(input, false);

    let mut indeces: Vec<usize> = Vec::new();
    for idx in 0..input.len() {
        indeces.push(idx as usize);
    }

    hull_idxs.push(point_min_x);
    hull_idxs.push(point_max_x);

    sub_quickhull(input, &indeces, hull_idxs, point_min_x, point_max_x);
    sub_quickhull(input, &indeces, hull_idxs, point_max_x, point_min_x);
}

fn sub_quickhull(input: &Vec<Point>,
                 indeces: &Vec<usize>,
                 output_idxs: &mut Vec<usize>,
                 first: usize,
                 second: usize) {

    let (next_indeces, max_point) = get_dist_set(input, indeces, first, second);

    if max_point != -1 {
        output_idxs.push(max_point as usize);
        sub_quickhull(input, &next_indeces, output_idxs, first, max_point as usize);
        sub_quickhull(input, &next_indeces, output_idxs, max_point as usize, second);
    }
}

fn get_dist_set(input: &Vec<Point>,
                indeces: &Vec<usize>,
                first: usize,
                second: usize) -> (Vec<usize>, isize) {
    let mut result: Vec<usize> = Vec::new();

    let mut idx_max_distance = -1isize;
    let mut max_distance = 0f64;

    for idx in indeces {
        let temp_distance = compute_dist(input, *idx, first, second);

        if temp_distance > 0f64 {
            result.push(*idx);
            if temp_distance > max_distance {
                idx_max_distance = *idx as isize;
                max_distance = temp_distance;
            }
        }
    }

    return (result, idx_max_distance);
}

fn compute_dist(input: &Vec<Point>, target: usize, first: usize, second: usize) -> f64 {
    // Uses a form of cross product
    let segment = (input[second].x - input[first].x, input[second].y - input[first].y);
    let vec_target = (input[target].x - input[first].x, input[target].y - input[first].y);

    return vec_target.1 * segment.0 - vec_target.0 * segment.1;

}

fn find_extreme(input: &Vec<Point>, left: bool) -> usize {
    let mut pivot_idx: usize = 0;
    for idx in 0..input.len() {
        if (left && input[idx].x < input[pivot_idx].x) ||
                (!left && input[idx].x > input[pivot_idx].x) {
            pivot_idx = idx as usize;
        }
    }

    return pivot_idx;
}
