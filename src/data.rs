use std::fs;

use crate::misc::Point;
/// Read the input data into a tuple of two vectors.
///
/// The first vector contains the points from the visuals, separated by days.
/// The second vector contains the translated morse code, also separated by days.
pub fn parse_files() -> (Vec<Vec<Point>>, Vec<Vec<Point>>) {
    let vis_path = "files/square_codes.txt";
    let morse_path = "files/morse_codes.txt";
    let vis_file = fs::read_to_string(vis_path).expect("Couldn't find square_codes.txt");

    let morse_file = fs::read_to_string(morse_path).expect("Couldn't find square_codes.txt");

    let mut vis = vec![];
    let mut cur = vec![];
    for l in vis_file.lines() {
        if l.is_empty() {
            continue;
        }
        if l.starts_with('0') {
            if !cur.is_empty() {
                vis.push(cur.clone());
                cur = vec![];
            }
        } else {
            cur.push(Point::from(l));
        }
    }
    vis.push(cur);

    let mut morse = vec![];
    let mut cur = vec![];
    for l in morse_file.lines() {
        if l.is_empty() {
            continue;
        }
        if l.starts_with('0') {
            if !cur.is_empty() {
                morse.push(cur.clone());
                cur = vec![];
            }
        } else {
            cur.push(Point::from(l));
        }
    }
    morse.push(cur);

    (vis, morse)
}
