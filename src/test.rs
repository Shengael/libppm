extern crate test;

use super::*;

use super::*;
use test::Bencher;

#[test]
fn test_revert() {
    let mut white: Vec<Pixel> = Vec::new();
    let mut black: Vec<Pixel> = Vec::new();

    for i in 0..4 {
        white.push(Pixel::create(255, 255, 255));
    }

    for i in 0..4 {
        black.push(Pixel::create(0, 0, 0));
    }

    let mut i_white: Image = Image::create(4, 4, white);
    let mut i_black: Image = Image::create(4, 4, black);

    i_black.revert();

    assert_eq!(true, i_white.equal(&mut i_black));
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
