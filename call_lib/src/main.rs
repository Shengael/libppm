extern crate ppm;

use std::path::Path;

fn main() {
    let p1 = ppm::Pixel::create(30, 128, 255);
    let p2 = ppm::Pixel::create(100, 24, 42);

    let mut p3 = ppm::Pixel::create(110, 90, 100);

    println!("Create : ");
    println!("p1 : ");
    p1.display();
    println!("p2 : ");
    p2.display();

    println!("________________");

    println!("Reverted : ");
    p3.display();
    println!("Reverted p3");
    p3 = !p3;
    p3.display();

    println!("________________");

    println!("Pixel equal : ");

    if ppm::Pixel::eq(&p1, &p2) {
        println!("p1 == p2");
    } else {
        println!("p1 != p2");
    }

    println!("________________");

    let p4 = ppm::Pixel::create(100, 24, 42);

    p4.display();
    if ppm::Pixel::eq(&p4, &p2) {
        println!("p4 == p2");
    } else {
        println!("p4 != p2");
    }

    println!("________________");

    println!("Grayscale : ");
    p3.display();
    p3.grayscale();
    println!("p3 en grayscale : " );
    p3.display();

    println!("________________");

    let img1 = ppm::Image::new_with_file(Path::new("src/test.ppm"));

    match img1 {
        Some(mut image) => {
            image.save(Path::new("src/test_save.ppm"));
        },
        None => println!("No Image found !")
    }

    let img2 = ppm::Image::new_with_file(Path::new("src/test.ppm"));

    match img2 {
        Some(mut image) => {
            image.grayscale();
            image.save(Path::new("src/test_grayscale.ppm"));
        },
        None => println!("No Image found !")
    }

    let img3 = ppm::Image::new_with_file(Path::new("src/test.ppm"));

    match img3 {
        Some(mut image) => {
            image.revert();
            image.save(Path::new("src/test_revert.ppm"));
        },
        None => println!("No Image found !")
    }
}
