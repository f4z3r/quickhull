//! Library module for quickhull algo
//!
//! Jakob Beckmann

use std::fmt;

/// Point structure.
pub struct Point {
    pub x: f64,
    pub y: f64
}

// Debug trait implementation for easy printing
impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

// Actual algorithm functions

/// Runs the quickhull algorithm.
///
/// # Arguments
/// - `input`: vector of points for which to compute the hull.
/// - `hull_idxs`: vector of integers that will receive the indeces of the hull points.
///   Note that if this is not empty, indeces will be added to the end of this vector.
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

/// Recursive nature of quickhull algorithm
///
/// Arguments
/// - `input`: vector of points for which to compute the hull.
/// - `indeces`: vector of indeces that are still in consideration.
/// - `output_idxs`: vector of indeces of points that are on the hull.
/// - `first`: first cut point index.
/// - `second`: second cut point index.
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

/// Returns the point of maximum distance to the cut segment and all points lying outside the
/// triangle created by this point and the segment.
///
/// Arguments:
/// - `input`: vector of points for which to compute the hull.
/// - `indeces`: vector of indeces that are still in consideration.
/// - `first`: first cut point index,
/// - `second`: second cut point index.
///
/// Returns
/// Returns a tuple containing the remaining points under consideration as a first element and
/// the index of the point of maximum distance. If so such point is found (because no points are
/// left in the `indeces` set or because the points remaining lie on the cut segment), -1 is
/// returned.
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

/// Computes the distance of a `target` point to the line segment build from `first` to `second`.
///
/// Arguments
/// - `input`: vector of points for which to compute the hull.
/// - `target`: target point for which the distance is computed.
/// - `first`: first point defining the line segment.
/// - `second`: second point defining the line segment.
///
/// Returns
/// Returns the distance as an `f64`.
fn compute_dist(input: &Vec<Point>, target: usize, first: usize, second: usize) -> f64 {
    // Uses a form of cross product
    let segment = (input[second].x - input[first].x, input[second].y - input[first].y);
    let vec_target = (input[target].x - input[first].x, input[target].y - input[first].y);

    return vec_target.1 * segment.0 - vec_target.0 * segment.1;

}


/// Find the left/right most point from a given list.
///
/// Arguments
/// - `input`: vector of points to consider.
/// - `left`: boolean defining whether to find leftmost or rightmost point.
///
/// Returns
/// Returns the index of the point found in `input`.
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
