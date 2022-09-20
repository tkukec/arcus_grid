use image::{Rgb, RgbImage};
use std::fmt;

/// Point data type
///
/// Stores the data for a point
///
/// # Examples
///
/// Basic usage:
/// ```no_run
/// use crate::misc::Point;
/// let pt = Point::from("A5Y");
///
/// println!("{:?}", pt.get_pos());
/// // (0, 4)
///
/// println!("{:?}", pt.get_rgb());
/// // Rgb([252, 237, 0])
///
/// ```
#[derive(Clone, Copy)]
pub struct Point {
    chr: char,
    num: u8,
    col: char,
}

impl From<&str> for Point {
    fn from(item: &str) -> Self {
        assert_eq!(item.len(), 3);
        let mut it = item.chars().map(|x| x.to_ascii_uppercase());
        let chr = it.next().unwrap();
        let num = it.next().unwrap().to_digit(10).unwrap() as u8;
        let col = it.next().unwrap();
        assert!(('A'..='F').contains(&chr));
        assert!((1u8..=6).contains(&num));
        assert!("ROYGBIV?".contains(col));
        Point { chr, num, col }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{}", self.chr, self.num, self.col)
    }
}
impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{}", self.chr, self.num, self.col)
    }
}

impl Point {
    /// Get the coordinates of a point
    pub fn get_pos(&self) -> (u32, u32) {
        let x = self.chr as u32 - 'A' as u32;
        let y = self.num as u32 - 1;
        (x, y)
    }

    pub fn get_transposed(&self) -> (u32, u32) {
        let (x, y) = self.get_pos();
        (y, x)
    }

    /// Get the color rgb value
    /// (the hex values were extracted from the video)
    pub fn get_rgb(&self) -> Rgb<u8> {
        match self.col {
            'R' => Rgb([255, 24, 0]),
            'O' => Rgb([255, 133, 0]),
            'Y' => Rgb([252, 237, 0]),
            'G' => Rgb([0, 109, 0]),
            'B' => Rgb([0, 15, 255]),
            'I' => Rgb([80, 15, 134]),
            'V' => Rgb([158, 27, 219]),
            '?' => Rgb([135, 74, 43]), // brown
            _ => unreachable!(),
        }
    }
}

/// Generate an image for a given list of points
/// # Arguments
/// `points` - a reference to a list of points to use
/// `show` - display the image after it is generated (depends on feh)
/// `path` - the name you want to use for the generated image
///
/// # Examples
///
/// Basic usage:
/// ```no_run
/// use crate::misc::{Point, generate_image};
/// let pts = [Point::from("A5Y"), Point::from("F6I")];
///
/// generate_image(&pts, false, "0.png");
/// // Creates an image generated/0.png
///
/// ```
pub fn generate_image(points: &[Point], show: bool, path: &str) {
    let mut img = RgbImage::new(6, 6);
    for p in points {
        let c = p.get_rgb();
        let (x, y) = p.get_pos();
        img.put_pixel(x, y, c);
    }
    let path = "generated/".to_owned() + path;
    img.save(&path).expect("Couldn't save image");
    if show {
        std::process::Command::new("feh")
            .args(["--force-aliasing", "--auto-zoom", &path])
            .spawn()
            .expect("Couldn't display image")
            .wait()
            .expect("Image display error");
    }
}
