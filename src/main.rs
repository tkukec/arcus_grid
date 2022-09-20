pub mod data;
pub mod misc;
use crate::misc::*;

fn main() {
    let (vis, morse) = data::parse_files();

    for i in 0..6 {
        let d: Vec<_> = vis.iter().map(|x| x[i]).collect();
        generate_image(&d, false, &format!("visual_by_second/second_{i}_vis.png"))
    }

    for i in 0..6 {
        let d: Vec<_> = morse.iter().map(|x| x[i]).collect();
        generate_image(&d, false, &format!("morse_by_second/second_{i}_morse.png"))
    }
}
