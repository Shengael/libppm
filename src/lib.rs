use std::ops::Not;
use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

pub struct Image {
    pub width: i32,
    pub height: i32,
    pub pixels: Vec<Pixel>,
}

impl Pixel {
    pub fn create(r: u8, g: u8, b: u8) -> Pixel {
        Pixel { r, g, b }
    }

    pub fn display(&self) {
        println!("red: {} green: {} blue: {}", self.r, self.g, self.b);
    }

    pub fn revert(&mut self) {
        self.r = 255 - self.r;
        self.g = 255 - self.g;
        self.b = 255 - self.b;
    }

    pub fn grayscale(&mut self) {
        let gray = (self.r as f32 * 0.3 + self.g as f32 * 0.59 + self.b as f32 * 0.11) as u8;
        self.r = gray;
        self.g = gray;
        self.b = gray;
    }

//    pub fn eq(&mut self, p: Pixel) -> bool {
//        self.r == p.r && self.g == p.g && self.b == p.b
//    }
}

impl Not for Pixel {
    type Output = Pixel;

    fn not(self) -> Self::Output {
        return Pixel { r: 255 - self.r, g: 255 - self.g, b: 255 - self.b };
    }
}

impl Image {
    pub fn create(width: i32, height: i32, pixels: Vec<Pixel>) -> Image {
        Image { width, height, pixels }
    }

    pub fn new_with_file(filename: &Path) -> Option<Image> {
        let mut img = Image {
            width: 0,
            height: 0,
            pixels: vec![],
        };

        let file = File::open(filename).expect("error");
        let buffer = BufReader::new(file);
        let mut count = -1;

        for (_i, line) in buffer.lines().enumerate().by_ref() {
            let l = line.unwrap();
            if l.chars().next().unwrap() == '#' { continue; }
            count = count + 1;

            if count == 0 {
                if &l != "P3" { return None; }
            } else if count == 1 {
                let size = get_size(l);
                img.width = size[0];
                img.height = size[1];
            } else if count > 2 {
                let pixels: Vec<u8> = get_numbers_from_line(l);
                if pixels.len() != 3 { return None; }
                img.pixels.push(Pixel::create(pixels[0], pixels[1], pixels[2]));
            }
        }
        return Some(img);
    }
}

pub fn get_numbers_from_line(line: String) -> Vec<u8> {
    line.split('#').collect::<Vec<&str>>()[0].split_ascii_whitespace()
        .map(|col| col.parse::<u8>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_else(|err| panic!("Cannot parse a column: {}", err))
}

pub fn get_size(line: String) -> Vec<i32> {
    line.split("#").collect::<Vec<&str>>()[0].split_ascii_whitespace()
        .map(|col| col.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_else(|err| panic!("Cannot parse a column: {}", err))
}