extern crate test;

use super::*;

use super::*;
use test::Bencher;


#[test]
fn pixels_equals() {
    let c1 = Pixel::create(130, 18, 42);
    let c2 = Pixel::create(130, 18, 42);

    assert_eq!(true, Pixel::eq(&c1, &c2));
}

#[test]
fn pixels_not_equals() {
    let c1 = Pixel::create(30, 128, 255);
    let c2 = Pixel::create(0, 0, 0);

    assert_ne!(true, Pixel::eq(&c1, &c2));
}

#[test]
fn revert_pixel() {
    let mut c1: Vec<Pixel> = Vec::new();
    let mut c2: Vec<Pixel> = Vec::new();

    for i in 0..4 {
        c1.push(Pixel::create(155, 145, 135));
    }

    for i in 0..4 {
        c2.push(Pixel::create(100, 110, 120));
    }

    let mut i_c1: Image = Image::create(4, 4, c1);
    let mut i_c2: Image = Image::create(4, 4, c2);

    i_c2.revert();

    assert_eq!(true, i_c1.equal(&mut i_c2));
}

#[test]
fn get_numbers_from_line_vector() {
    let s: String = String::from("45 12 41");
    let mut vec: Vec<u8> = Vec::new();
    vec.push(45);
    vec.push(12);
    vec.push(41);

    assert_eq!(vec, get_numbers_from_line(s));
}


#[test]
fn get_size_in_vector() {
    let s: String = String::from("2 3");
    let mut vec: Vec<i32> = Vec::new();
    vec.push(2);
    vec.push(3);

    assert_eq!(vec, get_size(s));
}


#[test]
fn grayscale_pixel() {
  let mut p: Pixel = Pixel::create(255, 255, 0);
        
  p.grayscale();

  assert_eq!(226, p.r);
}


#[test]
fn two_images_are_equals() {
    let mut c1: Vec<Pixel> = Vec::new();
    let mut c2: Vec<Pixel> = Vec::new();

    for i in 0..4 {
        c1.push(Pixel::create(155, 145, 135));
    }

    for i in 0..4 {
        c2.push(Pixel::create(155, 145, 135));
    }

    let mut i_c1: Image = Image::create(4, 4, c1);
    let mut i_c2: Image = Image::create(4, 4, c2);

    assert_eq!(true, i_c1.equal(&mut i_c2));
}

#[bench]
fn bench_create_pixel(b: &mut Bencher) {
    b.iter(|| Pixel::create(155, 155, 155));
}

#[bench]
fn bench_revert(b: &mut Bencher) {
    let mut i = Image::create(2, 4, vec![
        Pixel::create(255, 12, 96), Pixel::create(255, 255, 0), Pixel::create(0, 0, 255), Pixel::create(0, 255, 255),
        Pixel::create(0, 255, 255), Pixel::create(0, 0, 255), Pixel::create(255, 12, 96), Pixel::create(255, 255, 0)
    ]);
    b.iter(|| i.revert());
}

#[bench]
fn bench_grayscale(b: &mut Bencher) {
    let mut i = Image::create(2, 4, vec![
        Pixel::create(255, 12, 96), Pixel::create(255, 255, 0), Pixel::create(0, 0, 255), Pixel::create(0, 255, 255),
        Pixel::create(0, 255, 255), Pixel::create(0, 0, 255), Pixel::create(255, 12, 96), Pixel::create(255, 255, 0)
    ]);
    b.iter(|| i.grayscale());
}


#[bench]
fn bench_pixels_equal(b: &mut Bencher) {
    let p1 = Pixel::create(130, 128, 12);
    let p2 = Pixel::create(130, 128, 12);

    b.iter(|| Pixel::eq(&p1,&p2));
}