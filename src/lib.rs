use std::ops::Not;

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
    b: u8
}

impl Pixel {

    pub fn create(r: u8, g: u8, b: u8) -> Pixel {
        Pixel {r, g, b}
    }

    pub fn display(&self) {
        println!("red: {} green: {} blue: {}", self.r, self.g, self.b);
    }

    pub fn revert(&mut self)  {
        self.r = 255 - self.r;
        self.g = 255 - self.g;
        self.b = 255 - self.b;
    }

    pub fn grayscale(&mut self) {
        gray = self.r as f32 * 0.3 + self.g as f32 * 0.59 + self.b as f32 * 0.11;
        self.r = g;
        self.g = g;
        self.b = g;
    }

//    pub fn eq(&mut self, p: Pixel) -> bool {
//        self.r == p.r && self.g == p.g && self.b == p.b
//    }
}

impl PartialEq for Pixel {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }
}

impl Not for Pixel {
    type Output = Pixel;

    fn not(self) -> Self::Output {
        return Pixel {r: 255 - self.r, g: 255 - self.g, b: 255 - self.b};
    }
}