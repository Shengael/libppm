// This crate is a library
#![crate_type = "lib"]
// This crate is named "ppm"
#![crate_name = "ppm"]

#![feature(test)] //use cargo +nightly

#[cfg(test)]
mod test;


use std::ops::Not;
use std::path::Path;
use std::fs::File;
use std::io::{Write, BufReader, BufRead};

#[derive(Debug, Copy, Clone)]
/// Structure to represent a pixel which is composed of three colors.
pub struct Pixel {
    /// Red
    r: u8,
    /// Green
    g: u8,
    /// Blue
    b: u8,
}

/// Structure to represent an image which is defined by its width, height and pixels.
pub struct Image {
    /// Image's width in i32
    pub width: i32,
    /// Image's height in i32
    pub height: i32,
    /// Image's all pixels, in Vec<Pixel> which is a 2D dynamic array
    pub pixels: Vec<Pixel>,
}



/// Implements functions for the Pixel structure
impl Pixel {
    
    /// Constructor, return a Pixel with the three colors in arguments. Each color is a u8
    pub fn create(r: u8, g: u8, b: u8) -> Pixel {
        Pixel { r, g, b }
    }

    /// ToString() of Pixel structure; Display each colors of the Pixel
    pub fn display(&self) {
        println!("red: {} green: {} blue: {}", self.r, self.g, self.b);
    }

    /// Revert each color of the Pixel
    /// # Arguments
    /// * self - Current Pixel, Pixel to be reverted
    /// 
    /// # Example 
    /// If current Pixel's color are (155, 145, 135) 
    /// current's Pixel's color will become (100, 110, 120)
    pub fn revert(&mut self) {
        self.r = 255 - self.r;
        self.g = 255 - self.g;
        self.b = 255 - self.b;
    }

    /// Transform current Pixel into a grayscale Pixel
    /// Each color will take the same value of grayscale (between 0 and 255)
    /// # Arguments
    /// * self - Current Pixel, Pixel to be converted
    /// 
    /// # Example 
    /// If current Pixel's color are (179, 120, 141) 
    /// current's Pixel's color will become (140, 140, 140)
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

    /// Return a reverted version of a Pixel
    /// # Arguments
    /// * self - Pixel to be converted
    /// 
    /// # Example 
    /// If Pixel's color are (155, 145, 135) 
    /// Pixel's color will become (100, 110, 120)
    fn not(self) -> Self::Output {
        return Pixel { r: 255 - self.r, g: 255 - self.g, b: 255 - self.b };
    }
}

impl PartialEq for Pixel {

    /// Equals to determine if the two Pixel in parameters are equals.
    /// Return true if self and other and equals else return false
    /// (the r, g and b of self are equals to the r, g and b of other)
    /// 
    /// # Arguments
    /// * self - a Color to be compared
    /// * other - a second Color to compare the first one
    /// 
    /// # Return
    /// * bool - corresponding to the equality (or not) of the two arguments
    fn eq(&self, other: &Self) -> bool {
        return self.r == other.r && self.g == other.g && self.b == other.b;
    }
}

impl Image {
    /// Constructor, return an Image
    pub fn create(width: i32, height: i32, pixels: Vec<Pixel>) -> Image {
        Image { width, height, pixels }
    }

    /// Create a new Image from .ppm File
    /// # Arguments
    /// * filename: &Path - The path corresponding to the file to be read.
    ///    
    /// # Return
    /// * Option<Image> - The Image created through the file read. Use Optional
    ///                   in case there is an error during file reading
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
            // If the line begin with a #, it's a commentary, we ignore this line
            if l.chars().next().unwrap() == '#' { continue; }
            count = count + 1;

            // Check the value of the file's first line
            // If it's not "P3", it's not a RGB picture => end of function
            if count == 0 {
                if &l != "P3" { return None; }
            // Check the value of file's second line
            // Get the dimensions of the picture
            } else if count == 1 {
                let size = get_size(l);
                img.width = size[0];
                img.height = size[1];
            // Check the value of file's remeaning line 
            // Parse and get (r,g,b) pixel for Image pixels array
            } else if count > 2 {
                let pixels: Vec<u8> = get_numbers_from_line(l);
                if pixels.len() % 3 != 0 { println!("len : {}", pixels.len()); return None; }
                for idx in (0..pixels.len()).step_by(3) {
                    img.pixels.push(Pixel::create(pixels[idx], pixels[idx + 1], pixels[idx + 2]));
                }
            }
        }
        return Some(img);
    }

    /// Write current Image in a file
    /// # Arguments
    /// * self: Current Image to be written in file
    /// * filename: Filename of file for Image to be written in
    pub fn save(&self, filename: &Path) {
        // Convert current Image into a String to be written in the file

        // Write "P3" followed by Image's dimensions
        let mut str: String = format!("P3\n{} {}\n255\n", self.width, self.height);

        // Convert each Pixel of pixels array in String
        for p in self.pixels.iter() {
            str = str + format!("{} {} {}\n", p.r, p.g, p.b).as_ref();
        }

        // Create file with filename in arguments
        let mut f = File::create(filename).expect("error");

        //Write the String in the created File
        f.write_all(str.as_ref());
    }


    /// Convert current RGB Image to a grayscale Image.
    /// 
    /// # Arguments
    /// * image:self - Current Image, Image to be converted
    pub fn grayscale(&mut self) {
        for  p in self.pixels.iter_mut() {
            p.grayscale();
        }
    }

    /// Convert current RGB Image to the reverted version of current Image.
    /// 
    /// # Arguments
    /// * image:self - Current Image, Image to be converted
    pub fn revert(&mut self) {
        for p in self.pixels.iter_mut() {
            p.revert();
        }
    }

    /// Equals to determine if the two Image in parameters are equals.
    /// Return true if self and other and equals else return false
    /// (Same dimensions and pixels array in each Image)
    /// 
    /// # Arguments
    /// * self - current Image to be compared
    /// * Image - a second Image to compare the first one to
    /// 
    /// # Return
    /// * bool - corresponding to the equality (or not) of the two arguments
    pub fn equal(&mut self, i: &mut Image) -> bool {
        if self.width != i.width || self.height != i.height {
            return false;
        }

        for it in self.pixels.iter().zip(i.pixels.iter_mut()) {
            let (self_i, other_i) = it;
            if self_i != other_i {
                return false;
            }
        }

        return true;
    }
}

/// Transform a String with numbers in it into 
/// a Vector<u32> for the picture pixels.
/// # Example :
/// * "100 124 45" passed as parameters will return Vec{100, 124, 45}. 
pub fn get_numbers_from_line(line: String) -> Vec<u8> {
    line.split('#').collect::<Vec<&str>>()[0].split_ascii_whitespace()
        .map(|col| col.parse::<u8>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_else(|err| panic!("Cannot parse a column: {}", err))
}


/// Split whitespace in line in arguments 
/// 
/// # Arguments
/// * line - Line to be splitted
/// 
/// # Return
/// * Vec<i32> - Array of each string in line
/// 
/// # Exemple
/// * "2 3" passed as parameters will return Vec{}
pub fn get_size(line: String) -> Vec<i32> {
    line.split("#").collect::<Vec<&str>>()[0].split_ascii_whitespace()
        .map(|col| col.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_else(|err| panic!("Cannot parse a column: {}", err))
}